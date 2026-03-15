//! Unicode Scanner - Main Entry Point
//!
//! Primary entry point for Unicode attack detection.
//! 
//! Architecture Notes:
//! - Thread-safe for parallel scanning
//! - Configurable detectors
//! - Support for hot-reloading of configurations
//! - Includes scan statistics

use crate::unicode::config::UnicodeConfig;
use crate::unicode::findings::{UnicodeFinding, UnicodeCategory, Severity, UnicodeScanStats};
use crate::unicode::detectors::{
    InvisibleCharDetector,
    HomoglyphDetector,
    BidiDetector,
    GlasswormDetector,
    UnicodeTagDetector,
};
use std::sync::Arc;

/// Primary entry point for Unicode attack detection
pub struct UnicodeScanner {
    config: UnicodeConfig,
    invisible_detector: InvisibleCharDetector,
    homoglyph_detector: HomoglyphDetector,
    bidi_detector: BidiDetector,
    glassworm_detector: GlasswormDetector,
    tag_detector: UnicodeTagDetector,
    stats: UnicodeScanStats,
}

impl UnicodeScanner {
    /// Create a new Unicode scanner with the given configuration
    pub fn new(config: UnicodeConfig) -> Self {
        Self {
            invisible_detector: InvisibleCharDetector::new(config.clone()),
            homoglyph_detector: HomoglyphDetector::new(config.clone()),
            bidi_detector: BidiDetector::new(config.clone()),
            glassworm_detector: GlasswormDetector::new(config.clone()),
            tag_detector: UnicodeTagDetector::new(config.clone()),
            stats: UnicodeScanStats::new(),
            config,
        }
    }

    /// Create with default configuration
    pub fn with_default_config() -> Self {
        Self::new(UnicodeConfig::default())
    }

    /// Create for i18n projects (more permissive)
    pub fn for_i18n_project() -> Self {
        Self::new(UnicodeConfig::for_i18n_project())
    }

    /// Create for high-security projects (stricter)
    pub fn for_high_security() -> Self {
        Self::new(UnicodeConfig::for_high_security())
    }

    /// Scan content for all Unicode attacks
    pub fn scan(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        let mut all_findings = Vec::new();

        // Run all enabled detectors
        if self.config.detectors.invisible_chars {
            let findings = self.invisible_detector.detect(content, file_path);
            all_findings.extend(findings);
        }

        if self.config.detectors.homoglyphs {
            let findings = self.homoglyph_detector.detect(content, file_path);
            all_findings.extend(findings);
        }

        if self.config.detectors.bidirectional {
            let findings = self.bidi_detector.detect(content, file_path);
            all_findings.extend(findings);
        }

        if self.config.detectors.glassworm {
            let findings = self.glassworm_detector.detect(content, file_path);
            all_findings.extend(findings);
        }

        if self.config.detectors.unicode_tags {
            let findings = self.tag_detector.detect(content, file_path);
            all_findings.extend(findings);
        }

        // Sort findings by severity (critical first) and then by location
        all_findings.sort_by(|a, b| {
            b.severity.cmp(&a.severity)
                .then_with(|| a.line.cmp(&b.line))
                .then_with(|| a.column.cmp(&b.column))
        });

        all_findings
    }

    /// Scan only for invisible characters
    pub fn scan_invisible(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.invisible_detector.detect(content, file_path)
    }

    /// Scan only for homoglyph attacks
    pub fn scan_homoglyphs(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.homoglyph_detector.detect(content, file_path)
    }

    /// Scan only for bidirectional overrides
    pub fn scan_bidi(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.bidi_detector.detect(content, file_path)
    }

    /// Scan only for Glassworm patterns
    pub fn scan_glassworm(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.glassworm_detector.detect(content, file_path)
    }

    /// Scan only for Unicode tags
    pub fn scan_tags(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.tag_detector.detect(content, file_path)
    }

    /// Get scan statistics
    pub fn get_stats(&self) -> &UnicodeScanStats {
        &self.stats
    }

    /// Get the configuration
    pub fn get_config(&self) -> &UnicodeConfig {
        &self.config
    }

    /// List all available detectors
    pub fn list_detectors() -> Vec<&'static str> {
        vec![
            "invisible_char",
            "homoglyph",
            "bidi",
            "glassworm",
            "unicode_tag",
        ]
    }

    /// Check if content contains any invisible characters (quick check)
    pub fn has_invisible_chars(content: &str) -> bool {
        content.chars().any(|ch| {
            let cp = ch as u32;
            // Quick check for common invisible ranges
            (cp >= 0xFE00 && cp <= 0xFE0F) ||  // Variation selectors
            (cp >= 0x200B && cp <= 0x200F) ||  // Zero-width
            (cp >= 0x202A && cp <= 0x202E) ||  // Bidi
            (cp >= 0xE0000 && cp <= 0xE007F)   // Tags
        })
    }

    /// Check if content contains any confusable characters (quick check)
    pub fn has_confusables(content: &str) -> bool {
        use crate::unicode::confusables::data::is_confusable;
        content.chars().any(|ch| is_confusable(ch))
    }

    /// Deduplicate findings (same file, line, column, code_point)
    pub fn deduplicate_findings(findings: Vec<UnicodeFinding>) -> Vec<UnicodeFinding> {
        use std::collections::HashSet;
        
        let mut seen = HashSet::new();
        let mut deduped = Vec::new();

        for finding in findings {
            let key = (
                finding.file.clone(),
                finding.line,
                finding.column,
                finding.code_point,
            );

            if !seen.contains(&key) {
                seen.insert(key);
                deduped.push(finding);
            }
        }

        deduped
    }
}

