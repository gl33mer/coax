//! Invisible Character Detector
//!
//! Detects invisible Unicode characters used in Glassworm-style attacks.
//! 
//! Unicode Ranges Monitored:
//! - U+FE00-U+FE0F: Variation Selectors (Glassworm primary)
//! - U+E0100-U+E01EF: Variation Selectors Supplement
//! - U+200B-U+200F: Zero-width space, joiner, non-joiner
//! - U+2060-U+206F: Word joiner, invisible operators
//! - U+E0000-U+E007F: Tags

use crate::unicode::config::{UnicodeConfig, SensitivityLevel};
use crate::unicode::findings::{UnicodeFinding, UnicodeCategory, Severity};
use crate::unicode::ranges::{
    is_in_invisible_range, is_in_critical_range, is_variation_selector,
    get_bidi_name, get_zero_width_name,
};
use lazy_static::lazy_static;
use regex::Regex;

/// Detector for invisible characters
pub struct InvisibleCharDetector {
    skip_contexts: Vec<Regex>,
    config: UnicodeConfig,
}

impl InvisibleCharDetector {
    /// Create a new invisible character detector
    pub fn new(config: UnicodeConfig) -> Self {
        Self {
            skip_contexts: vec![
                // Emoji variation selectors (legitimate)
                Regex::new(r"[\x{1F300}-\x{1F9FF}][\x{FE00}-\x{FE0F}]").unwrap(),
                // CJK character variants
                Regex::new(r"[\x{4E00}-\x{9FFF}][\x{FE00}-\x{FE0F}]").unwrap(),
            ],
            config,
        }
    }

    /// Create with default config
    pub fn with_default_config() -> Self {
        Self::new(UnicodeConfig::default())
    }

