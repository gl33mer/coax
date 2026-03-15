//! Data Flow Diagram (DFD) Generation Module
//!
//! This module provides ASCII art DFD generation for visualizing
//! threat models and security findings.

use crate::model::{EntryPointKind, Threat, ThreatModel, TrustBoundaryKind};
use std::fmt::Write;

/// Generate ASCII art DFD from threat model
pub fn generate_dfd(model: &ThreatModel) -> String {
    let mut output = String::new();

    // Header
    writeln!(
        output,
        "╔══════════════════════════════════════════════════════════════════════╗"
    )
    .unwrap();
    writeln!(output, "║  Data Flow Diagram - {:<54} ║", truncate(&model.repository, 54)).unwrap();
    writeln!(
        output,
        "╠══════════════════════════════════════════════════════════════════════╣"
    )
    .unwrap();
    writeln!(output, "║  Generated: {:<59} ║", model.generated_at.format("%Y-%m-%d %H:%M:%S UTC")).unwrap();
    writeln!(
        output,
        "╠══════════════════════════════════════════════════════════════════════╣"
    )
    .unwrap();
    writeln!(output, "║                                                          ║").unwrap();

    // Entry Points Section
    if !model.entry_points.is_empty() {
        writeln!(output, "║  Entry Points:                                             ║").unwrap();
        writeln!(output, "║  ┌────────────────────────────────────────────────────┐    ║").unwrap();

        for (i, ep) in model.entry_points.iter().take(5).enumerate() {
            let auth_indicator = if ep.authentication { "🔐" } else { "🔓" };
            let kind_indicator = match ep.kind {
                EntryPointKind::Http => "🌐",
                EntryPointKind::Cli => "💻",
                EntryPointKind::Function => "⚙️",
                EntryPointKind::WebSocket => "🔌",
                EntryPointKind::Rpc => "📡",
                EntryPointKind::GraphQl => "🔷",
            };

            let method = ep.method.as_deref().unwrap_or("EP");
            let line = format!(
                "║  │  {} {} {} {:<35} │    ║",
                kind_indicator, auth_indicator, method, truncate(&ep.name, 35)
            );
            writeln!(output, "{}", line).unwrap();

            if i >= 4 && model.entry_points.len() > 5 {
                writeln!(
                    output,
                    "║  │  ... and {} more                                     │    ║",
                    model.entry_points.len() - 5
                )
                .unwrap();
                break;
            }
        }

        writeln!(output, "║  └────────────────────────────────────────────────────┘    ║").unwrap();
        writeln!(output, "║                                                          ║").unwrap();
    }

    // Data Flows Section
    if !model.data_flows.is_empty() {
        writeln!(output, "║  Data Flows:                                               ║").unwrap();

        for flow in model.data_flows.iter().take(5) {
            let encryption_indicator = if flow.encrypted { "🔒" } else { "🔓" };
            let protocol = flow.protocol.as_deref().unwrap_or("N/A");

            writeln!(
                output,
                "║    [App] {}──{}──{}> [{}]                            ║",
                encryption_indicator, protocol, flow.data_type, truncate(&flow.to, 20)
            )
            .unwrap();
        }

        if model.data_flows.len() > 5 {
            writeln!(
                output,
                "║    ... and {} more data flows                            ║",
                model.data_flows.len() - 5
            )
            .unwrap();
        }

        writeln!(output, "║                                                          ║").unwrap();
    }

    // Trust Boundaries Section
    if !model.trust_boundaries.is_empty() {
        writeln!(output, "║  Trust Boundaries:                                         ║").unwrap();
        writeln!(output, "║  ┌─────────────────────────────────────────────────┐       ║").unwrap();

        for boundary in model.trust_boundaries.iter().take(3) {
            let icon = match boundary.kind {
                TrustBoundaryKind::Auth => "🔐",
                TrustBoundaryKind::Network => "🌐",
                TrustBoundaryKind::Process => "📦",
                TrustBoundaryKind::Encryption => "🔒",
                TrustBoundaryKind::DataClassification => "📊",
            };

            let line = format!(
                "║  │  {} {:<48} │       ║",
                icon,
                truncate(&boundary.name, 48)
            );
            writeln!(output, "{}", line).unwrap();
        }

        if model.trust_boundaries.len() > 3 {
            writeln!(
                output,
                "║  │  ... and {} more boundaries                          │       ║",
                model.trust_boundaries.len() - 3
            )
            .unwrap();
        }

        writeln!(output, "║  └─────────────────────────────────────────────────┘       ║").unwrap();
        writeln!(output, "║                                                          ║").unwrap();
    }

    // Threats Summary Section
    let counts = model.threats_by_severity();
    writeln!(output, "║  Threats Summary:                                          ║").unwrap();
    writeln!(
        output,
        "║    🚨 Critical: {:<3}  ⚠️  High: {:<3}  ⚡ Medium: {:<3}  ℹ️  Low: {:<3}    ║",
        counts.critical, counts.high, counts.medium, counts.low
    )
    .unwrap();
    writeln!(
        output,
        "║    Total Risk Score: {:<36} ║",
        model.total_risk_score()
    )
    .unwrap();
    writeln!(output, "║                                                          ║").unwrap();

    // Top Threats
    if !model.threats.is_empty() {
        writeln!(output, "║  Top Threats:                                              ║").unwrap();

        let mut sorted_threats: Vec<&Threat> = model.threats.iter().collect();
        sorted_threats.sort_by(|a, b| b.risk_score.cmp(&a.risk_score));

        for threat in sorted_threats.iter().take(3) {
            writeln!(
                output,
                "║    • {} (Risk: {})                              ║",
                truncate(&threat.title, 45),
                threat.risk_score
            )
            .unwrap();
        }

        writeln!(output, "║                                                          ║").unwrap();
    }

    // Footer
    writeln!(
        output,
        "╚══════════════════════════════════════════════════════════════════════╝"
    )
    .unwrap();

    output
}

