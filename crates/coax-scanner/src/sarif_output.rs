//! SARIF Output Module
//!
//! This module provides SARIF 2.1.0 format output for Coax scan results.
//! SARIF (Static Analysis Results Interchange Format) is compatible with
//! GitHub Advanced Security and other security tools.

use serde::{Deserialize, Serialize};
use crate::ScanResult;

/// SARIF 2.1.0 output structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifOutput {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

/// A run of the scanner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

/// Tool information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

/// Driver (scanner) information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    #[serde(rename = "informationUri")]
    pub information_uri: String,
    #[serde(rename = "rules", skip_serializing_if = "Vec::is_empty")]
    pub rules: Vec<SarifRule>,
}

/// A rule definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRule {
    pub id: String,
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: SarifMultiformatMessageString,
    #[serde(rename = "helpUri", skip_serializing_if = "Option::is_none")]
    pub help_uri: Option<String>,
    pub properties: SarifRuleProperties,
}

/// Properties for a rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRuleProperties {
    #[serde(rename = "security-severity")]
    pub security_severity: String,
    pub tags: Vec<String>,
}

/// A single result/finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifResult {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub level: String,
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
}

/// Message with text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifMessage {
    pub text: String,
}

/// Multiformat message string
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifMultiformatMessageString {
    pub text: String,
}

/// Location of a finding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifLocation {
    #[serde(rename = "physicalLocation")]
    pub physical_location: SarifPhysicalLocation,
}

/// Physical location details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifPhysicalLocation {
    #[serde(rename = "artifactLocation")]
    pub artifact_location: SarifArtifactLocation,
    #[serde(rename = "region", skip_serializing_if = "Option::is_none")]
    pub region: Option<SarifRegion>,
}

/// Artifact (file) location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifArtifactLocation {
    pub uri: String,
}

/// Region within the artifact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifRegion {
    #[serde(rename = "startLine")]
    pub start_line: u32,
    #[serde(rename = "startColumn", skip_serializing_if = "Option::is_none")]
    pub start_column: Option<u32>,
    #[serde(rename = "endLine", skip_serializing_if = "Option::is_none")]
    pub end_line: Option<u32>,
    #[serde(rename = "endColumn", skip_serializing_if = "Option::is_none")]
    pub end_column: Option<u32>,
    #[serde(rename = "snippet", skip_serializing_if = "Option::is_none")]
    pub snippet: Option<SarifSnippet>,
}

/// Code snippet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SarifSnippet {
    pub text: String,
}

/// Convert severity to SARIF level
fn severity_to_level(severity: &str) -> &'static str {
    match severity.to_lowercase().as_str() {
        "critical" => "error",
        "high" => "error",
        "medium" => "warning",
        "low" => "note",
        _ => "none",
    }
}

/// Convert severity to numeric security severity score
fn severity_to_score(severity: &str) -> &'static str {
    match severity.to_lowercase().as_str() {
        "critical" => "9.0",
        "high" => "7.0",
        "medium" => "4.0",
        "low" => "1.0",
        _ => "0.0",
    }
}

