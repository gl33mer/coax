//! Homoglyph Detector
//!
//! Detects confusable characters that could be used for spoofing attacks.
//! 
//! Data Source: Embedded confusables database
//! Performance: O(1) lookup per character using HashMap

use crate::unicode::config::UnicodeConfig;
use crate::unicode::findings::{UnicodeFinding, UnicodeCategory, Severity};
use crate::unicode::confusables::data::{
    get_base_char, is_confusable, get_confusable_script, get_similarity,
};
use std::collections::HashMap;

/// A confusable match result
#[derive(Debug, Clone)]
pub struct ConfusableMatch {
    pub suspicious_char: char,
    pub base_char: char,
    pub confidence: f32,
    pub script_source: String,
    pub visual_similarity: f32,
}

/// Detector for homoglyph attacks
pub struct HomoglyphDetector {
    min_confidence: f32,
    config: UnicodeConfig,
}

impl HomoglyphDetector {
    /// Create a new homoglyph detector
    pub fn new(config: UnicodeConfig) -> Self {
        Self {
            min_confidence: 0.8,
            config,
        }
    }

    /// Create with default config
    pub fn with_default_config() -> Self {
        Self::new(UnicodeConfig::default())
    }

    /// Set minimum confidence threshold
    pub fn with_min_confidence(mut self, threshold: f32) -> Self {
        self.min_confidence = threshold;
        self
    }

    /// Scan content for homoglyph attacks
    pub fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col_num, ch) in line.chars().enumerate() {
                if let Some(match_result) = self.is_confusable(ch) {
                    if match_result.confidence < self.min_confidence {
                        continue;
                    }

                    let code_point = ch as u32;
                    let severity = self.determine_severity(&match_result);

                    let finding = UnicodeFinding::new(
                        file_path,
                        line_num + 1,
                        col_num + 1,
                        code_point,
                        ch,
                        UnicodeCategory::Homoglyph,
                        severity,
                        &self.get_description(&match_result),
                        &self.get_remediation(&match_result),
                    )
                    .with_cwe_id("CWE-172")
                    .with_reference("https://docs.github.com/en/security")
                    .with_context(&self.get_context(line, col_num));

                    findings.push(finding);
                }
            }
        }

        findings
    }

    /// Check if a character is confusable
    pub fn is_confusable(&self, ch: char) -> Option<ConfusableMatch> {
        if !is_confusable(ch) {
            return None;
        }

        let base = get_base_char(ch)?;
        let script = get_confusable_script(ch).unwrap_or("Unknown");
        let similarity = get_similarity(ch).unwrap_or(0.5);

        Some(ConfusableMatch {
            suspicious_char: ch,
            base_char: base,
            confidence: similarity,
            script_source: script.to_string(),
            visual_similarity: similarity,
        })
    }

    /// Scan an identifier for homoglyph attacks
    pub fn scan_identifier(&self, identifier: &str) -> Vec<ConfusableMatch> {
        let mut matches = Vec::new();

        for ch in identifier.chars() {
            if let Some(m) = self.is_confusable(ch) {
                matches.push(m);
            }
        }

        matches
    }

    /// Determine severity based on match characteristics
    fn determine_severity(&self, match_result: &ConfusableMatch) -> Severity {
        // High similarity = higher severity
        if match_result.visual_similarity >= 0.99 {
            Severity::Critical
        } else if match_result.visual_similarity >= 0.95 {
            Severity::High
        } else if match_result.visual_similarity >= 0.9 {
            Severity::Medium
        } else {
            Severity::Low
        }
    }

    /// Get human-readable description
    fn get_description(&self, match_result: &ConfusableMatch) -> String {
        format!(
            "Homoglyph detected: '{}' (U+{:04X}) from {} script confusable with '{}' - {:.0}% similarity",
            match_result.suspicious_char,
            match_result.suspicious_char as u32,
            match_result.script_source,
            match_result.base_char,
            match_result.visual_similarity * 100.0
        )
    }

    /// Get remediation guidance
    fn get_remediation(&self, match_result: &ConfusableMatch) -> String {
        format!(
            "Replace the {} character '{}' with the ASCII character '{}'. \
             This appears to be a homoglyph attack where visually similar characters \
             are used to spoof identifiers. Review the context to determine if this \
             is intentional (e.g., i18n) or malicious.",
            match_result.script_source,
            match_result.suspicious_char,
            match_result.base_char
        )
    }

    /// Get context around the character position
    /// Get context around the character position (Unicode-safe)
    fn get_context(&self, line: &str, char_pos: usize) -> String {
        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();
        let start = char_pos.saturating_sub(20);
        let end = (char_pos + 20).min(len);

        let prefix = if start > 0 { "..." } else { "" };
        let suffix = if end < len { "..." } else { "" };

        let context: String = chars[start..end].iter().collect();
        format!("{}{}{}", prefix, context, suffix)
    }

}