/// Generate simplified DFD for terminal output
pub fn generate_simple_dfd(model: &ThreatModel) -> String {
    let mut output = String::new();

    writeln!(output).unwrap();
    writeln!(output, "{}", "═".repeat(60)).unwrap();
    writeln!(output, "📊 DATA FLOW DIAGRAM").unwrap();
    writeln!(output, "{}", "─".repeat(60)).unwrap();

    // Simple flow diagram
    writeln!(output).unwrap();
    writeln!(output, "  [External User]").unwrap();
    writeln!(output, "        │").unwrap();
    writeln!(output, "        ▼").unwrap();

    if !model.entry_points.is_empty() {
        writeln!(output, "  ┌─────────────────┐").unwrap();
        writeln!(output, "  │  Entry Points   │").unwrap();
        writeln!(output, "  │  ({})          │", model.entry_points.len()).unwrap();
        writeln!(output, "  └────────┬────────┘").unwrap();
        writeln!(output, "           │").unwrap();
    }

    if !model.trust_boundaries.is_empty() {
        writeln!(output, "  ┌─────────────────┐").unwrap();
        writeln!(output, "  │ Trust Boundaries│").unwrap();
        writeln!(output, "  │  ({})          │", model.trust_boundaries.len()).unwrap();
        writeln!(output, "  └────────┬────────┘").unwrap();
        writeln!(output, "           │").unwrap();
    }

    if !model.data_flows.is_empty() {
        writeln!(output, "  ┌─────────────────┐").unwrap();
        writeln!(output, "  │   Data Flows    │").unwrap();
        writeln!(output, "  │  ({})          │", model.data_flows.len()).unwrap();
        writeln!(output, "  └────────┬────────┘").unwrap();
        writeln!(output, "           │").unwrap();
    }

    writeln!(output, "  ┌─────────────────┐").unwrap();
    writeln!(output, "  │    Threats      │").unwrap();
    writeln!(output, "  │  ({})          │", model.threats.len()).unwrap();
    writeln!(output, "  └─────────────────┘").unwrap();

    writeln!(output).unwrap();
    writeln!(output, "{}", "─".repeat(60)).unwrap();

    // Summary
    let counts = model.threats_by_severity();
    writeln!(
        output,
        "Threats: {} Critical | {} High | {} Medium | {} Low",
        counts.critical, counts.high, counts.medium, counts.low
    )
    .unwrap();
    writeln!(output, "Total Risk Score: {}", model.total_risk_score()).unwrap();
    writeln!(output, "{}", "═".repeat(60)).unwrap();

    output
}

