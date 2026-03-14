//! Coax CLI
//!
//! High-performance security scanner for detecting secrets and vulnerabilities.

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use colored::Colorize;
use coax_scanner::{Scanner, ScannerConfig, ScanResult, OutputFormat};
use std::path::PathBuf;
use std::time::Instant;

/// Coax - High-Performance Security Scanner
#[derive(Parser, Debug)]
#[command(name = "coax")]
#[command(author = "Coax Team")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "Coax secrets and vulnerabilities out of your codebases", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Quiet mode (minimal output)
    #[arg(short, long, global = true)]
    quiet: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Scan for secrets and vulnerabilities
    Scan {
        /// Path to scan (file or directory)
        #[arg(short, long, default_value = ".")]
        path: PathBuf,

        /// Output format
        #[arg(short, long, value_enum, default_value = "text")]
        format: OutputFormatArg,

        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Number of threads (0 = auto)
        #[arg(short, long, default_value = "0")]
        threads: usize,

        /// Exclude patterns (comma-separated)
        #[arg(short, long)]
        exclude: Option<String>,

        /// Include line content in results
        #[arg(long)]
        with_content: bool,

        /// Scan hidden files and directories
        #[arg(long)]
        hidden: bool,

        /// Maximum file size to scan (e.g., "10MB", "1GB")
        #[arg(long, default_value = "10MB")]
        max_file_size: String,
    },

    /// Show version information
    Version,
}

#[derive(ValueEnum, Debug, Clone)]
enum OutputFormatArg {
    Text,
    Json,
    Yaml,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(arg: OutputFormatArg) -> Self {
        match arg {
            OutputFormatArg::Text => OutputFormat::Text,
            OutputFormatArg::Json => OutputFormat::Json,
            OutputFormatArg::Yaml => OutputFormat::Yaml,
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize logging
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::DEBUG)
            .init();
    }

    match cli.command {
        Commands::Scan {
            path,
            format,
            output,
            threads,
            exclude,
            with_content,
            hidden,
            max_file_size,
        } => run_scan(
            path,
            format.into(),
            output,
            threads,
            exclude,
            with_content,
            hidden,
            max_file_size,
        ),
        Commands::Version => {
            println!("coax {}", env!("CARGO_PKG_VERSION"));
            Ok(())
        }
    }
}

/// Parse file size string (e.g., "10MB", "1GB") to bytes
fn parse_file_size(size: &str) -> Result<u64> {
    let size = size.trim().to_uppercase();
    let (num, suffix) = size.split_at(size.len() - 2);
    let num: u64 = num.trim().parse()?;

    let multiplier = match suffix.trim() {
        "KB" => 1024,
        "MB" => 1024 * 1024,
        "GB" => 1024 * 1024 * 1024,
        "B" => 1,
        _ => anyhow::bail!("Invalid size suffix: {}. Use B, KB, MB, or GB", suffix),
    };

    Ok(num * multiplier)
}

/// Run the security scan
#[allow(clippy::too_many_arguments)]
fn run_scan(
    path: PathBuf,
    format: OutputFormat,
    output: Option<PathBuf>,
    threads: usize,
    exclude: Option<String>,
    with_content: bool,
    hidden: bool,
    max_file_size: String,
) -> Result<()> {
    // Validate path
    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    // Parse max file size
    let max_size_bytes = parse_file_size(&max_file_size)?;

    // Build configuration
    let mut config = ScannerConfig::default()
        .with_threads(threads)
        .with_max_file_size(max_size_bytes);

    if with_content {
        config = config.with_line_content();
    }

    if hidden {
        config.scan_hidden = true;
    }

    // Add exclude patterns
    if let Some(exclude_str) = exclude {
        for pattern in exclude_str.split(',') {
            config.exclude_patterns.push(pattern.trim().to_string());
        }
    }

    // Create scanner
    let scanner = Scanner::with_config(config);

    if !cli_quiet() {
        eprintln!(
            "{} {} - Scanning {}",
            "🔍".bold().blue(),
            "Coax".bold(),
            path.display().to_string().cyan()
        );
        eprintln!(
            "   {} patterns loaded",
            scanner.pattern_count().to_string().yellow()
        );
        eprintln!();
    }

    // Run scan
    let start = Instant::now();
    let (results, summary) = scanner.scan_with_summary(&path);
    let duration = start.elapsed();

    // Format output
    let output_str = format_results(&results, &summary, format, duration)?;

    // Write output
    if let Some(output_path) = output {
        std::fs::write(&output_path, &output_str)?;
        if !cli_quiet() {
            eprintln!(
                "{} Results written to {}",
                "✓".bold().green(),
                output_path.display().to_string().cyan()
            );
        }
    } else {
        println!("{}", output_str);
    }

    // Print summary to stderr if not quiet
    if !cli_quiet() && format != OutputFormat::Json && format != OutputFormat::Yaml {
        eprintln!();
        eprintln!("{}", "─".repeat(60).dimmed());
        eprintln!(
            "{} {} files in {:?}",
            "📊".bold().blue(),
            summary.files_scanned.to_string().yellow(),
            duration
        );
        eprintln!(
            "{} {} findings ({} critical, {} high, {} medium, {} low)",
            "🔍".bold().blue(),
            summary.total_findings.to_string().red().bold(),
            summary.by_severity.critical.to_string().red(),
            summary.by_severity.high.to_string().yellow(),
            summary.by_severity.medium.to_string().blue(),
            summary.by_severity.low.to_string().white()
        );

        if !summary.top_patterns.is_empty() {
            eprintln!();
            eprintln!("{}", "Top Patterns:".bold());
            for (i, pattern) in summary.top_patterns.iter().take(5).enumerate() {
                eprintln!(
                    "   {}. {}: {} {}",
                    i + 1,
                    pattern.pattern.yellow(),
                    pattern.count.to_string().cyan(),
                    "finding(s)".dimmed()
                );
            }
        }

        eprintln!("{}", "─".repeat(60).dimmed());
    }

    // Exit with error code if critical/high findings
    if summary.by_severity.critical > 0 || summary.by_severity.high > 0 {
        std::process::exit(1);
    }

    Ok(())
}

