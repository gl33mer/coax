//! Threat Model Data Structures
//!
//! This module defines the core data structures for representing threat models,
//! including entry points, data flows, trust boundaries, threats, and assets.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Threat model for a codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatModel {
    /// Repository path or name
    pub repository: String,
    /// When the threat model was generated
    pub generated_at: DateTime<Utc>,
    /// Entry points (HTTP routes, CLI commands, public functions)
    pub entry_points: Vec<EntryPoint>,
    /// Data flows between components
    pub data_flows: Vec<DataFlow>,
    /// Trust boundaries (auth, network, process)
    pub trust_boundaries: Vec<TrustBoundary>,
    /// Identified threats
    pub threats: Vec<Threat>,
    /// Assets to protect
    pub assets: Vec<Asset>,
}

impl ThreatModel {
    /// Create a new empty threat model
    pub fn new(repository: String) -> Self {
        Self {
            repository,
            generated_at: Utc::now(),
            entry_points: Vec::new(),
            data_flows: Vec::new(),
            trust_boundaries: Vec::new(),
            threats: Vec::new(),
            assets: Vec::new(),
        }
    }

    /// Get threat count by severity
    pub fn threats_by_severity(&self) -> ThreatCounts {
        let mut counts = ThreatCounts::default();
        for threat in &self.threats {
            match threat.severity {
                Severity::Critical => counts.critical += 1,
                Severity::High => counts.high += 1,
                Severity::Medium => counts.medium += 1,
                Severity::Low => counts.low += 1,
            }
        }
        counts
    }

    /// Get total risk score
    pub fn total_risk_score(&self) -> u32 {
        self.threats.iter().map(|t| t.risk_score).sum()
    }
}

/// Threat counts by severity
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ThreatCounts {
    pub critical: u32,
    pub high: u32,
    pub medium: u32,
    pub low: u32,
}

/// Entry point (HTTP route, CLI command, public function)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPoint {
    /// Name of the entry point
    pub name: String,
    /// Kind of entry point
    pub kind: EntryPointKind,
    /// File path where it's defined
    pub path: String,
    /// HTTP method (GET, POST, etc.) for HTTP endpoints
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Whether authentication is required
    pub authentication: bool,
    /// Input parameters or data accepted
    pub inputs: Vec<String>,
    /// Line number in source file
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
}

/// Kind of entry point
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EntryPointKind {
    /// HTTP endpoint (REST, GraphQL, etc.)
    Http,
    /// CLI command
    Cli,
    /// Public function with external access
    Function,
    /// WebSocket endpoint
    WebSocket,
    /// RPC endpoint
    Rpc,
    /// GraphQL endpoint
    GraphQl,
}

impl std::fmt::Display for EntryPointKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EntryPointKind::Http => write!(f, "HTTP"),
            EntryPointKind::Cli => write!(f, "CLI"),
            EntryPointKind::Function => write!(f, "Function"),
            EntryPointKind::WebSocket => write!(f, "WebSocket"),
            EntryPointKind::Rpc => write!(f, "RPC"),
            EntryPointKind::GraphQl => write!(f, "GraphQL"),
        }
    }
}

/// Data flow between components
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataFlow {
    /// Source component
    pub from: String,
    /// Destination component
    pub to: String,
    /// Type of data being transferred
    pub data_type: String,
    /// Protocol used (HTTP, TCP, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    /// Whether the data is encrypted in transit
    pub encrypted: bool,
    /// Description of the data flow
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Trust boundary (auth, network, process)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustBoundary {
    /// Name of the trust boundary
    pub name: String,
    /// Kind of trust boundary
    pub kind: TrustBoundaryKind,
    /// Components within this boundary
    pub components: Vec<String>,
    /// Description of the boundary
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Kind of trust boundary
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TrustBoundaryKind {
    /// Authentication boundary (requires auth)
    Auth,
    /// Network boundary (VPC, subnet, firewall)
    Network,
    /// Process boundary (container, service)
    Process,
    /// Encryption boundary (TLS, at-rest)
    Encryption,
    /// Data classification boundary (PII, confidential)
    DataClassification,
}

impl std::fmt::Display for TrustBoundaryKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrustBoundaryKind::Auth => write!(f, "Authentication"),
            TrustBoundaryKind::Network => write!(f, "Network"),
            TrustBoundaryKind::Process => write!(f, "Process"),
            TrustBoundaryKind::Encryption => write!(f, "Encryption"),
            TrustBoundaryKind::DataClassification => write!(f, "Data Classification"),
        }
    }
}

/// Threat with STRIDE category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Threat {
    /// Unique identifier (e.g., THR-001)
    pub id: String,
    /// Short title
    pub title: String,
    /// Detailed description
    pub description: String,
    /// STRIDE category
    pub stride: StrideCategory,
    /// Severity level
    pub severity: Severity,
    /// Likelihood of occurrence
    pub likelihood: Likelihood,
    /// Impact if exploited
    pub impact: Impact,
    /// Calculated risk score (likelihood × impact)
    pub risk_score: u32,
    /// Affected component or file
    pub affected_component: String,
    /// Recommended mitigation
    pub mitigation: String,
    /// Related CWE ID (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cwe_id: Option<String>,
    /// Related finding pattern (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_finding: Option<String>,
    /// Entry point exploited (if applicable)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exploited_entry_point: Option<String>,
}