/// Generate SARIF output from scan results
pub fn generate_sarif(results: &[ScanResult], version: &str) -> SarifOutput {
    // Collect unique rules from results
    let mut rules_map: std::collections::HashMap<String, SarifRule> = std::collections::HashMap::new();
    
    for result in results {
        if !rules_map.contains_key(&result.pattern) {
            // Determine if this is a Unicode finding
            let is_unicode = result.pattern.starts_with("UNICODE-");
            
            // Create appropriate tags
            let mut tags = vec!["security".to_string()];
            if is_unicode {
                tags.push("unicode".to_string());
                tags.push("obfuscation".to_string());
            } else {
                tags.push("secrets".to_string());
            }

            // Create help URI - use Glassworm reference for Unicode findings
            let help_uri = if is_unicode {
                match result.pattern.as_str() {
                    "UNICODE-GLASSWORM_PATTERN" => Some("https://www.aikido.dev/blog/glassworm-returns".to_string()),
                    "UNICODE-BIDIRECTIONAL_OVERRIDE" => Some("https://www.w3.org/TR/unicode140/".to_string()),
                    "UNICODE-HOMOGLYPH" => Some("https://www.unicode.org/reports/tr39/".to_string()),
                    "UNICODE-INVISIBLE_CHARACTER" => Some("https://www.unicode.org/charts/PDF/U2000.pdf".to_string()),
                    _ => Some("https://github.com/gl33mer/coax/blob/main/docs/unicode-attacks.md".to_string()),
                }
            } else {
                Some(format!("https://github.com/gl33mer/coax/rules/{}", result.pattern.to_lowercase()))
            };

            let rule = SarifRule {
                id: result.pattern.clone(),
                name: result.pattern.clone(),
                short_description: SarifMultiformatMessageString {
                    text: result.recommendation.clone(),
                },
                help_uri,
                properties: SarifRuleProperties {
                    security_severity: severity_to_score(&result.severity).to_string(),
                    tags,
                },
            };
            rules_map.insert(result.pattern.clone(), rule);
        }
    }
    
    let rules: Vec<SarifRule> = rules_map.into_values().collect();
    
    // Convert results to SARIF results
    let sarif_results: Vec<SarifResult> = results.iter().map(|result| {
        SarifResult {
            rule_id: result.pattern.clone(),
            level: severity_to_level(&result.severity).to_string(),
            message: SarifMessage {
                text: format!("{} detected", result.pattern),
            },
            locations: vec![SarifLocation {
                physical_location: SarifPhysicalLocation {
                    artifact_location: SarifArtifactLocation {
                        uri: result.file.to_string_lossy().to_string(),
                    },
                    region: Some(SarifRegion {
                        start_line: result.line,
                        start_column: result.column,
                        end_line: None,
                        end_column: None,
                        snippet: result.line_content.clone().map(|text| SarifSnippet { text }),
                    }),
                },
            }],
        }
    }).collect();
    
    SarifOutput {
        schema: "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json".to_string(),
        version: "2.1.0".to_string(),
        runs: vec![SarifRun {
            tool: SarifTool {
                driver: SarifDriver {
                    name: "Coax".to_string(),
                    version: version.to_string(),
                    information_uri: "https://github.com/gl33mer/coax".to_string(),
                    rules,
                },
            },
            results: sarif_results,
        }],
    }
}

/// Generate SARIF JSON string from scan results
pub fn generate_sarif_json(results: &[ScanResult], version: &str) -> String {
    let sarif = generate_sarif(results, version);
    serde_json::to_string_pretty(&sarif).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use crate::{ScanResult, FindingContext};
    
    fn create_test_result() -> ScanResult {
        ScanResult {
            file: PathBuf::from("config.yml"),
            line: 45,
            column: Some(1),
            pattern: "AWS_ACCESS_KEY".to_string(),
            severity: "critical".to_string(),
            recommendation: "Remove immediately and rotate the key via AWS IAM Console".to_string(),
            detected_secret: Some("AKIAIOSFODNN7EXAMPLE".to_string()),
            line_content: Some("AWS_KEY=AKIAIOSFODNN7EXAMPLE".to_string()),
            context: FindingContext::default(),
        }
    }
    
    #[test]
    fn test_sarif_generation() {
        let results = vec![create_test_result()];
        let sarif = generate_sarif(&results, "0.3.0");
        
        assert_eq!(sarif.version, "2.1.0");
        assert_eq!(sarif.runs.len(), 1);
        assert_eq!(sarif.runs[0].results.len(), 1);
        
        let result = &sarif.runs[0].results[0];
        assert_eq!(result.rule_id, "AWS_ACCESS_KEY");
        assert_eq!(result.level, "error");
    }
    
    #[test]
    fn test_sarif_json_generation() {
        let results = vec![create_test_result()];
        let json = generate_sarif_json(&results, "0.3.0");
        
        // Verify it's valid JSON
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["version"], "2.1.0");
        assert!(parsed["runs"].is_array());
    }
    
    #[test]
    fn test_severity_mapping() {
        assert_eq!(severity_to_level("critical"), "error");
        assert_eq!(severity_to_level("high"), "error");
        assert_eq!(severity_to_level("medium"), "warning");
        assert_eq!(severity_to_level("low"), "note");
        
        assert_eq!(severity_to_score("critical"), "9.0");
        assert_eq!(severity_to_score("high"), "7.0");
        assert_eq!(severity_to_score("medium"), "4.0");
        assert_eq!(severity_to_score("low"), "1.0");
    }
    
    #[test]
    fn test_sarif_schema_url() {
        let results = vec![create_test_result()];
        let sarif = generate_sarif(&results, "0.3.0");
        
        assert_eq!(
            sarif.schema,
            "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json"
        );
    }
}
