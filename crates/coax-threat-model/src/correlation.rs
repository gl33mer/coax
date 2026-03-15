//! Finding-to-Threat Correlation Module
//!
//! This module provides correlation between security findings and threat models,
//! enabling enhanced threat analysis based on scan results.

use crate::model::{Asset, AssetKind, EntryPoint, Impact, Likelihood, Sensitivity, Severity, StrideCategory, Threat, ThreatModel};
use crate::stride::{
    calculate_risk_score, categorize_finding_stride, determine_impact, determine_likelihood,
};
use coax_scanner::ScanResult;

/// Correlate findings with entry points to generate enhanced threats
pub fn correlate_findings_with_threats(
    findings: &[ScanResult],
    entry_points: &[EntryPoint],
) -> Vec<Threat> {
    let mut threats = Vec::new();

    for finding in findings {
        // Get STRIDE categories for this finding
        let stride_categories = categorize_finding_stride(&finding.pattern);

        if stride_categories.is_empty() {
            continue;
        }

        // Find related entry points
        let related_entry_points = find_related_entry_points(finding, entry_points);

        // Generate threats for each STRIDE category
        for stride in stride_categories {
            let likelihood = calculate_likelihood(finding, &related_entry_points);
            let impact = calculate_impact(finding);
            let risk_score = calculate_risk_score(likelihood, impact);

            let exploited_ep = related_entry_points
                .first()
                .map(|ep| ep.name.clone());

            threats.push(Threat {
                id: format!("THR-{:03}", threats.len() + 1),
                title: format!("{} via {}", stride, finding.pattern),
                description: format!(
                    "Secret {} exposed at {}:{} could lead to {} attack. {}",
                    finding.pattern,
                    finding.file.display(),
                    finding.line,
                    stride,
                    stride_description(stride)
                ),
                stride,
                severity: severity_from_str(finding.severity.as_str()),
                likelihood,
                impact,
                risk_score,
                affected_component: format!("{}:{}", finding.file.display(), finding.line),
                mitigation: finding.recommendation.clone(),
                cwe_id: None,
                related_finding: Some(finding.pattern.clone()),
                exploited_entry_point: exploited_ep,
            });
        }
    }

    threats
}

/// Find entry points related to a finding
pub fn find_related_entry_points<'a>(
    finding: &'a ScanResult,
    entry_points: &'a [EntryPoint],
) -> Vec<&'a EntryPoint> {
    let finding_path = finding.file.to_string_lossy();

    entry_points
        .iter()
        .filter(|ep| {
            // Check if file paths match or are related
            ep.path.contains(finding_path.as_ref()) || finding_path.as_ref().contains(&ep.path)
        })
        .collect()
}

/// Calculate likelihood based on finding and entry points
pub fn calculate_likelihood(finding: &ScanResult, entry_points: &[&EntryPoint]) -> Likelihood {
    let is_exposed = !entry_points.is_empty();
    determine_likelihood(finding, is_exposed)
}

/// Calculate impact based on finding
pub fn calculate_impact(finding: &ScanResult) -> Impact {
    determine_impact(&finding.pattern)
}

/// Get STRIDE description
fn stride_description(category: StrideCategory) -> &'static str {
    match category {
        StrideCategory::Spoofing => "Attacker can impersonate a legitimate entity",
        StrideCategory::Tampering => "Attacker can modify data or code",
        StrideCategory::Repudiation => "Attacker can deny performing an action",
        StrideCategory::InformationDisclosure => "Attacker can access sensitive information",
        StrideCategory::DenialOfService => "Attacker can disrupt service availability",
        StrideCategory::ElevationOfPrivilege => "Attacker can gain unauthorized access",
    }
}

/// Convert severity string to enum
fn severity_from_str(severity: &str) -> Severity {
    match severity.to_lowercase().as_str() {
        "critical" => Severity::Critical,
        "high" => Severity::High,
        "medium" => Severity::Medium,
        _ => Severity::Low,
    }
}