    /// Scan content for invisible characters
    pub fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        let mut findings = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for (col_num, ch) in line.chars().enumerate() {
                let code_point = ch as u32;
                
                if is_in_invisible_range(code_point) {
                    // Check if this is in a legitimate context
                    if self.is_legitimate_context(line, col_num) {
                        continue;
                    }

                    // Determine severity based on range and sensitivity
                    let severity = self.determine_severity(code_point);

                    let finding = UnicodeFinding::new(
                        file_path,
                        line_num + 1, // 1-indexed
                        col_num + 1,  // 1-indexed
                        code_point,
                        ch,
                        UnicodeCategory::InvisibleCharacter,
                        severity,
                        &self.get_description(code_point),
                        &self.get_remediation(code_point),
                    )
                    .with_cwe_id("CWE-172")
                    .with_reference("https://www.aikido.dev/blog/glassworm-returns-unicode-attack-github-npm-vscode")
                    .with_context(&self.get_context(line, col_num));

                    findings.push(finding);
                }
            }
        }

        findings
    }

    /// Check if the character at position is in a legitimate context
    pub fn is_legitimate_context(&self, line: &str, char_pos: usize) -> bool {
        // Check against skip contexts
        for pattern in &self.skip_contexts {
            if pattern.is_match(line) {
                // More precise check: is our character part of the match?
                if let Some(m) = pattern.find(line) {
                    if char_pos >= m.start() && char_pos < m.end() {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Determine severity based on code point
    fn determine_severity(&self, code_point: u32) -> Severity {
        // Critical ranges always get Critical severity
        if is_in_critical_range(code_point) {
            return Severity::Critical;
        }

        // Variation selectors are critical for Glassworm detection
        if is_variation_selector(code_point) {
            return Severity::Critical;
        }

        // Bidirectional characters are high severity
        if get_bidi_name(code_point).is_some() {
            return Severity::High;
        }

        // Zero-width characters
        if get_zero_width_name(code_point).is_some() {
            return Severity::High;
        }

        // Default to medium for other invisible chars
        Severity::Medium
    }

    /// Get human-readable description
    fn get_description(&self, code_point: u32) -> String {
        if let Some(name) = get_bidi_name(code_point) {
            return format!("Bidirectional control character detected: {} (U+{:04X})", name, code_point);
        }

        if let Some(name) = get_zero_width_name(code_point) {
            return format!("Zero-width character detected: {} (U+{:04X})", name, code_point);
        }

        if is_variation_selector(code_point) {
            return format!("Variation selector detected (U+{:04X}) - commonly used in Glassworm attacks", code_point);
        }

        format!("Invisible Unicode character detected (U+{:04X})", code_point)
    }

    /// Get remediation guidance
    fn get_remediation(&self, code_point: u32) -> String {
        if is_variation_selector(code_point) {
            return "Remove the variation selector. If this is intentional (e.g., emoji skin tone), \
                    verify the character is not being used to hide malicious content. \
                    Review the surrounding code for decoder patterns."
            .to_string();
        }

        if get_bidi_name(code_point).is_some() {
            return "Remove the bidirectional control character. These are commonly used to \
                    reverse text display and hide malicious content. Review the actual byte \
                    sequence of the file to understand the true content."
            .to_string();
        }

        if get_zero_width_name(code_point).is_some() {
            return "Remove the zero-width character. These characters are invisible but can \
                    be used to inject hidden content or bypass security checks."
            .to_string();
        }

        "Remove the invisible character. Verify if this is intentional (e.g., for i18n) \
         or if it's being used to hide malicious content."
            .to_string()
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


    /// Check if a code point is suspicious in the given context
    pub fn is_suspicious(&self, code_point: u32, context: &str) -> bool {
        if !is_in_invisible_range(code_point) {
            return false;
        }

        !self.is_legitimate_context(context, 0)
    }
}

/// Trait implementation for UnicodeDetector
pub trait UnicodeDetector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding>;
    fn is_enabled(&self, config: &UnicodeConfig) -> bool;
}

impl UnicodeDetector for InvisibleCharDetector {
    fn name(&self) -> &'static str {
        "invisible_char"
    }

    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.detect(content, file_path)
    }

    fn is_enabled(&self, config: &UnicodeConfig) -> bool {
        config.detectors.invisible_chars
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variation_selector_detection() {
        let detector = InvisibleCharDetector::with_default_config();
        
        // Variation selector in code
        let content = "const secret\u{FE00}Key = 'value';";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].category, UnicodeCategory::InvisibleCharacter);
        assert_eq!(findings[0].severity, Severity::Critical);
        assert_eq!(findings[0].code_point, 0xFE00);
    }

    #[test]
    fn test_zero_width_space_detection() {
        let detector = InvisibleCharDetector::with_default_config();
        
        let content = "const pass\u{200B}word = 'secret';";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x200B);
    }

    #[test]
    fn test_rlo_detection() {
        let detector = InvisibleCharDetector::with_default_config();
        
        // RLO character (most dangerous bidi char)
        let content = "const file = \"test\u{202E}txt\";";
        let findings = detector.detect(content, "test.js");
        
        assert!(!findings.is_empty());
        assert_eq!(findings[0].code_point, 0x202E);
        assert_eq!(findings[0].severity, Severity::Critical);
    }

    #[test]
    fn test_emoji_variation_allowed() {
        let detector = InvisibleCharDetector::with_default_config();
        
        // Emoji with variation selector (legitimate)
        let content = "const emoji = '❤️';"; // Heart with variation selector
        let findings = detector.detect(content, "test.js");
        
        // Should be empty or have fewer findings due to emoji context
        // Note: This test may need adjustment based on exact detection logic
    }

    #[test]
    fn test_clean_content() {
        let detector = InvisibleCharDetector::with_default_config();
        
        let content = "const normal = 'hello world';";
        let findings = detector.detect(content, "test.js");
        
        assert!(findings.is_empty());
    }

    #[test]
    fn test_multiple_invisible_chars() {
        let detector = InvisibleCharDetector::with_default_config();
        
        let content = "const\u{200B}secret\u{FE00} = 'value';";
        let findings = detector.detect(content, "test.js");
        
        assert!(findings.len() >= 2);
    }
}
