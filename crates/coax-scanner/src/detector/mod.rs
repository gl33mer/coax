//! Detector Model
//!
//! This module provides the Detector abstraction for grouping related patterns
//! by provider and optionally verifying findings.
//!
//! See DETECTOR-MODEL-DESIGN.md for full design specification.

use crate::pattern_cache::{CompiledPattern, PatternConfig};
use crate::result::{ScanResult, VerificationStatus};
use async_trait::async_trait;
use std::collections::HashMap;
use std::fmt::Debug;

/// Verification result for a credential
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifyResult {
    pub status: VerificationStatus,
    pub message: Option<String>,
}

impl VerifyResult {
    pub fn unverified() -> Self {
        Self {
            status: VerificationStatus::Unverified,
            message: None,
        }
    }

    pub fn verified() -> Self {
        Self {
            status: VerificationStatus::Verified,
            message: Some("Credential verified against live service".to_string()),
        }
    }

    pub fn invalid() -> Self {
        Self {
            status: VerificationStatus::Invalid,
            message: Some("Credential is invalid or revoked".to_string()),
        }
    }

    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            status: VerificationStatus::Error(msg.into()),
            message: None,
        }
    }
}

/// Detector trait for secret detection and optional verification
///
/// Each detector represents a provider or category (AWS, GitHub, Stripe, etc.)
/// and owns its patterns. Verification-capable detectors can verify findings
/// against live services.
#[async_trait]
pub trait Detector: Send + Sync {
    /// Unique identifier for this detector
    fn id(&self) -> &str;

    /// Human-readable name
    fn name(&self) -> &str;

    /// Get all patterns for this detector
    fn patterns(&self) -> &[PatternConfig];

    /// Get compiled patterns for efficient matching
    fn compiled_patterns(&self) -> &[CompiledPattern];

    /// Whether this detector is enabled
    fn is_enabled(&self) -> bool;

    /// Whether verification is enabled for this detector
    fn is_verification_enabled(&self) -> bool;

    /// Verify a detected credential (optional, default: Unverified)
    ///
    /// The secret parameter is the actual detected credential text.
    /// Implementations should:
    /// - Use fail-open principle: errors return Error status, findings still emitted
    /// - Respect rate limits with exponential backoff
    /// - Use hard timeouts (default 5 seconds)
    async fn verify(&self, _secret: &str) -> VerifyResult {
        VerifyResult::unverified()
    }

    /// Post-process a finding after pattern match
    /// Default implementation adds detector_id to the finding
    fn process_finding(&self, mut finding: ScanResult) -> ScanResult {
        // Add detector metadata if not already present
        if finding.description.is_none() {
            // Could add detector-specific description here
        }
        finding
    }
}

/// Pattern-only detector (no verification)
///
/// This is a convenience struct for detectors that only do pattern matching
/// without live verification. It wraps a set of PatternConfig and provides
/// trivial Detector implementation.
pub struct PatternDetector {
    id: String,
    name: String,
    patterns: Vec<PatternConfig>,
    compiled: Vec<CompiledPattern>,
    enabled: bool,
}

impl PatternDetector {
    /// Create a new pattern-only detector
    pub fn new(id: impl Into<String>, name: impl Into<String>, patterns: Vec<PatternConfig>) -> Self {
        let compiled: Vec<CompiledPattern> = patterns
            .iter()
            .filter_map(|p| CompiledPattern::try_from_config(p).ok())
            .collect();

        Self {
            id: id.into(),
            name: name.into(),
            patterns,
            compiled,
            enabled: true,
        }
    }

    /// Set whether this detector is enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

#[async_trait]
impl Detector for PatternDetector {
    fn id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn patterns(&self) -> &[PatternConfig] {
        &self.patterns
    }

    fn compiled_patterns(&self) -> &[CompiledPattern] {
        &self.compiled
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn is_verification_enabled(&self) -> bool {
        false
    }
}

/// Registry of all detectors
///
/// Central registry that holds all detector instances. The scanner queries
/// the registry for patterns and looks up which detector owns a matched
/// pattern for verification dispatch.
pub struct DetectorRegistry {
    detectors: HashMap<String, Box<dyn Detector>>,
    all_patterns: Vec<CompiledPattern>,
    pattern_to_detector: HashMap<String, String>, // pattern_name -> detector_id
}

impl DetectorRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            detectors: HashMap::new(),
            all_patterns: Vec::new(),
            pattern_to_detector: HashMap::new(),
        }
    }

    /// Register a detector
    pub fn register<D: Detector + 'static>(&mut self, detector: D) {
        let id = detector.id().to_string();
        let name = detector.name().to_string();

        // Register patterns
        for pattern in detector.compiled_patterns() {
            self.pattern_to_detector
                .insert(pattern.name.to_string(), id.clone());
            self.all_patterns.push(pattern.clone());
        }

        self.detectors.insert(id.clone(), Box::new(detector));

        tracing::debug!("Registered detector: {} ({})", name, id);
    }

    /// Get all compiled patterns from all detectors
    pub fn all_patterns(&self) -> &[CompiledPattern] {
        &self.all_patterns
    }

    /// Get the detector ID for a pattern name
    pub fn get_detector_for_pattern(&self, pattern_name: &str) -> Option<&str> {
        self.pattern_to_detector.get(pattern_name).map(|s| s.as_str())
    }

    /// Get a detector by ID
    pub fn get_detector(&self, id: &str) -> Option<&dyn Detector> {
        self.detectors.get(id).map(|b| b.as_ref())
    }

    /// Get all enabled detectors
    pub fn enabled_detectors(&self) -> impl Iterator<Item = &dyn Detector> {
        self.detectors
            .values()
            .filter(|d| d.is_enabled())
            .map(|b| b.as_ref())
    }

    /// Get count of registered detectors
    pub fn detector_count(&self) -> usize {
        self.detectors.len()
    }

    /// Get count of all patterns
    pub fn pattern_count(&self) -> usize {
        self.all_patterns.len()
    }

    /// Enable/disable a detector by ID
    pub fn set_detector_enabled(&mut self, id: &str, enabled: bool) {
        if let Some(detector) = self.detectors.get_mut(id) {
            // Note: This is a limitation - we'd need mutable access to the trait object
            // For now, this is a placeholder for future implementation
            tracing::warn!("set_detector_enabled not fully implemented for trait objects");
        }
    }
}

impl Default for DetectorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_detector_creation() {
        let patterns = vec![PatternConfig::new(
            "TEST_PATTERN",
            r"test\d+",
            "high",
            "Test recommendation",
        )];

        let detector = PatternDetector::new("test", "Test Detector", patterns.clone());

        assert_eq!(detector.id(), "test");
        assert_eq!(detector.name(), "Test Detector");
        assert_eq!(detector.patterns().len(), 1);
        assert!(detector.is_enabled());
        assert!(!detector.is_verification_enabled());
    }

    #[test]
    fn test_detector_registry() {
        let mut registry = DetectorRegistry::new();

        let patterns = vec![PatternConfig::new(
            "AWS_ACCESS_KEY",
            r"AKIA[0-9A-Z]{16}",
            "critical",
            "Rotate immediately",
        )];

        let detector = PatternDetector::new("aws", "AWS Detector", patterns);
        registry.register(detector);

        assert_eq!(registry.detector_count(), 1);
        assert_eq!(registry.pattern_count(), 1);
        assert_eq!(
            registry.get_detector_for_pattern("AWS_ACCESS_KEY"),
            Some("aws")
        );
    }
}
