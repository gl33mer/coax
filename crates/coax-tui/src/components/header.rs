//! Header component for Coax TUI

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// Render the application header
pub fn render_header(frame: &mut Frame, area: Rect, title: &str, version: &str) {
    let text = Line::from(vec![
        Span::styled("🛡️ ", Style::default().fg(Color::Blue)),
        Span::styled(
            " Coax Security Scanner",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" v{}", version)),
    ]);

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Blue))
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            )),
    );

    frame.render_widget(paragraph, area);
}
