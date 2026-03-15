//! Settings view for Coax TUI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Render the settings view
pub fn render_settings(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Min(15),    // Settings list
            Constraint::Length(3),  // Actions
        ])
        .split(area);

    // Title
    render_settings_title(frame, chunks[0]);

    // Settings list
    render_settings_list(frame, app, chunks[1]);

    // Actions
    render_settings_actions(frame, chunks[2]);
}

fn render_settings_title(frame: &mut Frame, area: Rect) {
    let text = Line::from(vec![
        Span::styled("⚙️ ", Style::default().fg(Color::Blue)),
        Span::styled(" Settings", Style::default().add_modifier(Modifier::BOLD)),
    ]);

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Configuration"));

    frame.render_widget(paragraph, area);
}

fn render_settings_list(frame: &mut Frame, app: &App, area: Rect) {
    let settings = vec![
        ("Scan Path", app.scan_path.display().to_string()),
        ("Sort By", format!("{:?}", app.sort_by)),
        ("Sort Order", format!("{:?}", app.sort_order)),
        ("Filter Severity", app.filter_severity.as_ref().map_or("All".to_string(), |s| format!("{:?}", s))),
        ("Total Findings", app.scan_results.len().to_string()),
        ("Filtered Findings", app.filtered_results.len().to_string()),
    ];

    let mut lines = Vec::new();
    for (key, value) in settings {
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:<20}", key),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(": "),
            Span::raw(value),
        ]));
        lines.push(Line::from("")); // Empty line for spacing
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Current Settings"))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_settings_actions(frame: &mut Frame, area: Rect) {
    let actions = vec![
        Span::raw("["),
        Span::styled("←", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("] Back  |  "),
        Span::raw("Settings are session-only (not persisted)"),
    ];

    let paragraph = Paragraph::new(Line::from(actions))
        .block(Block::default().borders(Borders::ALL).title("Actions"));

    frame.render_widget(paragraph, area);
}