/// Enhance a threat model with scan findings
pub fn enhance_threat_model(model: &mut ThreatModel, findings: &[ScanResult]) {
    // Generate threats from findings
    let new_threats = correlate_findings_with_threats(&findings, &model.entry_points);

    // Add new threats to model
    for threat in new_threats {
        // Check for duplicates
        if !model.threats.iter().any(|t| {
            t.related_finding == threat.related_finding
                && t.affected_component == threat.affected_component
        }) {
            model.threats.push(threat);
        }
    }

    // Add assets from findings
    for finding in findings {
        let (kind, sensitivity) = classify_asset_from_pattern(&finding.pattern);

        let asset = Asset {
            name: finding.pattern.clone(),
            kind,
            description: format!("Detected at {}:{}", finding.file.display(), finding.line),
            sensitivity,
            location: Some(format!("{}:{}", finding.file.display(), finding.line)),
        };

        // Avoid duplicates
        if !model.assets.iter().any(|a| {
            a.name == asset.name && a.location == asset.location
        }) {
            model.assets.push(asset);
        }
    }
}

/// Classify asset from pattern
fn classify_asset_from_pattern(pattern: &str) -> (AssetKind, Sensitivity) {
    match pattern.to_uppercase().as_str() {
        p if p.contains("PASSWORD") => (AssetKind::Secret, Sensitivity::Restricted),
        p if p.contains("PRIVATE_KEY") => (AssetKind::PrivateKey, Sensitivity::Restricted),
        p if p.contains("AWS") => (AssetKind::ApiKey, Sensitivity::Restricted),
        p if p.contains("DATABASE") => (AssetKind::Database, Sensitivity::Confidential),
        p if p.contains("TOKEN") || p.contains("KEY") => {
            (AssetKind::ApiKey, Sensitivity::Confidential)
        }
        p if p.contains("SECRET") => (AssetKind::Secret, Sensitivity::Confidential),
        _ => (AssetKind::Secret, Sensitivity::Internal),
    }
}

/// Generate risk heatmap data
pub struct RiskHeatmap {
    pub critical_high: u32,
    pub critical_medium: u32,
    pub critical_low: u32,
    pub high_high: u32,
    pub high_medium: u32,
    pub high_low: u32,
    pub medium_high: u32,
    pub medium_medium: u32,
    pub medium_low: u32,
    pub low_high: u32,
    pub low_medium: u32,
    pub low_low: u32,
}

impl RiskHeatmap {
    /// Generate heatmap from threats
    pub fn from_threats(threats: &[Threat]) -> Self {
        let mut heatmap = Self {
            critical_high: 0,
            critical_medium: 0,
            critical_low: 0,
            high_high: 0,
            high_medium: 0,
            high_low: 0,
            medium_high: 0,
            medium_medium: 0,
            medium_low: 0,
            low_high: 0,
            low_medium: 0,
            low_low: 0,
        };

        for threat in threats {
            match (threat.severity, threat.likelihood) {
                (Severity::Critical, Likelihood::High) => heatmap.critical_high += 1,
                (Severity::Critical, Likelihood::Medium) => heatmap.critical_medium += 1,
                (Severity::Critical, Likelihood::Low) => heatmap.critical_low += 1,
                (Severity::High, Likelihood::High) => heatmap.high_high += 1,
                (Severity::High, Likelihood::Medium) => heatmap.high_medium += 1,
                (Severity::High, Likelihood::Low) => heatmap.high_low += 1,
                (Severity::Medium, Likelihood::High) => heatmap.medium_high += 1,
                (Severity::Medium, Likelihood::Medium) => heatmap.medium_medium += 1,
                (Severity::Medium, Likelihood::Low) => heatmap.medium_low += 1,
                (Severity::Low, Likelihood::High) => heatmap.low_high += 1,
                (Severity::Low, Likelihood::Medium) => heatmap.low_medium += 1,
                (Severity::Low, Likelihood::Low) => heatmap.low_low += 1,
                _ => {}
            }
        }

        heatmap
    }
}

