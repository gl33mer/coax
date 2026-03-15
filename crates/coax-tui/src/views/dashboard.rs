//! Dashboard view for Coax TUI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::App;

/// Render the dashboard view
pub fn render_dashboard(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Repository info
            Constraint::Length(10), // Severity counts
            Constraint::Min(10),    // Recent findings
            Constraint::Length(3),  // Quick actions
        ])
        .split(area);

    // Repository info
    render_repository_info(frame, app, chunks[0]);

    // Severity counts
    render_severity_counts(frame, app, chunks[1]);

    // Recent findings
    render_recent_findings(frame, app, chunks[2]);

    // Quick actions
    render_quick_actions(frame, app, chunks[3]);
}

fn render_repository_info(frame: &mut Frame, app: &App, area: Rect) {
    let scan_time = app
        .last_scan_time
        .map(|t| t.format("%Y-%m-%d %H:%M (%_M min ago)").to_string())
        .unwrap_or_else(|| "Never".to_string());

    let text = vec![
        Line::from(vec![
            Span::styled("🛡️ ", Style::default().fg(Color::Blue)),
            Span::styled(" Coax Security Dashboard", Style::default().add_modifier(Modifier::BOLD)),
        ]),
        Line::from(format!(
            "Repository: {}  |  Last Scan: {}",
            app.scan_path.display(),
            scan_time
        )),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Info"));

    frame.render_widget(paragraph, area);
}

fn render_severity_counts(frame: &mut Frame, app: &App, area: Rect) {
    let counts = app.severity_counts();

    let rows = vec![
        Row::new(vec![
            "Critical".to_string(),
            counts.critical.to_string(),
        ])
        .style(Style::default().fg(Color::Red)),
        Row::new(vec![
            "High".to_string(),
            counts.high.to_string(),
        ])
        .style(Style::default().fg(Color::Yellow)),
        Row::new(vec![
            "Medium".to_string(),
            counts.medium.to_string(),
        ])
        .style(Style::default().fg(Color::Cyan)),
        Row::new(vec![
            "Low".to_string(),
            counts.low.to_string(),
        ])
        .style(Style::default().fg(Color::Green)),
        Row::new(vec![
            "Info".to_string(),
            counts.info.to_string(),
        ])
        .style(Style::default().fg(Color::White)),
    ];

    let table = Table::new(
        rows,
        [Constraint::Percentage(70), Constraint::Percentage(30)],
    )
    .block(Block::default().borders(Borders::ALL).title("Scan Results"))
    .header(
        Row::new(vec!["Severity", "Count"])
            .style(Style::default().add_modifier(Modifier::BOLD))
            .bottom_margin(1),
    );

    frame.render_widget(table, area);
}

fn render_recent_findings(frame: &mut Frame, app: &App, area: Rect) {
    use crate::app::Severity;

    let findings: Vec<ListItem> = app
        .recent_findings(10)
        .iter()
        .map(|f| {
            let icon = match f.severity {
                Severity::Critical => "🚨",
                Severity::High => "⚠️",
                Severity::Medium => "⚡",
                Severity::Low => "ℹ️",
                Severity::Info => "📋",
            };

            let severity_color = match f.severity {
                Severity::Critical => Color::Red,
                Severity::High => Color::Yellow,
                Severity::Medium => Color::Cyan,
                Severity::Low => Color::Green,
                Severity::Info => Color::White,
            };

            let content = format!(
                "{} {}  {}:{}  [Enter to view]",
                icon,
                f.pattern,
                f.file,
                f.line
            );

            ListItem::new(Line::from(vec![
                Span::raw(content),
            ])).style(Style::default().fg(severity_color))
        })
        .collect();

    let list = List::new(findings)
        .block(Block::default().borders(Borders::ALL).title("Recent Findings"));

    frame.render_widget(list, area);
}

fn render_quick_actions(frame: &mut Frame, app: &App, area: Rect) {
    let actions = if app.scan_results.is_empty() {
        vec![
            Span::raw("Press "),
            Span::styled("R", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to scan  |  "),
            Span::styled("L", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" for list view  |  "),
            Span::styled("Q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to quit"),
        ]
    } else {
        vec![
            Span::raw("["),
            Span::styled("↑↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("] Navigate  "),
            Span::raw("["),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("] View  "),
            Span::raw("["),
            Span::styled("/", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("] Search  "),
            Span::raw("["),
            Span::styled("R", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("] Rescan  "),
            Span::raw("["),
            Span::styled("Q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("] Quit"),
        ]
    };

    let paragraph = Paragraph::new(Line::from(actions))
        .block(Block::default().borders(Borders::ALL).title("Quick Actions"))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
