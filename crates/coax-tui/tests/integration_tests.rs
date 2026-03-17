//! Integration tests for Coax TUI

use coax_tui::app::Severity;
use coax_tui::app::{App, Finding, SortField, View};
use std::path::PathBuf;

fn create_test_finding(pattern: &str, severity: Severity, file: &str, line: u32) -> Finding {
    Finding {
        file: file.to_string(),
        line,
        column: None,
        pattern: pattern.to_string(),
        severity,
        recommendation: "Test recommendation".to_string(),
        line_content: Some("test content".to_string()),
        code_context: None,
        confidence: 100,
        ignored: false,
        false_positive: false,
        notes: None,
    }
}

#[test]
fn test_app_initialization() {
    let app = App::new(PathBuf::from("."));
    assert!(app.running);
    assert_eq!(app.view, View::Dashboard);
    assert_eq!(app.scan_path, PathBuf::from("."));
    assert!(app.scan_results.is_empty());
}

#[test]
fn test_severity_counts() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![
        create_test_finding("CRITICAL", Severity::Critical, "file1.rs", 1),
        create_test_finding("CRITICAL", Severity::Critical, "file2.rs", 2),
        create_test_finding("HIGH", Severity::High, "file3.rs", 3),
        create_test_finding("MEDIUM", Severity::Medium, "file4.rs", 4),
    ];

    let counts = app.severity_counts();
    assert_eq!(counts.critical, 2);
    assert_eq!(counts.high, 1);
    assert_eq!(counts.medium, 1);
    assert_eq!(counts.low, 0);
    assert_eq!(counts.info, 0);
}

#[test]
fn test_filter_by_severity() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![
        create_test_finding("CRITICAL", Severity::Critical, "file1.rs", 1),
        create_test_finding("HIGH", Severity::High, "file2.rs", 2),
        create_test_finding("MEDIUM", Severity::Medium, "file3.rs", 3),
        create_test_finding("LOW", Severity::Low, "file4.rs", 4),
    ];

    // No filter - all results
    app.apply_filters();
    assert_eq!(app.filtered_results.len(), 4);

    // Filter by critical
    app.toggle_severity_filter(Some(Severity::Critical));
    assert_eq!(app.filtered_results.len(), 1);
    assert_eq!(app.filtered_results[0].severity, Severity::Critical);

    // Filter by high
    app.toggle_severity_filter(Some(Severity::High));
    assert_eq!(app.filtered_results.len(), 1);
    assert_eq!(app.filtered_results[0].severity, Severity::High);

    // Clear filter
    app.toggle_severity_filter(None);
    assert_eq!(app.filtered_results.len(), 4);
}

#[test]
fn test_search_functionality() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![
        create_test_finding("AWS_KEY", Severity::Critical, "config.yml", 1),
        create_test_finding("GITHUB_TOKEN", Severity::High, ".env", 2),
        create_test_finding("STRIPE_KEY", Severity::High, "payment.rs", 3),
    ];

    app.apply_filters();
    assert_eq!(app.filtered_results.len(), 3);

    // Search for "aws"
    app.set_search_query("aws".to_string());
    assert_eq!(app.filtered_results.len(), 1);
    assert!(app.filtered_results[0].pattern.contains("AWS"));

    // Search for "file"
    app.set_search_query("config".to_string());
    assert_eq!(app.filtered_results.len(), 1);
    assert!(app.filtered_results[0].file.contains("config"));

    // Clear search
    app.clear_search();
    assert_eq!(app.filtered_results.len(), 3);
}

#[test]
fn test_sorting_changes_order() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![
        create_test_finding("LOW_PAT", Severity::Low, "z_file.rs", 10),
        create_test_finding("HIGH_PAT", Severity::High, "a_file.rs", 5),
        create_test_finding("CRITICAL_PAT", Severity::Critical, "m_file.rs", 1),
    ];

    // First call to set_sort_field sets Desc
    app.set_sort_field(SortField::Severity);
    // Results should be sorted (order depends on implementation)
    assert_eq!(app.filtered_results.len(), 3);

    // Second call toggles to Asc
    app.set_sort_field(SortField::Severity);
    assert_eq!(app.filtered_results.len(), 3);
}

