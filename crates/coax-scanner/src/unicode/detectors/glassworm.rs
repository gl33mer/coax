//! Glassworm-Specific Detector
//!
//! Specialized detector for Glassworm attack patterns.

use crate::unicode::config::UnicodeConfig;
use crate::unicode::findings::{Severity, SourceLocation, UnicodeCategory, UnicodeFinding};
use lazy_static::lazy_static;
use regex::Regex;

/// A Glassworm indicator
#[derive(Debug, Clone)]
pub struct GlasswormIndicator {
    pub indicator_type: String,
    pub location: SourceLocation,
    pub snippet: String,
    pub confidence: f32,
}

/// Detector for Glassworm attack patterns
pub struct GlasswormDetector {
    decoder_patterns: Vec<Regex>,
    eval_patterns: Vec<Regex>,
    encoding_patterns: Vec<Regex>,
    config: UnicodeConfig,
}

lazy_static! {
    /// Decoder patterns characteristic of Glassworm
    static ref DECODER_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"codePointAt\s*\(\s*0\s*\)").unwrap(),
        Regex::new(r"String\.fromCharCode\s*\(").unwrap(),
        Regex::new(r"String\.fromCodePoint\s*\(").unwrap(),
        Regex::new(r"\.filter\s*\(\s*c\s*=>\s*c\s*!==\s*null\s*\)").unwrap(),
    ];

    /// Eval patterns for code execution
    static ref EVAL_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"\beval\s*\(").unwrap(),
        Regex::new(r"\bFunction\s*\(").unwrap(),
        Regex::new(r"new\s+Function\s*\(").unwrap(),
    ];

    /// Encoding/decoding patterns
    static ref ENCODING_PATTERNS: Vec<Regex> = vec![
        Regex::new(r"Buffer\.from\s*\([^,]+,\s*hex\s*\)").unwrap(),
        Regex::new(r"Buffer\.from\s*\([^,]+,\s*base64\s*\)").unwrap(),
        Regex::new(r"\batob\s*\(").unwrap(),
        Regex::new(r"\bbtoa\s*\(").unwrap(),
    ];
}

impl GlasswormDetector {
    pub fn new(config: UnicodeConfig) -> Self {
        Self {
            decoder_patterns: DECODER_PATTERNS.clone(),
            eval_patterns: EVAL_PATTERNS.clone(),
            encoding_patterns: ENCODING_PATTERNS.clone(),
            config,
        }
    }

    pub fn with_default_config() -> Self {
        Self::new(UnicodeConfig::default())
    }

    pub fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        let mut findings = Vec::new();
        let mut indicators = Vec::new();

        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.decoder_patterns {
                if let Some(m) = pattern.find(line) {
                    indicators.push(GlasswormIndicator {
                        indicator_type: "decoder_pattern".to_string(),
                        location: SourceLocation::new(file_path, line_num + 1, m.start() + 1),
                        snippet: line[m.start()..m.end()].to_string(),
                        confidence: 0.7,
                    });
                }
            }

            for pattern in &self.eval_patterns {
                if let Some(m) = pattern.find(line) {
                    indicators.push(GlasswormIndicator {
                        indicator_type: "eval_pattern".to_string(),
                        location: SourceLocation::new(file_path, line_num + 1, m.start() + 1),
                        snippet: line[m.start()..m.end()].to_string(),
                        confidence: 0.8,
                    });
                }
            }

            for pattern in &self.encoding_patterns {
                if let Some(m) = pattern.find(line) {
                    indicators.push(GlasswormIndicator {
                        indicator_type: "encoding_pattern".to_string(),
                        location: SourceLocation::new(file_path, line_num + 1, m.start() + 1),
                        snippet: line[m.start()..m.end()].to_string(),
                        confidence: 0.6,
                    });
                }
            }
        }

        let confidence = self.calculate_confidence(&indicators);

        if confidence >= 0.5 {
            for indicator in &indicators {
                let severity = if confidence >= 0.8 {
                    Severity::Critical
                } else if confidence >= 0.6 {
                    Severity::High
                } else {
                    Severity::Medium
                };

                let finding = UnicodeFinding::new(
                    file_path,
                    indicator.location.line,
                    indicator.location.column,
                    0,
                    '\0',
                    UnicodeCategory::GlasswormPattern,
                    severity,
                    &format!(
                        "Glassworm attack pattern detected: {} (confidence: {:.0}%)",
                        indicator.indicator_type,
                        confidence * 100.0
                    ),
                    &self.get_remediation(confidence),
                )
                .with_cwe_id("CWE-956")
                .with_reference("https://www.aikido.dev/blog/glassworm-returns-unicode-attack-github-npm-vscode")
                .with_context(&indicator.snippet);

                findings.push(finding);
            }
        }

        findings
    }

    pub fn detect_decoder(&self, content: &str) -> Option<GlasswormIndicator> {
        for (line_num, line) in content.lines().enumerate() {
            for pattern in &self.decoder_patterns {
                if let Some(m) = pattern.find(line) {
                    return Some(GlasswormIndicator {
                        indicator_type: "decoder".to_string(),
                        location: SourceLocation::new("", line_num + 1, m.start() + 1),
                        snippet: line[m.start()..m.end()].to_string(),
                        confidence: 0.7,
                    });
                }
            }
        }
        None
    }

    pub fn calculate_confidence(&self, indicators: &[GlasswormIndicator]) -> f32 {
        if indicators.is_empty() {
            return 0.0;
        }

        let sum_confidence: f32 = indicators.iter().map(|i| i.confidence).sum();
        let avg_confidence = sum_confidence / indicators.len() as f32;

        let unique_types: std::collections::HashSet<_> =
            indicators.iter().map(|i| &i.indicator_type).collect();

        let type_bonus = match unique_types.len() {
            1 => 0.0,
            2 => 0.1,
            3 => 0.2,
            _ => 0.25,
        };

        let count_bonus = match indicators.len() {
            1 => 0.0,
            2..=3 => 0.05,
            4..=5 => 0.1,
            _ => 0.15,
        };

        (avg_confidence + type_bonus + count_bonus).min(1.0)
    }

    fn get_remediation(&self, confidence: f32) -> String {
        if confidence >= 0.8 {
            "CRITICAL: This code exhibits strong Glassworm attack characteristics.".to_string()
        } else if confidence >= 0.6 {
            "HIGH: This code shows patterns consistent with Glassworm-style attacks.".to_string()
        } else {
            "MEDIUM: Some patterns associated with Glassworm attacks were detected.".to_string()
        }
    }
}

pub trait UnicodeDetector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding>;
    fn is_enabled(&self, config: &UnicodeConfig) -> bool;
}

impl UnicodeDetector for GlasswormDetector {
    fn name(&self) -> &'static str {
        "glassworm"
    }

    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
        self.detect(content, file_path)
    }

    fn is_enabled(&self, config: &UnicodeConfig) -> bool {
        config.detectors.glassworm
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decoder_pattern_detection() {
        let detector = GlasswormDetector::with_default_config();
        let content = r#"const codes = "secret".split('').map(c => c.codePointAt(0));"#;
        let findings = detector.detect(content, "test.js");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_eval_pattern_detection() {
        let detector = GlasswormDetector::with_default_config();
        let content = r#"eval(code);"#;
        let findings = detector.detect(content, "test.js");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_clean_content() {
        let detector = GlasswormDetector::with_default_config();
        let content = r#"const normal = 'hello world';"#;
        let findings = detector.detect(content, "test.js");
        assert!(findings.is_empty());
    }
}