/// Generate component diagram
pub fn generate_component_diagram(model: &ThreatModel) -> String {
    let mut output = String::new();

    writeln!(
        output,
        "╔══════════════════════════════════════════════════════════╗"
    )
    .unwrap();
    writeln!(output, "║  Component Diagram                                    ║").unwrap();
    writeln!(
        output,
        "╠══════════════════════════════════════════════════════════╣"
    )
    .unwrap();
    writeln!(output, "║                                                      ║").unwrap();

    // Group components by type
    let mut components: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    for ep in &model.entry_points {
        components
            .entry("Entry Points".to_string())
            .or_insert_with(Vec::new)
            .push(ep.name.clone());
    }

    for tb in &model.trust_boundaries {
        components
            .entry("Boundaries".to_string())
            .or_insert_with(Vec::new)
            .push(tb.name.clone());
    }

    for flow in &model.data_flows {
        components
            .entry("Destinations".to_string())
            .or_insert_with(Vec::new)
            .push(flow.to.clone());
    }

    // Render components
    for (category, items) in &components {
        writeln!(output, "║  {}:", category.bold()).unwrap();
        for item in items.iter().take(5) {
            writeln!(output, "║    • {}", truncate(item, 50)).unwrap();
        }
        if items.len() > 5 {
            writeln!(output, "║    ... and {} more", items.len() - 5).unwrap();
        }
        writeln!(output, "║                                                      ║").unwrap();
    }

    writeln!(
        output,
        "╚══════════════════════════════════════════════════════════╝"
    )
    .unwrap();

    output
}

/// Truncate string to max length with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len > 3 {
        format!("{}...", &s[..max_len - 3])
    } else {
        s[..max_len].to_string()
    }
}

/// Extension trait for bold formatting (simple version)
trait BoldString {
    fn bold(&self) -> &str;
}

impl BoldString for str {
    fn bold(&self) -> &str {
        self // Simple implementation - could add ANSI codes if needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{DataFlow, EntryPoint, Likelihood, Severity, StrideCategory};
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

        model.data_flows.push(DataFlow {
            from: "Application".to_string(),
            to: "Database".to_string(),
            data_type: "SQL Query".to_string(),
            protocol: Some("TCP".to_string()),
            encrypted: true,
            description: None,
        });

        model.threats.push(Threat {
            id: "THR-001".to_string(),
            title: "Information Disclosure via AWS_ACCESS_KEY".to_string(),
            description: "Test threat".to_string(),
            stride: StrideCategory::InformationDisclosure,
            severity: Severity::Critical,
            likelihood: Likelihood::High,
            impact: crate::model::Impact::Major,
            risk_score: 20,
            affected_component: "config.rs:10".to_string(),
            mitigation: "Rotate key".to_string(),
            cwe_id: None,
            related_finding: None,
            exploited_entry_point: None,
        });

        model
    }

    #[test]
    fn test_dfd_generation() {
        let model = create_test_model();
        let dfd = generate_dfd(&model);

        assert!(dfd.contains("Data Flow Diagram"));
        assert!(dfd.contains("Entry Points"));
        assert!(dfd.contains("GET /api/users"));
        assert!(dfd.contains("Critical"));
    }

    #[test]
    fn test_simple_dfd_generation() {
        let model = create_test_model();
        let dfd = generate_simple_dfd(&model);

        assert!(dfd.contains("DATA FLOW DIAGRAM"));
        assert!(dfd.contains("Entry Points"));
        assert!(dfd.contains("Threats"));
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("this is a long string", 10), "this is...");
        assert_eq!(truncate("exact", 5), "exact");
    }

    #[test]
    fn test_component_diagram() {
        let model = create_test_model();
        let diagram = generate_component_diagram(&model);

        assert!(diagram.contains("Component Diagram"));
        assert!(diagram.contains("Entry Points"));
    }
}
