//! Finding list view for Coax TUI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Row, Table, Wrap},
    Frame,
};

use crate::app::{App, SortField, SortOrder};

/// Render the finding list view
pub fn render_finding_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3), // Title and filter info
            Constraint::Length(3), // Filter tabs
            Constraint::Length(3), // Sort info
            Constraint::Min(10),   // Finding list
            Constraint::Length(3), // Status bar
        ])
        .split(area);

    // Title
    render_list_title(frame, app, chunks[0]);

    // Filter tabs
    render_filter_tabs(frame, app, chunks[1]);

    // Sort info
    render_sort_info(frame, app, chunks[2]);

    // Finding list
    render_finding_table(frame, app, chunks[3]);

    // Status bar
    render_list_status(frame, chunks[4]);
}

fn render_list_title(frame: &mut Frame, app: &App, area: Rect) {
    let total = app.scan_results.len();
    let filtered = app.filtered_results.len();

    let search_info = if !app.search_query.is_empty() {
        format!(" (searching: \"{}\")", app.search_query)
    } else {
        String::new()
    };

    let text = Line::from(vec![
        Span::styled("📋 ", Style::default().fg(Color::Blue)),
        Span::styled(
            " Finding List",
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            " - {} findings{} (showing {})",
            total, search_info, filtered
        )),
    ]);

    let paragraph =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Findings"));

    frame.render_widget(paragraph, area);
}

fn render_filter_tabs(frame: &mut Frame, app: &App, area: Rect) {
    use crate::app::Severity;

    let current_filter = app.filter_severity.as_ref();

    let tabs = vec![
        ("All", None),
        ("Critical", Some(Severity::Critical)),
        ("High", Some(Severity::High)),
        ("Medium", Some(Severity::Medium)),
        ("Low", Some(Severity::Low)),
    ];

    let mut spans = Vec::new();
    for (i, (label, severity)) in tabs.iter().enumerate() {
        let is_selected = current_filter == severity.as_ref();

        if is_selected {
            spans.push(Span::styled(
                format!(" [{}] ", label),
                Style::default()
                    .bg(Color::White)
                    .fg(Color::Black)
                    .add_modifier(Modifier::BOLD),
            ));
        } else {
            let key = match i {
                0 => "1",
                1 => "2",
                2 => "3",
                3 => "4",
                4 => "5",
                _ => " ",
            };
            spans.push(Span::raw(format!(" [{}]{} ", key, label)));
        }

        if i < tabs.len() - 1 {
            spans.push(Span::raw("|"));
        }
    }

    let paragraph = Paragraph::new(Line::from(spans))
        .block(Block::default().borders(Borders::ALL).title("Filter (1-5)"))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_sort_info(frame: &mut Frame, app: &App, area: Rect) {
    let order_symbol = match app.sort_order {
        SortOrder::Asc => "↑",
        SortOrder::Desc => "↓",
    };

    let field_name = match app.sort_by {
        SortField::Severity => "Severity",
        SortField::File => "File",
        SortField::Line => "Line",
        SortField::Pattern => "Pattern",
    };

    let text = Line::from(vec![
        Span::raw("Sort by: "),
        Span::styled(
            format!("{} {}", field_name, order_symbol),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  |  "),
        Span::raw("Keys: "),
        Span::styled("s", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("=Severity  "),
        Span::styled("f", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("=File  "),
        Span::styled("l", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("=Line  "),
        Span::styled("p", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw("=Pattern"),
    ]);

    let paragraph =
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Sorting"));

    frame.render_widget(paragraph, area);
}

fn render_finding_table(frame: &mut Frame, app: &App, area: Rect) {
    use crate::app::Severity;

    if app.filtered_results.is_empty() {
        let text = if app.search_query.is_empty() {
            "No findings match the current filter.\nPress '1' to show all findings."
        } else {
            "No findings match your search.\nPress Esc to clear search."
        };

        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL))
            .wrap(Wrap { trim: false })
            .style(Style::default().fg(Color::Yellow));

        frame.render_widget(paragraph, area);
        return;
    }

    let rows: Vec<Row> = app
        .filtered_results
        .iter()
        .enumerate()
        .map(|(i, f)| {
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

            let style = if i == app.selected_index {
                Style::default()
                    .bg(Color::DarkGray)
                    .fg(severity_color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(severity_color)
            };

            Row::new(vec![
                if i == app.selected_index {
                    "▶".to_string()
                } else {
                    " ".to_string()
                },
                format!("{} {}", icon, f.pattern),
                f.file.clone(),
                f.line.to_string(),
            ])
            .style(style)
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(2),
            Constraint::Percentage(35),
            Constraint::Percentage(45),
            Constraint::Length(8),
        ],
    )
    .block(Block::default().borders(Borders::ALL).title(format!(
        "Findings ({}/{})",
        app.selected_index + 1,
        app.filtered_results.len()
    )))
    .header(
        Row::new(vec!["", "Pattern", "File", "Line"])
            .style(Style::default().add_modifier(Modifier::BOLD))
            .bottom_margin(1),
    );

    frame.render_widget(table, area);
}

fn render_list_status(_frame: &mut Frame, _area: Rect) {
    // Status bar rendering removed - handled in footer
}