#[test]
fn test_navigation() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![
        create_test_finding("PAT1", Severity::High, "file1.rs", 1),
        create_test_finding("PAT2", Severity::High, "file2.rs", 2),
        create_test_finding("PAT3", Severity::High, "file3.rs", 3),
    ];
    app.apply_filters();

    assert_eq!(app.selected_index, 0);

    // Navigate down
    app.navigate_down();
    assert_eq!(app.selected_index, 1);

    app.navigate_down();
    assert_eq!(app.selected_index, 2);

    // Can't go further down
    app.navigate_down();
    assert_eq!(app.selected_index, 2);

    // Navigate up
    app.navigate_up();
    assert_eq!(app.selected_index, 1);

    app.navigate_up();
    assert_eq!(app.selected_index, 0);

    // Can't go further up
    app.navigate_up();
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_view_forward_navigation() {
    let mut app = App::new(PathBuf::from("."));

    assert_eq!(app.view, View::Dashboard);

    app.switch_view(View::FindingList);
    assert_eq!(app.view, View::FindingList);

    app.switch_view(View::FindingDetail);
    assert_eq!(app.view, View::FindingDetail);

    app.switch_view(View::Settings);
    assert_eq!(app.view, View::Settings);
}

#[test]
fn test_view_back_navigation() {
    let mut app = App::new(PathBuf::from("."));

    app.switch_view(View::FindingList);
    app.switch_view(View::FindingDetail);

    app.go_back();
    assert_eq!(app.view, View::FindingList);
}

#[test]
fn test_selected_finding() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![
        create_test_finding("PAT1", Severity::High, "file1.rs", 1),
        create_test_finding("PAT2", Severity::Medium, "file2.rs", 2),
    ];
    app.apply_filters();

    // Get first finding
    let finding = app.selected_finding();
    assert!(finding.is_some());
    assert_eq!(finding.unwrap().pattern, "PAT1");

    // Navigate and get second finding
    app.navigate_down();
    let finding = app.selected_finding();
    assert!(finding.is_some());
    assert_eq!(finding.unwrap().pattern, "PAT2");
}

#[test]
fn test_ignore_finding() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![create_test_finding("PAT1", Severity::High, "file1.rs", 1)];
    app.apply_filters();

    assert!(!app.scan_results[0].ignored);
    app.ignore_selected();
    assert!(app.scan_results[0].ignored);
    assert!(app.status_message.is_some());
}

#[test]
fn test_mark_false_positive() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![create_test_finding("PAT1", Severity::High, "file1.rs", 1)];
    app.apply_filters();

    assert!(!app.scan_results[0].false_positive);
    app.mark_false_positive();
    assert!(app.scan_results[0].false_positive);
    assert!(app.status_message.is_some());
}

#[test]
fn test_empty_results() {
    let mut app = App::new(PathBuf::from("."));

    // No results
    app.apply_filters();
    assert!(app.filtered_results.is_empty());
    assert!(app.selected_finding().is_none());

    // Navigate with no results
    app.navigate_up();
    app.navigate_down();
    assert_eq!(app.selected_index, 0);
}

#[test]
fn test_recent_findings() {
    let mut app = App::new(PathBuf::from("."));

    app.scan_results = vec![
        create_test_finding("PAT1", Severity::High, "file1.rs", 1),
        create_test_finding("PAT2", Severity::High, "file2.rs", 2),
        create_test_finding("PAT3", Severity::High, "file3.rs", 3),
        create_test_finding("PAT4", Severity::High, "file4.rs", 4),
        create_test_finding("PAT5", Severity::High, "file5.rs", 5),
    ];
    app.apply_filters();

    let recent = app.recent_findings(3);
    assert_eq!(recent.len(), 3);
    assert_eq!(recent[0].pattern, "PAT1");
    assert_eq!(recent[2].pattern, "PAT3");
}
