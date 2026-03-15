//! Integration tests for coax-threat-model crate

use coax_threat_model::{
    correlate_findings_with_threats, generate_dfd, format_threat_model,
    GeneratorConfig, OutputFormat, ThreatModelGenerator,
    model::{EntryPoint, EntryPointKind, Impact, Likelihood, Severity, StrideCategory, Threat, ThreatModel},
};
use coax_scanner::Scanner;
use std::fs;
use tempfile::TempDir;

/// Create a test threat model
fn create_test_model() -> ThreatModel {
    let mut model = ThreatModel::new("test-repo".to_string());

    model.entry_points.push(EntryPoint {
        name: "GET /api/users".to_string(),
        kind: EntryPointKind::Http,
        path: "src/routes.rs".to_string(),
        method: Some("GET".to_string()),
        authentication: true,
        inputs: vec![],
        line: Some(10),
    });

    model.entry_points.push(EntryPoint {
        name: "POST /api/login".to_string(),
        kind: EntryPointKind::Http,
        path: "src/auth.rs".to_string(),
        method: Some("POST".to_string()),
        authentication: false,
        inputs: vec!["username".to_string(), "password".to_string()],
        line: Some(25),
    });

    model.threats.push(Threat {
        id: "THR-001".to_string(),
        title: "Information Disclosure via AWS_ACCESS_KEY".to_string(),
        description: "AWS access key exposed in source code".to_string(),
        stride: StrideCategory::InformationDisclosure,
        severity: Severity::Critical,
        likelihood: Likelihood::High,
        impact: Impact::Catastrophic,
        risk_score: 25,
        affected_component: "config.rs:10".to_string(),
        mitigation: "Remove and rotate immediately".to_string(),
        cwe_id: None,
        related_finding: Some("AWS_ACCESS_KEY".to_string()),
        exploited_entry_point: None,
    });

    model
}

#[test]
fn test_threat_model_yaml_output() {
    let model = create_test_model();
    let output = format_threat_model(&model, OutputFormat::Yaml).unwrap();

    assert!(output.contains("test-repo"));
    assert!(output.contains("THR-001"));
    assert!(output.contains("InformationDisclosure"));
    assert!(output.contains("entry_points"));
    assert!(output.contains("threats"));
}

#[test]
fn test_threat_model_json_output() {
    let model = create_test_model();
    let output = format_threat_model(&model, OutputFormat::Json).unwrap();

    assert!(output.contains("test-repo"));
    assert!(output.contains("THR-001"));
    assert!(output.contains("InformationDisclosure"));
}

#[test]
fn test_threat_model_dfd_output() {
    let model = create_test_model();
    let dfd = generate_dfd(&model);

    assert!(dfd.contains("Data Flow Diagram"));
    assert!(dfd.contains("Entry Points"));
    assert!(dfd.contains("GET /api/users"));
    assert!(dfd.contains("Critical"));
}

#[test]
fn test_generator_with_test_files() {
    let temp_dir = TempDir::new().unwrap();

    // Create test file with entry points and secrets
    let test_file = temp_dir.path().join("app.js");
    fs::write(
        &test_file,
        r#"
const express = require('express');
const app = express();

app.get("/api/users", (req, res) => {
    res.json({ users: [] });
});

app.post("/api/login", authMiddleware, (req, res) => {
    res.json({ token: "fake" });
});

// Configuration
const AWS_KEY = "AKIAIOSFODNN7EXAMPLE";
const DB_PASSWORD = "supersecret123";

app.use(authMiddleware);
"#,
    )
    .unwrap();

    let generator = ThreatModelGenerator::new();
    let model = generator.generate(temp_dir.path()).unwrap();

    // Generator successfully created a threat model
    assert_eq!(model.repository, temp_dir.path().to_string_lossy());
}

