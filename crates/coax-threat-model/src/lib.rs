//! Coax Threat Model
//!
//! Threat modeling capabilities for the coax security scanner, including:
//! - STRIDE threat categorization
//! - Threat model generation
//! - Entry point detection
//! - Trust boundary detection
//! - Data flow analysis
//! - Finding correlation
//! - ASCII DFD generation
//!
//! # Example
//!
//! ```rust
//! use coax_threat_model::{ThreatModelGenerator, GeneratorConfig};
//! use std::path::Path;
//!
//! // Create generator
//! let generator = ThreatModelGenerator::new();
//!
//! // Generate threat model
//! let model = generator.generate(Path::new(".")).unwrap();
//!
//! // Access results
//! println!("Found {} threats", model.threats.len());
//! println!("Found {} entry points", model.entry_points.len());
//! ```

pub mod model;
pub mod stride;
pub mod generator;
pub mod correlation;
pub mod dfd;

// Re-export main types for convenience
pub use model::{
    Asset, AssetKind, DataFlow, EntryPoint, EntryPointKind, Impact, Likelihood, Sensitivity,
    Severity, StrideCategory, Threat, ThreatCounts, ThreatModel, TrustBoundary, TrustBoundaryKind,
};

pub use generator::{GeneratorConfig, ThreatModelGenerator};

pub use correlation::{
    correlate_findings_with_threats, enhance_threat_model, find_related_entry_points,
    get_priority_recommendations, RiskHeatmap,
};

pub use dfd::{generate_component_diagram, generate_dfd, generate_simple_dfd};

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Output format for threat models
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// YAML format (Threagile-compatible)
    Yaml,
    /// JSON format
    Json,
    /// Text format with ASCII DFD
    Text,
    /// Text format with simple DFD
    SimpleText,
    /// Component diagram
    Component,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Text => write!(f, "text"),
            OutputFormat::SimpleText => write!(f, "simple-text"),
            OutputFormat::Component => write!(f, "component"),
        }
    }
}

/// Format a threat model for output
pub fn format_threat_model(model: &ThreatModel, format: OutputFormat) -> Result<String, FormatError> {
    match format {
        OutputFormat::Yaml => Ok(serde_yaml::to_string(model).map_err(|e| FormatError::Yaml(e.to_string()))?),
        OutputFormat::Json => Ok(serde_json::to_string_pretty(model).map_err(|e| FormatError::Json(e.to_string()))?),
        OutputFormat::Text => Ok(generate_dfd(model)),
        OutputFormat::SimpleText => Ok(generate_simple_dfd(model)),
        OutputFormat::Component => Ok(generate_component_diagram(model)),
    }
}

/// Error type for formatting operations
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
    #[error("YAML formatting error: {0}")]
    Yaml(String),
    #[error("JSON formatting error: {0}")]
    Json(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn create_test_model() -> ThreatModel {
        let mut model = ThreatModel::new("test-repo".to_string());
        model.generated_at = Utc::now();

        model.entry_points.push(EntryPoint {
            name: "GET /api/users".to_string(),
            kind: EntryPointKind::Http,
            path: "src/routes.rs".to_string(),
            method: Some("GET".to_string()),
            authentication: true,
            inputs: vec![],
            line: Some(10),
        });

        model.threats.push(Threat {
            id: "THR-001".to_string(),
            title: "Test Threat".to_string(),
            description: "Test description".to_string(),
            stride: StrideCategory::Spoofing,
            severity: Severity::High,
            likelihood: Likelihood::Medium,
            impact: Impact::Moderate,
            risk_score: 6,
            affected_component: "test.rs:10".to_string(),
            mitigation: "Fix it".to_string(),
            cwe_id: None,
            related_finding: None,
            exploited_entry_point: None,
        });

        model
    }

    #[test]
    fn test_yaml_output() {
        let model = create_test_model();
        let output = format_threat_model(&model, OutputFormat::Yaml).unwrap();
        assert!(output.contains("test-repo"));
        assert!(output.contains("THR-001"));
    }

    #[test]
    fn test_json_output() {
        let model = create_test_model();
        let output = format_threat_model(&model, OutputFormat::Json).unwrap();
        assert!(output.contains("test-repo"));
        assert!(output.contains("THR-001"));
    }

    #[test]
    fn test_text_output() {
        let model = create_test_model();
        let output = format_threat_model(&model, OutputFormat::Text).unwrap();
        assert!(output.contains("Data Flow Diagram"));
    }

    #[test]
    fn test_output_format_display() {
        assert_eq!(format!("{}", OutputFormat::Yaml), "yaml");
        assert_eq!(format!("{}", OutputFormat::Json), "json");
        assert_eq!(format!("{}", OutputFormat::Text), "text");
    }
}
