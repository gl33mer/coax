//! Scan Result Types
//!
//! This module defines the data structures for representing scan results.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// A single finding from the security scan
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ScanResult {
    /// File path where the finding was detected
    pub file: PathBuf,
    /// Line number (1-indexed)
    pub line: u32,
    /// Column number (1-indexed), if available
    pub column: Option<u32>,
    /// Pattern name that matched
    pub pattern: String,
    /// Severity level: critical, high, medium, low
    pub severity: String,
    /// Recommendation for remediation
    pub recommendation: String,
    /// The actual detected secret (masked if sensitive)
    pub detected_secret: Option<String>,
    /// The actual line content (optional, for reporting)
    pub line_content: Option<String>,
    /// Context information about the finding
    pub context: FindingContext,
}

/// Context information about a finding
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FindingContext {
    /// Whether the finding is in a comment
    pub is_comment: bool,
    /// Whether the finding is in a test file
    pub is_test_file: bool,
    /// Whether the finding is in documentation
    pub is_documentation: bool,
    /// Whether the finding appears to be a placeholder
    pub is_placeholder: bool,
    /// Whether the finding is an AWS example key
    pub is_aws_example: bool,
    /// Adjusted severity based on context
    pub adjusted_severity: Option<String>,
    /// Reason for severity adjustment or exclusion
    pub note: Option<String>,
}

impl ScanResult {
    /// Create a new scan result
    pub fn new(
        file: PathBuf,
        line: u32,
        pattern: String,
        severity: String,
        recommendation: String,
    ) -> Self {
        Self {
            file,
            line,
            column: None,
            pattern,
            severity,
            recommendation,
            detected_secret: None,
            line_content: None,
            context: FindingContext::default(),
        }
    }

    /// Create a scan result with line content
    pub fn with_line_content(mut self, content: String) -> Self {
        self.line_content = Some(content);
        self
    }

    /// Create a scan result with detected secret
    pub fn with_detected_secret(mut self, secret: String) -> Self {
        self.detected_secret = Some(secret);
        self
    }

    /// Create a scan result with column information
    pub fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }

    /// Create a scan result with context
    pub fn with_context(mut self, context: FindingContext) -> Self {
        self.context = context;
        self
    }

    /// Get the severity as a numeric value for sorting
    pub fn severity_score(&self) -> u8 {
        match self.severity.to_lowercase().as_str() {
            "critical" => 4,
            "high" => 3,
            "medium" => 2,
            "low" => 1,
            _ => 0,
        }
    }

    /// Check if this is a critical finding
    pub fn is_critical(&self) -> bool {
        self.severity.to_lowercase() == "critical"
    }

    /// Check if this is a high severity finding
    pub fn is_high(&self) -> bool {
        self.severity.to_lowercase() == "high"
    }
}

impl std::fmt::Display for ScanResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{} - {} [{}] - {}",
            self.file.display(),
            self.line,
            self.pattern,
            self.severity,
            self.recommendation
        )
    }
}

/// Summary statistics for a scan
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScanSummary {
    /// Total number of files scanned
    pub files_scanned: u32,
    /// Total number of findings
    pub total_findings: u32,
    /// Findings by severity
    pub by_severity: SeverityCounts,
    /// Top patterns detected
    pub top_patterns: Vec<PatternCount>,
    /// Scan duration in milliseconds
    pub scan_duration_ms: u64,
}

/// Count of findings by severity level
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SeverityCounts {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
}

impl SeverityCounts {
    /// Create from scan results
    pub fn from_results(results: &[ScanResult]) -> Self {
        let mut counts = Self::default();
        for result in results {
            match result.severity.to_lowercase().as_str() {
                "critical" => counts.critical += 1,
                "high" => counts.high += 1,
                "medium" => counts.medium += 1,
                "low" => counts.low += 1,
                _ => {}
            }
        }
        counts
    }

    /// Total count across all severities
    pub fn total(&self) -> u32 {
        self.critical + self.high + self.medium + self.low
    }
}

/// Pattern occurrence count
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCount {
    pub pattern: String,
    pub count: u32,
}

/// Output format for scan results
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Yaml,
    Sarif,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => Self::Json,
            "yaml" | "yml" => Self::Yaml,
            "sarif" => Self::Sarif,
            _ => Self::Text,
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Json => write!(f, "json"),
            Self::Yaml => write!(f, "yaml"),
            Self::Sarif => write!(f, "sarif"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_result_creation() {
        let result = ScanResult::new(
            PathBuf::from("test.txt"),
            42,
            "AWS_KEY".to_string(),
            "critical".to_string(),
            "Rotate immediately".to_string(),
        );

        assert_eq!(result.file, PathBuf::from("test.txt"));
        assert_eq!(result.line, 42);
        assert_eq!(result.pattern, "AWS_KEY");
        assert!(result.is_critical());
    }

    #[test]
    fn test_severity_counts() {
        let results = vec![
            ScanResult::new(
                PathBuf::from("a.txt"),
                1,
                "AWS".to_string(),
                "critical".to_string(),
                "".to_string(),
            ),
            ScanResult::new(
                PathBuf::from("b.txt"),
                2,
                "GITHUB".to_string(),
                "high".to_string(),
                "".to_string(),
            ),
            ScanResult::new(
                PathBuf::from("c.txt"),
                3,
                "GENERIC".to_string(),
                "medium".to_string(),
                "".to_string(),
            ),
        ];

        let counts = SeverityCounts::from_results(&results);
        assert_eq!(counts.critical, 1);
        assert_eq!(counts.high, 1);
        assert_eq!(counts.medium, 1);
        assert_eq!(counts.total(), 3);
    }

    #[test]
    fn test_output_format_parsing() {
        assert!(matches!(OutputFormat::from_str("json"), OutputFormat::Json));
        assert!(matches!(OutputFormat::from_str("yaml"), OutputFormat::Yaml));
        assert!(matches!(OutputFormat::from_str("yml"), OutputFormat::Yaml));
        assert!(matches!(
            OutputFormat::from_str("sarif"),
            OutputFormat::Sarif
        ));
        assert!(matches!(OutputFormat::from_str("text"), OutputFormat::Text));
        assert!(matches!(
            OutputFormat::from_str("unknown"),
            OutputFormat::Text
        ));
    }
}
