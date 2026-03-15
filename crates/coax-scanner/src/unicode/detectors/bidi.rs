//! Bidirectional Override Detector
//!
//! Detects bidirectional text overrides that can reverse displayed text.
//! Critical for: Comments, strings, documentation
//!
//! Unicode Characters:
//! - U+202A: LRE (Left-to-Right Embedding)
//! - U+202B: RLE (Right-to-Left Embedding)
//! - U+202C: PDF (Pop Directional Formatting)
//! - U+202D: LRO (Left-to-Right Override)
//! - U+202E: RLO (Right-to-Left Override) - MOST DANGEROUS
//! - U+2066-U+2069: Isolation characters

use crate::unicode::config::UnicodeConfig;
use crate::unicode::findings::{UnicodeFinding, UnicodeCategory, Severity};
use crate::unicode::ranges::get_bidi_name;

/// Detector for bidirectional override attacks
pub struct BidiDetector {
    config: UnicodeConfig,
}

impl BidiDetector {
    /// Create a new bidi detector
    pub fn new(config: UnicodeConfig) -> Self {
        Self { config }
    }

    /// Create with default config
    pub fn with_default_config() -> Self {
        Self::new(UnicodeConfig::default())
    }

    /// Scan content for bidirectional override attacks
    pub fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col_num, ch) in line.chars().enumerate() {
                let code_point = ch as u32;
                
                // Check if this is a bidi character
                if let Some(bidi_name) = get_bidi_name(code_point) {
                    let severity = self.determine_severity(code_point, bidi_name);

                    let finding = UnicodeFinding::new(
                        file_path,
                        line_num + 1,
                        col_num + 1,
                        code_point,
                        ch,
                        UnicodeCategory::BidirectionalOverride,
                        severity,
                        &self.get_description(code_point, bidi_name),
                        &self.get_remediation(code_point, bidi_name),
                    )
                    .with_cwe_id("CWE-172")
                    .with_reference("https://unicode.org/reports/tr36/")
                    .with_context(&self.get_context(line, col_num));

                    findings.push(finding);
                }
            }
        }

        findings
    }

    /// Determine severity based on bidi character type
    fn determine_severity(&self, code_point: u32, bidi_name: &str) -> Severity {
        match bidi_name {
            // RLO is the most dangerous - completely reverses text
            "RLO" => Severity::Critical,
            // RLE and LRO are also very dangerous
            "RLE" | "LRO" => Severity::High,
            // LRE is less dangerous but still suspicious
            "LRE" => Severity::Medium,
            // PDF and isolation characters are medium severity
            "PDF" | "LRI" | "RLI" | "FSI" | "PDI" => Severity::Medium,
            // Marks are lower severity
            "LRM" | "RLM" | "ALM" => Severity::Low,
            _ => Severity::Medium,
        }
    }

    /// Get human-readable description
    fn get_description(&self, code_point: u32, bidi_name: &str) -> String {
        let danger_level = match bidi_name {
            "RLO" => " [MOST DANGEROUS - reverses text display]",
            "RLE" | "LRO" => " [HIGH RISK - can hide malicious content]",
            _ => "",
        };

        format!(
            "Bidirectional control character detected: {} (U+{:04X}){}",
            bidi_name, code_point, danger_level
        )
    }

    /// Get remediation guidance
    fn get_remediation(&self, code_point: u32, bidi_name: &str) -> String {
        match bidi_name {
            "RLO" => "IMMEDIATE ACTION: Remove this RLO character immediately. It reverses \
                     text display and is commonly used to hide malicious content. For example, \
                     'exe.txt' with RLO becomes 'txt.exe' when displayed. Review the actual \
                     byte sequence of the file using a hex editor."
                .to_string(),
            "RLE" | "LRO" => "Remove this bidirectional override character. These can be used \
                             to hide malicious content by reversing text display. Review the \
                             context to understand the true content."
                .to_string(),
            _ => "Remove the bidirectional control character unless there's a legitimate \
                  reason for it (e.g., proper RTL language support). Review the context \
                  to ensure it's not being used to hide content."
                .to_string(),
        }
    }

    /// Get context around the character position
    fn get_context(&self, line: &str, char_pos: usize) -> String {
        let start = char_pos.saturating_sub(20);
        let end = (char_pos + 20).min(line.len());
        
        let prefix = if start > 0 { "..." } else { "" };
        let suffix = if end < line.len() { "..." } else { "" };
        
        format!("{}{}{}", prefix, &line[start..end], suffix)
    }
}

/// Trait implementation for UnicodeDetector
pub trait UnicodeDetector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding>;
    fn is_enabled(&self, config: &UnicodeConfig) -> bool;
}

impl UnicodeDetector for BidiDetector {
    fn name(&self) -> &'static str {
        "bidi"
    }

    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.detect(content, file_path)
    }

    fn is_enabled(&self, config: &UnicodeConfig) -> bool {
        config.detectors.bidirectional
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rlo_detection() {
        let detector = BidiDetector::with_default_config();
        
        // RLO - most dangerous
        let content = "const file = \"test\u{202E}exe\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x202E);
        assert_eq!(findings[0].severity, Severity::Critical);
    }

    #[test]
    fn test_rle_detection() {
        let detector = BidiDetector::with_default_config();
        
        // RLE
        let content = "const text = \"hello\u{202B}world\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x202B);
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn test_lro_detection() {
        let detector = BidiDetector::with_default_config();
        
        // LRO
        let content = "const text = \"hello\u{202D}world\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x202D);
        assert_eq!(findings[0].severity, Severity::High);
    }

    #[test]
    fn test_pdf_detection() {
        let detector = BidiDetector::with_default_config();
        
        // PDF (Pop Directional Formatting)
        let content = "const text = \"hello\u{202C}world\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x202C);
    }

    #[test]
    fn test_isolation_characters() {
        let detector = BidiDetector::with_default_config();
        
        // RLI (Right-to-Left Isolate)
        let content = "const text = \"hello\u{2067}world\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x2067);
    }

    #[test]
    fn test_clean_content() {
        let detector = BidiDetector::with_default_config();
        
        let content = "const normal = 'hello world';";
        let findings = detector.detect(content, "test.js");
        
        assert!(findings.is_empty());
    }

    #[test]
    fn test_multiple_bidi_chars() {
        let detector = BidiDetector::with_default_config();
        
        let content = "const x = \"\u{202E}exe\u{202C}\";"; // RLO + PDF
        let findings = detector.detect(content, "test.js");
        
        assert_eq!(findings.len(), 2);
    }
}
