//! Footer component for Coax TUI

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// Render the application footer with keybindings
pub fn render_footer(frame: &mut Frame, area: Rect, status_message: Option<&str>) {
    let content = if let Some(msg) = status_message {
        Line::from(vec![
            Span::styled("ℹ️ ", Style::default().fg(Color::Blue)),
            Span::raw(msg.to_string()),
        ])
    } else {
        Line::from(vec![
            Span::raw("["),
            Span::styled("?", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("] Help  |  "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Quit"),
        ])
    };

    let paragraph = Paragraph::new(content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Blue))
                .title("Status"),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Render a simple footer with keybindings for a specific view
pub fn render_view_footer(frame: &mut Frame, area: Rect, keys: &[(&str, &str)]) {
    let mut spans = Vec::new();

    for (i, (key, desc)) in keys.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  |  "));
        }
        spans.push(Span::raw("["));
        spans.push(Span::styled(
            *key,
            Style::default().add_modifier(Modifier::BOLD),
        ));
        spans.push(Span::raw(format!("] {}", desc)));
    }

    let paragraph = Paragraph::new(Line::from(spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
                .title("Keybindings"),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
