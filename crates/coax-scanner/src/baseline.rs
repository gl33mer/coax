//! Baseline Module
//!
//! This module provides baseline file functionality for managing known findings.
//! Baselines allow you to:
//! - Generate a baseline of existing findings
//! - Compare new scans against the baseline
//! - Only report NEW findings (not in baseline)

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use crate::ScanResult;

/// Baseline file version
pub const BASELINE_VERSION: &str = "1.0";

/// Baseline file structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaselineFile {
    pub version: String,
    pub generated: String,
    pub findings: Vec<BaselineFinding>,
}

/// A single finding in the baseline
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BaselineFinding {
    pub hash: String,
    pub pattern: String,
    pub file: String,
    pub line: u32,
    #[serde(default = "default_status")]
    pub status: String,
}

fn default_status() -> String {
    "accepted".to_string()
}

impl BaselineFile {
    /// Create a new baseline file
    pub fn new() -> Self {
        Self {
            version: BASELINE_VERSION.to_string(),
            generated: Utc::now().to_rfc3339(),
            findings: Vec::new(),
        }
    }
    
    /// Create baseline from scan results
    pub fn from_results(results: &[ScanResult]) -> Self {
        let mut baseline = Self::new();
        
        for result in results {
            baseline.findings.push(BaselineFinding::from_result(result));
        }
        
        baseline
    }
    
    /// Load baseline from file
    pub fn load(path: &Path) -> Result<Self, BaselineError> {
        let content = fs::read_to_string(path)
            .map_err(|e| BaselineError::IoError {
                path: path.to_string_lossy().to_string(),
                source: e,
            })?;
        
        let baseline: BaselineFile = serde_json::from_str(&content)
            .map_err(|e| BaselineError::ParseError {
                path: path.to_string_lossy().to_string(),
                source: e,
            })?;
        
        Ok(baseline)
    }
    
    /// Save baseline to file
    pub fn save(&self, path: &Path) -> Result<(), BaselineError> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| BaselineError::IoError {
                    path: parent.to_string_lossy().to_string(),
                    source: e,
                })?;
        }
        
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| BaselineError::SerializeError {
                path: path.to_string_lossy().to_string(),
                source: e,
            })?;
        
        fs::write(path, content)
            .map_err(|e| BaselineError::IoError {
                path: path.to_string_lossy().to_string(),
                source: e,
            })?;
        
        Ok(())
    }
    
    /// Update baseline with new findings
    pub fn update(&mut self, new_results: &[ScanResult]) -> Vec<ScanResult> {
        let mut truly_new = Vec::new();
        let mut new_findings_to_add = Vec::new();
        let existing_hashes: std::collections::HashSet<_> = 
            self.findings.iter().map(|f| &f.hash).collect();
        
        for result in new_results {
            let hash = calculate_finding_hash(result);
            
            if !existing_hashes.contains(&hash) {
                // This is a new finding
                truly_new.push(result.clone());
                new_findings_to_add.push(BaselineFinding::from_result(result));
            }
        }
        
        // Add new findings after the borrow is done
        self.findings.extend(new_findings_to_add);
        
        // Update generation timestamp
        self.generated = Utc::now().to_rfc3339();
        
        truly_new
    }
    
    /// Filter results to only include new findings (not in baseline)
    pub fn filter_new_findings(&self, results: &[ScanResult]) -> Vec<ScanResult> {
        let existing_hashes: std::collections::HashSet<_> = 
            self.findings.iter().map(|f| &f.hash).collect();
        
        results.iter()
            .filter(|result| {
                let hash = calculate_finding_hash(result);
                !existing_hashes.contains(&hash)
            })
            .cloned()
            .collect()
    }
}

impl Default for BaselineFile {
    fn default() -> Self {
        Self::new()
    }
}

impl BaselineFinding {
    /// Create from scan result
    pub fn from_result(result: &ScanResult) -> Self {
        Self {
            hash: calculate_finding_hash(result),
            pattern: result.pattern.clone(),
            file: result.file.to_string_lossy().to_string(),
            line: result.line,
            status: "accepted".to_string(),
        }
    }
}

/// Calculate a unique hash for a finding
pub fn calculate_finding_hash(result: &ScanResult) -> String {
    let mut hasher = Sha256::new();
    
    // Hash the unique identifiers of a finding
    let finding_id = format!(
        "{}:{}:{}:{}",
        result.file.to_string_lossy(),
        result.pattern,
        result.line,
        result.column.unwrap_or(0)
    );
    
    hasher.update(finding_id.as_bytes());
    let hash = hasher.finalize();
    
    format!("sha256:{:x}", hash)
}

/// Compare scan results against baseline
pub fn compare_with_baseline(
    results: &[ScanResult],
    baseline: &BaselineFile,
) -> BaselineComparison {
    let existing_hashes: std::collections::HashSet<_> = 
        baseline.findings.iter().map(|f| &f.hash).collect();
    
    let mut new_findings = Vec::new();
    let mut existing_findings = Vec::new();
    
    for result in results {
        let hash = calculate_finding_hash(result);
        
        if existing_hashes.contains(&hash) {
            existing_findings.push(result.clone());
        } else {
            new_findings.push(result.clone());
        }
    }
    
    BaselineComparison {
        new_findings,
        existing_findings,
        total_baseline_findings: baseline.findings.len(),
    }
}

