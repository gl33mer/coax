//! Unicode Tag Detector
//!
//! Detects Unicode tag characters that can be used for metadata injection.
//!
//! Unicode Range:
//! - U+E0000-U+E007F: Tags (language tags, etc.)

use crate::unicode::config::UnicodeConfig;
use crate::unicode::findings::{UnicodeFinding, UnicodeCategory, Severity};

/// Detector for Unicode tag attacks
pub struct UnicodeTagDetector {
    config: UnicodeConfig,
}

impl UnicodeTagDetector {
    /// Create a new Unicode tag detector
    pub fn new(config: UnicodeConfig) -> Self {
        Self { config }
    }

    /// Create with default config
    pub fn with_default_config() -> Self {
        Self::new(UnicodeConfig::default())
    }

    /// Scan content for Unicode tag attacks
    pub fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col_num, ch) in line.chars().enumerate() {
                let code_point = ch as u32;
                
                // Check if this is in the tags range (U+E0000-U+E007F)
                if code_point >= 0xE0000 && code_point <= 0xE007F {
                    let severity = Severity::Medium;

                    let tag_name = self.get_tag_name(code_point);

                    let finding = UnicodeFinding::new(
                        file_path,
                        line_num + 1,
                        col_num + 1,
                        code_point,
                        ch,
                        UnicodeCategory::UnicodeTag,
                        severity,
                        format!("Unicode tag character detected: {} (U+{:04X})", tag_name, code_point).as_str(),
                        "Remove the Unicode tag character. These are rarely used in legitimate \
                         code and can be used to inject hidden metadata or bypass security checks. \
                         If this appears in a string literal, it may be an attempt to hide data.",
                    )
                    .with_cwe_id("CWE-172")
                    .with_context(&self.get_context(line, col_num));

                    findings.push(finding);
                }
            }
        }

        findings
    }

    /// Get human-readable name for a tag character
    fn get_tag_name(&self, code_point: u32) -> String {
        match code_point {
            0xE0001 => "Language Tag".to_string(),
            0xE007F => "Cancel Tag".to_string(),
            0xE0020..=0xE007E => {
                // Tag characters that represent ASCII
                let ascii = (code_point - 0xE0000) as u8;
                if ascii >= 0x20 && ascii <= 0x7E {
                    format!("Tag: {}", ascii as char)
                } else {
                    format!("Tag (U+{:04X})", code_point)
                }
            }
            _ => format!("Tag (U+{:04X})", code_point),
        }
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

impl UnicodeDetector for UnicodeTagDetector {
    fn name(&self) -> &'static str {
        "unicode_tag"
    }

    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.detect(content, file_path)
    }

    fn is_enabled(&self, config: &UnicodeConfig) -> bool {
        config.detectors.unicode_tags
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_tag_detection() {
        let detector = UnicodeTagDetector::with_default_config();
        
        // Language tag character
        let content = "const text = \"hello\u{E0001}world\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].category, UnicodeCategory::UnicodeTag);
        assert_eq!(findings[0].code_point, 0xE0001);
    }

    #[test]
    fn test_cancel_tag_detection() {
        let detector = UnicodeTagDetector::with_default_config();
        
        // Cancel tag
        let content = "const text = \"hello\u{E007F}world\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0xE007F);
    }

    #[test]
    fn test_tag_ascii_detection() {
        let detector = UnicodeTagDetector::with_default_config();
        
        // Tag character representing 'A'
        let content = "const text = \"hello\u{E0041}world\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert!(findings[0].description.contains("Tag: A"));
    }

    #[test]
    fn test_clean_content() {
        let detector = UnicodeTagDetector::with_default_config();
        
        let content = "const normal = 'hello world';";
        let findings = detector.detect(content, "test.js");
        
        assert!(findings.is_empty());
    }
}