/// Statistics for a Unicode scan session
#[derive(Debug, Clone, Default)]
pub struct ScanSessionStats {
    pub total_files: usize,
    pub total_findings: usize,
    pub critical_findings: usize,
    pub high_findings: usize,
    pub medium_findings: usize,
    pub low_findings: usize,
    pub scan_duration_ms: u64,
}

impl ScanSessionStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_findings(findings: &[UnicodeFinding], duration_ms: u64) -> Self {
        let mut stats = Self {
            total_files: findings.iter()
                .map(|f| &f.file)
                .collect::<std::collections::HashSet<_>>()
                .len(),
            total_findings: findings.len(),
            scan_duration_ms: duration_ms,
            ..Default::default()
        };

        for finding in findings {
            match finding.severity {
                Severity::Critical => stats.critical_findings += 1,
                Severity::High => stats.high_findings += 1,
                Severity::Medium => stats.medium_findings += 1,
                Severity::Low => stats.low_findings += 1,
            }
        }

        stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = UnicodeScanner::with_default_config();
        assert_eq!(UnicodeScanner::list_detectors().len(), 5);
    }

    #[test]
    fn test_full_scan_variation_selector() {
        let scanner = UnicodeScanner::with_default_config();
        
        let content = "const secret\u{FE00}Key = 'value';";
        let findings = scanner.scan(content, "test.js");
        
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.category == UnicodeCategory::InvisibleCharacter));
    }

    #[test]
    fn test_full_scan_homoglyph() {
        let scanner = UnicodeScanner::with_default_config();
        
        let content = "const pаssword = 'secret';"; // Cyrillic 'а'
        let findings = scanner.scan(content, "test.js");
        
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.category == UnicodeCategory::Homoglyph));
    }

    #[test]
    fn test_full_scan_bidi() {
        let scanner = UnicodeScanner::with_default_config();
        
        let content = "const file = \"test\u{202E}exe\";";
        let findings = scanner.scan(content, "test.js");
        
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.category == UnicodeCategory::BidirectionalOverride));
    }

    #[test]
    fn test_full_scan_glassworm() {
        let scanner = UnicodeScanner::with_default_config();
        
        let content = r#"
            const codes = "secret".split('').map(c => c.codePointAt(0));
            eval(String.fromCharCode(...codes));
        "#;
        let findings = scanner.scan(content, "test.js");
        
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.category == UnicodeCategory::GlasswormPattern));
    }

    #[test]
    fn test_clean_content() {
        let scanner = UnicodeScanner::with_default_config();
        
        let content = "const normal = 'hello world';";
        let findings = scanner.scan(content, "test.js");
        
        assert!(findings.is_empty());
    }

    #[test]
    fn test_has_invisible_chars() {
        assert!(UnicodeScanner::has_invisible_chars("hello\u{FE00}world"));
        assert!(UnicodeScanner::has_invisible_chars("test\u{200B}"));
        assert!(!UnicodeScanner::has_invisible_chars("normal text"));
    }

    #[test]
    fn test_has_confusables() {
        assert!(UnicodeScanner::has_confusables("pаssword")); // Cyrillic 'а'
        assert!(!UnicodeScanner::has_confusables("password")); // All ASCII
    }

    #[test]
    fn test_deduplication() {
        let finding1 = UnicodeFinding::new(
            "test.js", 1, 5, 0xFE00, '\u{FE00}',
            UnicodeCategory::InvisibleCharacter, Severity::Critical,
            "test", "fix"
        );
        let finding2 = finding1.clone(); // Duplicate
        let finding3 = UnicodeFinding::new(
            "test.js", 2, 10, 0xFE01, '\u{FE01}',
            UnicodeCategory::InvisibleCharacter, Severity::Critical,
            "test", "fix"
        );

        let findings = vec![finding1.clone(), finding2, finding3];
        let deduped = UnicodeScanner::deduplicate_findings(findings);

        assert_eq!(deduped.len(), 2);
    }

    #[test]
    fn test_i18n_config() {
        let scanner = UnicodeScanner::for_i18n_project();
        assert_eq!(scanner.config.sensitivity.as_str(), "medium");
    }

    #[test]
    fn test_high_security_config() {
        let scanner = UnicodeScanner::for_high_security();
        assert_eq!(scanner.config.sensitivity.as_str(), "critical");
        assert!(scanner.config.detectors.normalization);
    }
}