/// STRIDE categories
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum StrideCategory {
    /// Spoofing - Impersonating a legitimate entity
    Spoofing,
    /// Tampering - Modifying data or code
    Tampering,
    /// Repudiation - Denying an action
    Repudiation,
    /// Information Disclosure - Exposing sensitive data
    InformationDisclosure,
    /// Denial of Service - Disrupting service availability
    DenialOfService,
    /// Elevation of Privilege - Gaining unauthorized access
    ElevationOfPrivilege,
}

impl std::fmt::Display for StrideCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StrideCategory::Spoofing => write!(f, "Spoofing"),
            StrideCategory::Tampering => write!(f, "Tampering"),
            StrideCategory::Repudiation => write!(f, "Repudiation"),
            StrideCategory::InformationDisclosure => write!(f, "Information Disclosure"),
            StrideCategory::DenialOfService => write!(f, "Denial of Service"),
            StrideCategory::ElevationOfPrivilege => write!(f, "Elevation of Privilege"),
        }
    }
}

/// Severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Critical => write!(f, "Critical"),
            Severity::High => write!(f, "High"),
            Severity::Medium => write!(f, "Medium"),
            Severity::Low => write!(f, "Low"),
        }
    }
}

/// Likelihood of occurrence
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Likelihood {
    VeryLow = 1,
    Low = 2,
    Medium = 3,
    High = 4,
    VeryHigh = 5,
}

impl std::fmt::Display for Likelihood {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Likelihood::VeryLow => write!(f, "Very Low"),
            Likelihood::Low => write!(f, "Low"),
            Likelihood::Medium => write!(f, "Medium"),
            Likelihood::High => write!(f, "High"),
            Likelihood::VeryHigh => write!(f, "Very High"),
        }
    }
}

/// Impact if exploited
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Impact {
    Negligible = 1,
    Minor = 2,
    Moderate = 3,
    Major = 4,
    Catastrophic = 5,
}

impl std::fmt::Display for Impact {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Impact::Negligible => write!(f, "Negligible"),
            Impact::Minor => write!(f, "Minor"),
            Impact::Moderate => write!(f, "Moderate"),
            Impact::Major => write!(f, "Major"),
            Impact::Catastrophic => write!(f, "Catastrophic"),
        }
    }
}

/// Asset to protect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    /// Asset name
    pub name: String,
    /// Asset type
    pub kind: AssetKind,
    /// Description
    pub description: String,
    /// Sensitivity level
    pub sensitivity: Sensitivity,
    /// Location (file path, database, etc.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// Kind of asset
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum AssetKind {
    /// Secret or credential
    Secret,
    /// Database
    Database,
    /// API key
    ApiKey,
    /// Private key
    PrivateKey,
    /// PII data
    PiiData,
    /// Configuration
    Configuration,
    /// Source code
    SourceCode,
    /// Infrastructure
    Infrastructure,
}

impl std::fmt::Display for AssetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssetKind::Secret => write!(f, "Secret"),
            AssetKind::Database => write!(f, "Database"),
            AssetKind::ApiKey => write!(f, "API Key"),
            AssetKind::PrivateKey => write!(f, "Private Key"),
            AssetKind::PiiData => write!(f, "PII Data"),
            AssetKind::Configuration => write!(f, "Configuration"),
            AssetKind::SourceCode => write!(f, "Source Code"),
            AssetKind::Infrastructure => write!(f, "Infrastructure"),
        }
    }
}

/// Sensitivity level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Sensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
}

impl std::fmt::Display for Sensitivity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Sensitivity::Public => write!(f, "Public"),
            Sensitivity::Internal => write!(f, "Internal"),
            Sensitivity::Confidential => write!(f, "Confidential"),
            Sensitivity::Restricted => write!(f, "Restricted"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_threat_model_creation() {
        let model = ThreatModel::new("test-repo".to_string());
        assert_eq!(model.repository, "test-repo");
        assert!(model.entry_points.is_empty());
        assert!(model.threats.is_empty());
    }

    #[test]
    fn test_threat_counts() {
        let mut model = ThreatModel::new("test".to_string());
        model.threats.push(Threat {
            id: "THR-001".to_string(),
            title: "Test".to_string(),
            description: "Test threat".to_string(),
            stride: StrideCategory::Spoofing,
            severity: Severity::Critical,
            likelihood: Likelihood::High,
            impact: Impact::Major,
            risk_score: 20,
            affected_component: "test.rs".to_string(),
            mitigation: "Fix it".to_string(),
            cwe_id: None,
            related_finding: None,
            exploited_entry_point: None,
        });
        model.threats.push(Threat {
            id: "THR-002".to_string(),
            title: "Test 2".to_string(),
            description: "Test threat 2".to_string(),
            stride: StrideCategory::Tampering,
            severity: Severity::High,
            likelihood: Likelihood::Medium,
            impact: Impact::Moderate,
            risk_score: 9,
            affected_component: "test.rs".to_string(),
            mitigation: "Fix it".to_string(),
            cwe_id: None,
            related_finding: None,
            exploited_entry_point: None,
        });

        let counts = model.threats_by_severity();
        assert_eq!(counts.critical, 1);
        assert_eq!(counts.high, 1);
        assert_eq!(counts.medium, 0);
        assert_eq!(counts.low, 0);
    }

    #[test]
    fn test_stride_display() {
        assert_eq!(format!("{}", StrideCategory::Spoofing), "Spoofing");
        assert_eq!(
            format!("{}", StrideCategory::InformationDisclosure),
            "Information Disclosure"
        );
    }

    #[test]
    fn test_severity_display() {
        assert_eq!(format!("{}", Severity::Critical), "Critical");
        assert_eq!(format!("{}", Severity::Low), "Low");
    }
}
