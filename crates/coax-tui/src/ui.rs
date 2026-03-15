//! Main UI rendering for Coax TUI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, View};
use crate::components;
use crate::views;

/// Render the complete UI
pub fn render(frame: &mut Frame, app: &mut App) {
    let area = frame.area();
    
    // Create main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(20),    // Main content
            Constraint::Length(3),  // Footer
        ])
        .split(area);

    // Render header
    components::render_header(frame, chunks[0], "Coax TUI", crate::VERSION);

    // Render main content based on current view
    render_main_content(frame, app, chunks[1]);

    // Render footer
    components::render_footer(frame, chunks[2], app.status_message.as_deref());

    // Render help popup if visible
    if app.show_help {
        render_help_popup(frame, area);
    }
}

/// Render the main content area based on current view
fn render_main_content(frame: &mut Frame, app: &mut App, area: Rect) {
    match app.view {
        View::Dashboard => views::render_dashboard(frame, app, area),
        View::FindingList => views::render_finding_list(frame, app, area),
        View::FindingDetail => views::render_finding_detail(frame, app, area),
        View::Settings => views::render_settings(frame, app, area),
        View::Help => views::render_dashboard(frame, app, area),
    }
}

/// Render the help popup
fn render_help_popup(frame: &mut Frame, area: Rect) {
    // Calculate popup size (60% width, 70% height)
    let popup_area = centered_rect(60, 70, area);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    // Create popup content
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "🛡️  Coax TUI - Keybindings Help",
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("Global", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  q / Ctrl+C  - Quit application"),
        Line::from("  ?           - Toggle this help"),
        Line::from(""),
        Line::from(Span::styled("Navigation", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  ↑ / k       - Move up"),
        Line::from("  ↓ / j       - Move down"),
        Line::from("  Enter       - Select / View detail"),
        Line::from("  ← / h       - Go back"),
        Line::from(""),
        Line::from(Span::styled("Dashboard View", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  R           - Rescan"),
        Line::from("  L           - Go to list view"),
        Line::from("  S           - Go to settings"),
        Line::from("  /           - Search"),
        Line::from(""),
        Line::from(Span::styled("List View", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  1           - Show all findings"),
        Line::from("  2           - Filter: Critical"),
        Line::from("  3           - Filter: High"),
        Line::from("  4           - Filter: Medium"),
        Line::from("  5           - Filter: Low"),
        Line::from("  s           - Sort by severity"),
        Line::from("  f           - Sort by file"),
        Line::from("  l           - Sort by line"),
        Line::from("  p           - Sort by pattern"),
        Line::from("  D           - Go to dashboard"),
        Line::from(""),
        Line::from(Span::styled("Detail View", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  I           - Ignore finding"),
        Line::from("  F           - Mark as false positive"),
        Line::from("  R           - Show rotate instructions"),
        Line::from("  C           - Clear status"),
        Line::from(""),
        Line::from(Span::styled("Search", Style::default().add_modifier(Modifier::BOLD))),
        Line::from("  /           - Enter search mode"),
        Line::from("  Esc         - Exit search / Clear search"),
        Line::from("  Backspace   - Delete character"),
        Line::from(""),
        Line::from(Span::styled(
            "Press q, Esc, or ? to close this help",
            Style::default().fg(Color::Yellow),
        )),
    ];

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title("Help"),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, popup_area);
}

/// Create a centered rectangle with given percentage dimensions
fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;

    #[test]
    fn test_centered_rect() {
        let area = Rect::new(0, 0, 100, 50);
        let popup = centered_rect(60, 70, area);
        
        assert_eq!(popup.width, 60);
        assert_eq!(popup.height, 35);
        assert_eq!(popup.x, 20);
        assert_eq!(popup.y, 8);
    }
}