/// Trait implementation for UnicodeDetector
pub trait UnicodeDetector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding>;
    fn is_enabled(&self, config: &UnicodeConfig) -> bool;
}

impl UnicodeDetector for HomoglyphDetector {
    fn name(&self) -> &'static str {
        "homoglyph"
    }

    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.detect(content, file_path)
    }

    fn is_enabled(&self, config: &UnicodeConfig) -> bool {
        config.detectors.homoglyphs
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cyrillic_a_detection() {
        let detector = HomoglyphDetector::with_default_config();
        
        // Cyrillic 'а' (U+0430) vs Latin 'a'
        let content = "const pаssword = 'secret';"; // Cyrillic 'а'
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].category, UnicodeCategory::Homoglyph);
        assert_eq!(findings[0].code_point, 0x0430);
    }

    #[test]
    fn test_greek_o_detection() {
        let detector = HomoglyphDetector::with_default_config();
        
        // Greek 'ο' (U+03BF) vs Latin 'o'
        let content = "const lοgin = 'user';"; // Greek 'ο'
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x03BF);
    }

    #[test]
    fn test_cyrillic_e_detection() {
        let detector = HomoglyphDetector::with_default_config();
        
        // Cyrillic 'е' (U+0435) vs Latin 'e'
        let content = "const usеr = 'test';"; // Cyrillic 'е'
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x0435);
    }

    #[test]
    fn test_uppercase_cyrillic_detection() {
        let detector = HomoglyphDetector::with_default_config();
        
        // Cyrillic 'А' (U+0410) vs Latin 'A'
        let content = "const АPI_KEY = 'secret';"; // Cyrillic 'А'
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x0410);
    }

    #[test]
    fn test_clean_content() {
        let detector = HomoglyphDetector::with_default_config();
        
        let content = "const password = 'secret';"; // All ASCII
        let findings = detector.detect(content, "test.js");
        
        assert!(findings.is_empty());
    }

    #[test]
    fn test_scan_identifier() {
        let detector = HomoglyphDetector::with_default_config();
        
        // Identifier with Cyrillic 'а'
        let matches = detector.scan_identifier("pаssword");
        
        assert!(!matches.is_empty());
        assert_eq!(matches[0].base_char, 'a');
        assert_eq!(matches[0].script_source, "Cyrillic");
    }

    #[test]
    fn test_confidence_threshold() {
        let detector = HomoglyphDetector::with_default_config()
            .with_min_confidence(0.95);
        
        // Cyrillic 'а' has 100% similarity
        let result = detector.is_confusable('а');
        assert!(result.is_some());
        assert!(result.unwrap().confidence >= 0.95);
    }

    #[test]
    fn test_multiple_homoglyphs() {
        let detector = HomoglyphDetector::with_default_config();
        
        // Multiple confusables in one line
        let content = "const sесrеt = 'value';"; // Cyrillic 'е' and 'с'
        let findings = detector.detect(content, "test.js");
        
        assert!(findings.len() >= 2);
    }
}