/// Result of baseline comparison
#[derive(Debug, Clone)]
pub struct BaselineComparison {
    pub new_findings: Vec<ScanResult>,
    pub existing_findings: Vec<ScanResult>,
    pub total_baseline_findings: usize,
}

impl BaselineComparison {
    /// Check if there are any new findings
    pub fn has_new_findings(&self) -> bool {
        !self.new_findings.is_empty()
    }
    
    /// Get count of new findings
    pub fn new_count(&self) -> usize {
        self.new_findings.len()
    }
}

/// Baseline errors
#[derive(Debug, thiserror::Error)]
pub enum BaselineError {
    #[error("IO error for {path}: {source}")]
    IoError {
        path: String,
        #[source]
        source: std::io::Error,
    },
    
    #[error("Failed to parse baseline file {path}: {source}")]
    ParseError {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    
    #[error("Failed to serialize baseline file {path}: {source}")]
    SerializeError {
        path: String,
        #[source]
        source: serde_json::Error,
    },
}

/// Generate baseline file path
pub fn default_baseline_path() -> PathBuf {
    PathBuf::from(".coax-baseline.json")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::FindingContext;
    use tempfile::TempDir;
    
    fn create_test_result() -> ScanResult {
        ScanResult {
            file: PathBuf::from("config.yml"),
            line: 45,
            column: Some(1),
            pattern: "AWS_ACCESS_KEY".to_string(),
            severity: "critical".to_string(),
            recommendation: "Remove immediately".to_string(),
            detected_secret: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
            line_content: Some("AWS_KEY=AKIAIOSFODNN7EXAMPLE".to_string()),
            context: FindingContext::default(),
        }
    }
    
    #[test]
    fn test_baseline_creation() {
        let result = create_test_result();
        let baseline = BaselineFile::from_results(&[result.clone()]);
        
        assert_eq!(baseline.version, BASELINE_VERSION);
        assert_eq!(baseline.findings.len(), 1);
        assert_eq!(baseline.findings[0].pattern, "AWS_ACCESS_KEY");
        assert_eq!(baseline.findings[0].file, "config.yml");
        assert_eq!(baseline.findings[0].line, 45);
    }
    
    #[test]
    fn test_finding_hash() {
        let result = create_test_result();
        let hash1 = calculate_finding_hash(&result);
        let hash2 = calculate_finding_hash(&result);
        
        // Same finding should produce same hash
        assert_eq!(hash1, hash2);
        
        // Hash should start with sha256:
        assert!(hash1.starts_with("sha256:"));
    }
    
    #[test]
    fn test_baseline_save_load() {
        let temp_dir = TempDir::new().unwrap();
        let baseline_path = temp_dir.path().join("test-baseline.json");
        
        let result = create_test_result();
        let mut baseline = BaselineFile::from_results(&[result]);
        
        // Save
        baseline.save(&baseline_path).unwrap();
        
        // Load
        let loaded = BaselineFile::load(&baseline_path).unwrap();
        
        assert_eq!(loaded.findings.len(), 1);
        assert_eq!(loaded.findings[0].pattern, "AWS_ACCESS_KEY");
    }
    
    #[test]
    fn test_filter_new_findings() {
        let result1 = create_test_result();
        let result2 = ScanResult {
            file: PathBuf::from("secret.txt"),
            line: 10,
            column: None,
            pattern: "GITHUB_PAT".to_string(),
            severity: "critical".to_string(),
            recommendation: "Remove".to_string(),
            detected_secret: None,
            line_content: None,
            context: FindingContext::default(),
        };
        
        // Create baseline with result1
        let baseline = BaselineFile::from_results(&[result1.clone()]);
        
        // Filter results - should only return result2 as new
        let new_findings = baseline.filter_new_findings(&[result1.clone(), result2.clone()]);
        
        assert_eq!(new_findings.len(), 1);
        assert_eq!(new_findings[0].pattern, "GITHUB_PAT");
    }
    
    #[test]
    fn test_baseline_update() {
        let result1 = create_test_result();
        let result2 = ScanResult {
            file: PathBuf::from("secret.txt"),
            line: 10,
            column: None,
            pattern: "GITHUB_PAT".to_string(),
            severity: "critical".to_string(),
            recommendation: "Remove".to_string(),
            detected_secret: None,
            line_content: None,
            context: FindingContext::default(),
        };
        
        let mut baseline = BaselineFile::from_results(&[result1.clone()]);
        let new_findings = baseline.update(&[result2.clone()]);
        
        assert_eq!(new_findings.len(), 1);
        assert_eq!(baseline.findings.len(), 2);
    }
    
    #[test]
    fn test_baseline_comparison() {
        let result1 = create_test_result();
        let result2 = ScanResult {
            file: PathBuf::from("secret.txt"),
            line: 10,
            column: None,
            pattern: "GITHUB_PAT".to_string(),
            severity: "critical".to_string(),
            recommendation: "Remove".to_string(),
            detected_secret: None,
            line_content: None,
            context: FindingContext::default(),
        };
        
        let baseline = BaselineFile::from_results(&[result1.clone()]);
        let comparison = compare_with_baseline(&[result1.clone(), result2.clone()], &baseline);
        
        assert_eq!(comparison.new_count(), 1);
        assert!(comparison.has_new_findings());
        assert_eq!(comparison.existing_findings.len(), 1);
    }
}