/// Get priority recommendations based on threats
pub fn get_priority_recommendations(threats: &[Threat]) -> Vec<String> {
    let mut recommendations = Vec::new();

    // Group by STRIDE category
    let mut stride_counts: std::collections::HashMap<StrideCategory, u32> =
        std::collections::HashMap::new();

    for threat in threats {
        *stride_counts.entry(threat.stride).or_insert(0) += 1;
    }

    // Generate recommendations based on most common STRIDE categories
    if let Some(count) = stride_counts.get(&StrideCategory::InformationDisclosure) {
        if *count > 0 {
            recommendations.push(format!(
                "🔒 Address {} Information Disclosure risks: Remove hardcoded secrets and use secret management",
                count
            ));
        }
    }

    if let Some(count) = stride_counts.get(&StrideCategory::Spoofing) {
        if *count > 0 {
            recommendations.push(format!(
                "🔐 Address {} Spoofing risks: Implement strong authentication and rotate credentials",
                count
            ));
        }
    }

    if let Some(count) = stride_counts.get(&StrideCategory::ElevationOfPrivilege) {
        if *count > 0 {
            recommendations.push(format!(
                "⚠️ Address {} Elevation of Privilege risks: Apply principle of least privilege",
                count
            ));
        }
    }

    if let Some(count) = stride_counts.get(&StrideCategory::Tampering) {
        if *count > 0 {
            recommendations.push(format!(
                "🛡️ Address {} Tampering risks: Implement integrity checks and access controls",
                count
            ));
        }
    }

    recommendations
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use coax_scanner::FindingContext;

    #[test]
    fn test_correlate_findings() {
        let findings = vec![ScanResult {
            file: PathBuf::from("config.rs"),
            line: 10,
            column: Some(5),
            pattern: "AWS_ACCESS_KEY".to_string(),
            severity: "critical".to_string(),
            recommendation: "Rotate immediately".to_string(),
            detected_secret: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
            line_content: None,
            context: FindingContext::default(),
        }];

        let entry_points = vec![];
        let threats = correlate_findings_with_threats(&findings, &entry_points);

        assert!(!threats.is_empty());
        assert!(threats[0].id.starts_with("THR-"));
        assert!(threats[0].risk_score > 0);
    }

    #[test]
    fn test_find_related_entry_points() {
        let finding = ScanResult {
            file: PathBuf::from("src/config.rs"),
            line: 10,
            column: Some(5),
            pattern: "AWS_ACCESS_KEY".to_string(),
            severity: "critical".to_string(),
            recommendation: "Rotate".to_string(),
            detected_secret: None,
            line_content: None,
            context: FindingContext::default(),
        };

        let entry_points = vec![
            EntryPoint {
                name: "GET /api/users".to_string(),
                kind: crate::model::EntryPointKind::Http,
                path: "src/config.rs".to_string(),
                method: Some("GET".to_string()),
                authentication: true,
                inputs: vec![],
                line: Some(5),
            },
            EntryPoint {
                name: "POST /api/login".to_string(),
                kind: crate::model::EntryPointKind::Http,
                path: "src/routes.rs".to_string(),
                method: Some("POST".to_string()),
                authentication: false,
                inputs: vec![],
                line: Some(20),
            },
        ];

        let related = find_related_entry_points(&finding, &entry_points);
        assert_eq!(related.len(), 1);
        assert!(related[0].path.contains("config.rs"));
    }

    #[test]
    fn test_risk_heatmap() {
        let threats = vec![
            Threat {
                id: "THR-001".to_string(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                stride: StrideCategory::Spoofing,
                severity: Severity::Critical,
                likelihood: Likelihood::High,
                impact: Impact::Major,
                risk_score: 20,
                affected_component: "test.rs".to_string(),
                mitigation: "Fix".to_string(),
                cwe_id: None,
                related_finding: None,
                exploited_entry_point: None,
            },
            Threat {
                id: "THR-002".to_string(),
                title: "Test".to_string(),
                description: "Test".to_string(),
                stride: StrideCategory::Tampering,
                severity: Severity::High,
                likelihood: Likelihood::Medium,
                impact: Impact::Moderate,
                risk_score: 6,
                affected_component: "test.rs".to_string(),
                mitigation: "Fix".to_string(),
                cwe_id: None,
                related_finding: None,
                exploited_entry_point: None,
            },
        ];

        let heatmap = RiskHeatmap::from_threats(&threats);
        assert_eq!(heatmap.critical_high, 1);
        assert_eq!(heatmap.high_medium, 1);
    }

    #[test]
    fn test_priority_recommendations() {
        let threats = vec![Threat {
            id: "THR-001".to_string(),
            title: "Test".to_string(),
            description: "Test".to_string(),
            stride: StrideCategory::InformationDisclosure,
            severity: Severity::Critical,
            likelihood: Likelihood::High,
            impact: Impact::Major,
            risk_score: 20,
            affected_component: "test.rs".to_string(),
            mitigation: "Fix".to_string(),
            cwe_id: None,
            related_finding: None,
            exploited_entry_point: None,
        }];

        let recommendations = get_priority_recommendations(&threats);
        assert!(!recommendations.is_empty());
        assert!(recommendations[0].contains("Information Disclosure"));
    }
}