/// Format results based on output format
fn format_results(
    results: &[ScanResult],
    summary: &coax_scanner::ScanSummary,
    format: OutputFormat,
    duration: std::time::Duration,
) -> Result<String> {
    match format {
        OutputFormat::Text => Ok(format_text(results, summary, duration)),
        OutputFormat::Json => Ok(serde_json::to_string_pretty(&serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "scan_duration_ms": duration.as_millis() as u64,
            "summary": summary,
            "findings": results
        }))?),
        OutputFormat::Yaml => Ok(serde_yaml::to_string(&serde_yaml::Value::Mapping(
            serde_yaml::Mapping::from_iter([
                (
                    serde_yaml::Value::String("version".to_string()),
                    serde_yaml::Value::String(env!("CARGO_PKG_VERSION").to_string()),
                ),
                (
                    serde_yaml::Value::String("scan_duration_ms".to_string()),
                    serde_yaml::Value::Number((duration.as_millis() as u64).into()),
                ),
                (
                    serde_yaml::Value::String("summary".to_string()),
                    serde_yaml::to_value(summary)?,
                ),
                (
                    serde_yaml::Value::String("findings".to_string()),
                    serde_yaml::to_value(results)?,
                ),
            ]),
        ))?),
        OutputFormat::Sarif => Ok(format_sarif(results)),
    }
}

/// Format results as text
fn format_text(
    results: &[ScanResult],
    _summary: &coax_scanner::ScanSummary,
    _duration: std::time::Duration,
) -> String {
    if results.is_empty() {
        return format!(
            "{}\n{}\n",
            "✓ No secrets or vulnerabilities detected".green().bold(),
            "Your code looks clean!".dimmed()
        );
    }

    let mut output = String::new();

    // Sort by severity (critical first)
    let mut sorted_results = results.to_vec();
    sorted_results.sort_by(|a, b| b.severity_score().cmp(&a.severity_score()));

    for result in &sorted_results {
        let severity_color = match result.severity.to_lowercase().as_str() {
            "critical" => |s: &str| s.red().bold(),
            "high" => |s: &str| s.yellow().bold(),
            "medium" => |s: &str| s.blue().bold(),
            "low" => |s: &str| s.white().bold(),
            _ => |s: &str| s.normal(),
        };

        let icon = match result.severity.to_lowercase().as_str() {
            "critical" => "🚨",
            "high" => "⚠️",
            "medium" => "⚡",
            "low" => "ℹ️",
            _ => "•",
        };

        output.push_str(&format!(
            "{} {}:{}:{} - {} [{}]\n",
            icon,
            result.file.display().to_string().cyan(),
            result.line.to_string().yellow(),
            result.column.map(|c| c.to_string()).unwrap_or_default(),
            result.pattern.yellow(),
            severity_color(&result.severity.to_uppercase()),
        ));

        output.push_str(&format!(
            "   {}\n",
            result.recommendation.dimmed()
        ));

        if let Some(content) = &result.line_content {
            output.push_str(&format!("   {}\n", content.dimmed()));
        }

        output.push('\n');
    }

    output
}

/// Format results as SARIF
fn format_sarif(results: &[ScanResult]) -> String {
    let sarif = serde_json::json!({
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "Coax",
                    "version": env!("CARGO_PKG_VERSION"),
                    "informationUri": "https://github.com/gl33mer/coax",
                    "rules": results.iter()
                        .map(|r| {
                            serde_json::json!({
                                "id": r.pattern,
                                "name": r.pattern,
                                "shortDescription": {
                                    "text": r.recommendation
                                },
                                "defaultConfiguration": {
                                    "level": match r.severity.to_lowercase().as_str() {
                                        "critical" => "error",
                                        "high" => "error",
                                        "medium" => "warning",
                                        "low" => "note",
                                        _ => "none"
                                    }
                                }
                            })
                        })
                        .collect::<Vec<_>>()
                }
            },
            "results": results.iter()
                .map(|r| {
                    serde_json::json!({
                        "ruleId": r.pattern,
                        "level": match r.severity.to_lowercase().as_str() {
                            "critical" | "high" => "error",
                            "medium" => "warning",
                            "low" => "note",
                            _ => "none"
                        },
                        "message": {
                            "text": r.recommendation
                        },
                        "locations": [{
                            "physicalLocation": {
                                "artifactLocation": {
                                    "uri": r.file.to_string_lossy()
                                },
                                "region": {
                                    "startLine": r.line
                                }
                            }
                        }]
                    })
                })
                .collect::<Vec<_>>()
        }]
    });

    serde_json::to_string_pretty(&sarif).unwrap_or_default()
}

/// Check if CLI should be quiet
fn cli_quiet() -> bool {
    // This would need to be passed from main, but for now we'll check env var
    std::env::var("COAX_QUIET").is_ok()
}