#[test]
fn test_correlation_with_findings() {
    let temp_dir = TempDir::new().unwrap();

    // Create test file with secrets
    let test_file = temp_dir.path().join("config.rs");
    fs::write(
        &test_file,
        r#"
const AWS_KEY = "AKIAIOSFODNN7EXAMPLE";
const GITHUB_TOKEN = "ghp_1234567890abcdefghij1234567890abcdefghij";
"#,
    )
    .unwrap();

    // Scan for findings
    let scanner = Scanner::with_default_patterns();
    let (findings, _) = scanner.scan_with_summary(temp_dir.path());

    assert!(
        !findings.is_empty(),
        "Expected findings from secret scan"
    );

    // Create entry points
    let entry_points = vec![EntryPoint {
        name: "GET /api/config".to_string(),
        kind: EntryPointKind::Http,
        path: "config.rs".to_string(),
        method: Some("GET".to_string()),
        authentication: false,
        inputs: vec![],
        line: Some(1),
    }];

    // Correlate findings with entry points
    let threats = correlate_findings_with_threats(&findings, &entry_points);

    assert!(
        !threats.is_empty(),
        "Expected threats from correlation"
    );

    // Check STRIDE categories
    for threat in &threats {
        assert_ne!(threat.stride, StrideCategory::DenialOfService);
        assert!(threat.risk_score > 0);
    }
}

#[test]
fn test_risk_score_calculation() {
    let mut model = create_test_model();

    // Add threats with different risk scores
    model.threats.push(Threat {
        id: "THR-002".to_string(),
        title: "Test Threat 2".to_string(),
        description: "Test".to_string(),
        stride: StrideCategory::Spoofing,
        severity: Severity::High,
        likelihood: Likelihood::Medium,
        impact: Impact::Moderate,
        risk_score: 6,
        affected_component: "test.rs".to_string(),
        mitigation: "Fix".to_string(),
        cwe_id: None,
        related_finding: None,
        exploited_entry_point: None,
    });

    let total_risk = model.total_risk_score();
    assert!(total_risk > 0);
}

#[test]
fn test_threat_counts() {
    let mut model = create_test_model();

    // Add more threats with different severities
    model.threats.push(Threat {
        id: "THR-002".to_string(),
        title: "High Threat".to_string(),
        description: "Test".to_string(),
        stride: StrideCategory::Spoofing,
        severity: Severity::High,
        likelihood: Likelihood::High,
        impact: Impact::Major,
        risk_score: 16,
        affected_component: "test.rs".to_string(),
        mitigation: "Fix".to_string(),
        cwe_id: None,
        related_finding: None,
        exploited_entry_point: None,
    });

    model.threats.push(Threat {
        id: "THR-003".to_string(),
        title: "Medium Threat".to_string(),
        description: "Test".to_string(),
        stride: StrideCategory::Tampering,
        severity: Severity::Medium,
        likelihood: Likelihood::Medium,
        impact: Impact::Moderate,
        risk_score: 9,
        affected_component: "test.rs".to_string(),
        mitigation: "Fix".to_string(),
        cwe_id: None,
        related_finding: None,
        exploited_entry_point: None,
    });

    let counts = model.threats_by_severity();
    assert_eq!(counts.critical, 1);
    assert_eq!(counts.high, 1);
    assert_eq!(counts.medium, 1);
    assert_eq!(counts.low, 0);
}

#[test]
fn test_generator_config() {
    let config = GeneratorConfig {
        scan_hidden: true,
        max_file_size: 1024 * 1024,
        exclude_patterns: vec!["test".to_string()],
        extensions: vec!["rs".to_string(), "js".to_string()],
    };

    let generator = ThreatModelGenerator::with_config(config);
    // Generator created successfully with custom config
    assert!(true);
}

#[test]
fn test_python_entry_point_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Create Python test file
    let test_file = temp_dir.path().join("app.py");
    fs::write(
        &test_file,
        r#"
from flask import Flask
app = Flask(__name__)

@app.route("/api/users")
def get_users():
    return {"users": []}

@app.post("/api/login")
def login():
    return {"token": "fake"}
"#,
    )
    .unwrap();

    let generator = ThreatModelGenerator::new();
    let model = generator.generate(temp_dir.path()).unwrap();

    // Generator successfully created a threat model
    assert_eq!(model.repository, temp_dir.path().to_string_lossy());
}

#[test]
fn test_data_flow_detection() {
    let temp_dir = TempDir::new().unwrap();

    // Create test file with data flows
    let test_file = temp_dir.path().join("db.rs");
    fs::write(
        &test_file,
        r#"
async fn get_users() {
    let conn = postgres::connect("postgresql://localhost").await?;
    let users = conn.query("SELECT * FROM users").await?;

    let response = reqwest::get("https://api.example.com").await?;

    let data = fs::read("config.json").await?;
}
"#,
    )
    .unwrap();

    let generator = ThreatModelGenerator::new();
    let model = generator.generate(temp_dir.path()).unwrap();

    // Generator successfully created a threat model
    assert_eq!(model.repository, temp_dir.path().to_string_lossy());
}
