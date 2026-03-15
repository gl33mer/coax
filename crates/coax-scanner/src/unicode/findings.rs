//! Unicode Finding Types
//!
//! This module defines the data structures for representing Unicode attack findings.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Severity levels for Unicode findings
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Low => "low",
            Severity::Medium => "medium",
            Severity::High => "high",
            Severity::Critical => "critical",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => Severity::Critical,
            "high" => Severity::High,
            "medium" => Severity::Medium,
            _ => Severity::Low,
        }
    }
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Category of Unicode attack
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnicodeCategory {
    InvisibleCharacter,
    Homoglyph,
    BidirectionalOverride,
    UnicodeTag,
    NormalizationAttack,
    GlasswormPattern,
    EmojiObfuscation,
    Unknown,
}

impl UnicodeCategory {
    pub fn as_str(&self) -> &'static str {
        match self {
            UnicodeCategory::InvisibleCharacter => "invisible_character",
            UnicodeCategory::Homoglyph => "homoglyph",
            UnicodeCategory::BidirectionalOverride => "bidirectional_override",
            UnicodeCategory::UnicodeTag => "unicode_tag",
            UnicodeCategory::NormalizationAttack => "normalization_attack",
            UnicodeCategory::GlasswormPattern => "glassworm_pattern",
            UnicodeCategory::EmojiObfuscation => "emoji_obfuscation",
            UnicodeCategory::Unknown => "unknown",
        }
    }
}

/// Represents a source location in a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub byte_offset: Option<usize>,
}

impl SourceLocation {
    pub fn new(file: &str, line: usize, column: usize) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
            byte_offset: None,
        }
    }
}

/// A Unicode security finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodeFinding {
    /// File path where the finding was detected
    pub file: String,
    /// Line number (1-indexed)
    pub line: usize,
    /// Column number (1-indexed)
    pub column: usize,
    /// Unicode code point value
    pub code_point: u32,
    /// The character itself (may be empty for invisible chars)
    pub character: String,
    /// Category of the attack
    pub category: UnicodeCategory,
    /// Severity level
    pub severity: Severity,
    /// Human-readable description
    pub description: String,
    /// Remediation guidance
    pub remediation: String,
    /// CWE ID if applicable (e.g., "CWE-172")
    pub cwe_id: Option<String>,
    /// References to research/advisories
    pub references: Vec<String>,
    /// Optional snippet of surrounding context
    pub context: Option<String>,
}

impl UnicodeFinding {
    pub fn new(
        file: &str,
        line: usize,
        column: usize,
        code_point: u32,
        character: char,
        category: UnicodeCategory,
        severity: Severity,
        description: &str,
        remediation: &str,
    ) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
            code_point,
            character: character.to_string(),
            category,
            severity,
            description: description.to_string(),
            remediation: remediation.to_string(),
            cwe_id: None,
            references: Vec::new(),
            context: None,
        }
    }

    pub fn with_cwe_id(mut self, cwe_id: &str) -> Self {
        self.cwe_id = Some(cwe_id.to_string());
        self
    }

    pub fn with_reference(mut self, url: &str) -> Self {
        self.references.push(url.to_string());
        self
    }

    pub fn with_context(mut self, context: &str) -> Self {
        self.context = Some(context.to_string());
        self
    }

    pub fn location(&self) -> SourceLocation {
        SourceLocation::new(&self.file, self.line, self.column)
    }
}

impl fmt::Display for UnicodeFinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}:{}:{} - U+{:04X} ({}) - {}",
            self.severity.as_str().to_uppercase(),
            self.file,
            self.line,
            self.column,
            self.code_point,
            self.category.as_str(),
            self.description
        )
    }
}

/// Statistics for a Unicode scan
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UnicodeScanStats {
    pub total_characters_scanned: usize,
    pub total_files_scanned: usize,
    pub total_findings: usize,
    pub findings_by_category: std::collections::HashMap<String, usize>,
    pub findings_by_severity: std::collections::HashMap<String, usize>,
    pub scan_duration_ms: u64,
}

impl UnicodeScanStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn increment_category(&mut self, category: &UnicodeCategory) {
        *self.findings_by_category.entry(category.as_str().to_string()).or_insert(0) += 1;
    }

    pub fn increment_severity(&mut self, severity: &Severity) {
        *self.findings_by_severity.entry(severity.as_str().to_string()).or_insert(0) += 1;
    }
}
