//! CFG-Based Vulnerability Slicing Tests

use coax_scanner::cfg::{
    entry_points, sinks, BackwardSlicer, CFGBuilder, ForwardSlicer, Language, SliceIntersection,
};

#[test]
fn test_cfg_builder_rust_simple() {
    let code = r#"
fn main() {
    let x = 5;
    let y = x + 10;
}
"#;
    let builder = CFGBuilder::new(Language::Rust);
    let cfg = builder.build(code);
    assert!(cfg.is_ok());
}

#[test]
fn test_language_from_extension() {
    assert_eq!(Language::from_extension("rs"), Language::Rust);
    assert_eq!(Language::from_extension("py"), Language::Python);
    assert_eq!(Language::from_extension("js"), Language::JavaScript);
    assert_eq!(Language::from_extension("ts"), Language::TypeScript);
    assert_eq!(Language::from_extension("unknown"), Language::Unknown);
}

#[test]
fn test_sink_sql_detection() {
    let code = r#"
fn get_user(conn: &Connection, id: u32) {
    conn.execute("SELECT * FROM users", [id]);
}
"#;
    let builder = CFGBuilder::new(Language::Rust);
    let cfg = builder.build(code).unwrap();
    let sinks = sinks::detect_all(&cfg);
    let sql_sinks: Vec<_> = sinks
        .iter()
        .filter(|s| matches!(s, coax_scanner::cfg::SinkPoint::SqlExecution { .. }))
        .collect();
    assert!(!sql_sinks.is_empty());
}

#[test]
fn test_sink_command_detection() {
    let code = r#"
fn run_command(cmd: &str) {
    std::process::Command::new("sh").arg(cmd).exec();
}
"#;
    let builder = CFGBuilder::new(Language::Rust);
    let cfg = builder.build(code).unwrap();
    let sinks = sinks::detect_all(&cfg);
    let cmd_sinks: Vec<_> = sinks
        .iter()
        .filter(|s| matches!(s, coax_scanner::cfg::SinkPoint::CommandExecution { .. }))
        .collect();
    assert!(!cmd_sinks.is_empty());
}

#[test]
fn test_backward_slicing() {
    let code = r#"
fn process(input: &str) {
    let query = format!("SELECT * FROM t WHERE x = {}", input);
    db.execute(&query, []);
}
"#;
    let builder = CFGBuilder::new(Language::Rust);
    let cfg = builder.build(code).unwrap();
    let slicer = BackwardSlicer::new(&cfg);
    let slice = slicer.slice(cfg.exit);
    assert!(!slice.nodes.is_empty());
}

#[test]
fn test_forward_slicing() {
    let code = r#"
fn handle(input: &str) -> String {
    let data = parse(input);
    format_output(data)
}
"#;
    let builder = CFGBuilder::new(Language::Rust);
    let cfg = builder.build(code).unwrap();
    let slicer = ForwardSlicer::new(&cfg);
    let slice = slicer.slice(cfg.entry);
    assert!(!slice.nodes.is_empty());
}

#[test]
fn test_confidence_calculation() {
    use coax_scanner::cfg::{EntryPoint, SinkPoint, VulnerabilitySlice};
    use petgraph::graph::NodeIndex;

    let entry = EntryPoint::PublicFunction {
        name: "test".to_string(),
        file: "test.rs".to_string(),
    };
    let sink = SinkPoint::SqlExecution {
        query: "SELECT".to_string(),
        method: "execute".to_string(),
    };
    let mut slice = VulnerabilitySlice::new(entry, sink);
    for i in 0..3 {
        slice.nodes.push(NodeIndex::new(i));
        slice.line_numbers.push(i as u32 + 1);
    }
    let confidence = slice.calculate_confidence();
    assert!(confidence >= 0.0 && confidence <= 1.0);
}
