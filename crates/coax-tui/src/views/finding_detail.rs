//! Finding detail view for Coax TUI

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Render the finding detail view
pub fn render_finding_detail(frame: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),  // Title
            Constraint::Length(6),  // Finding info
            Constraint::Min(8),     // Code preview
            Constraint::Length(4),  // Recommendation
            Constraint::Length(3),  // Actions
        ])
        .split(area);

    // Get the selected finding
    let finding = match app.selected_finding() {
        Some(f) => f,
        None => {
            let paragraph = Paragraph::new("No finding selected")
                .block(Block::default().borders(Borders::ALL))
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(paragraph, area);
            return;
        }
    };

    // Title
    render_detail_title(frame, finding, chunks[0]);

    // Finding info
    render_finding_info(frame, finding, chunks[1]);

    // Code preview
    render_code_preview(frame, finding, chunks[2]);

    // Recommendation
    render_recommendation(frame, finding, chunks[3]);

    // Actions
    render_detail_actions(frame, app, chunks[4]);
}

fn render_detail_title(frame: &mut Frame, finding: &crate::app::Finding, area: Rect) {
    use crate::app::Severity;

    let severity_color = match finding.severity {
        Severity::Critical => Color::Red,
        Severity::High => Color::Yellow,
        Severity::Medium => Color::Cyan,
        Severity::Low => Color::Green,
        Severity::Info => Color::White,
    };

    let icon = match finding.severity {
        Severity::Critical => "🚨",
        Severity::High => "⚠️",
        Severity::Medium => "⚡",
        Severity::Low => "ℹ️",
        Severity::Info => "📋",
    };

    let text = Line::from(vec![
        Span::raw(icon),
        Span::raw(" "),
        Span::styled(
            &finding.pattern,
            Style::default()
                .fg(severity_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" ({:?})", finding.severity)),
    ]);

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Finding"));

    frame.render_widget(paragraph, area);
}

fn render_finding_info(frame: &mut Frame, finding: &crate::app::Finding, area: Rect) {
    let location = if let Some(col) = finding.column {
        format!("{}:{}:{}", finding.file, finding.line, col)
    } else {
        format!("{}:{}", finding.file, finding.line)
    };

    let confidence = if finding.false_positive {
        "Marked as False Positive".to_string()
    } else if finding.ignored {
        "Ignored".to_string()
    } else {
        format!("{}%", finding.confidence)
    };

    let lines = vec![
        Line::from(vec![
            Span::styled("Location: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(location),
        ]),
        Line::from(vec![
            Span::styled("Confidence: ", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(confidence),
        ]),
    ];

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Details"));

    frame.render_widget(paragraph, area);
}

fn render_code_preview(frame: &mut Frame, finding: &crate::app::Finding, area: Rect) {
    let mut lines = Vec::new();

    // Add code context if available
    if let Some(context) = &finding.code_context {
        for ctx_line in &context.before {
            lines.push(Line::from(format!("{:4}: {}", ctx_line.number, ctx_line.content)));
        }
        
        // Highlight the finding line
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:4}: ", finding.line),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                finding.line_content.as_deref().unwrap_or(""),
                Style::default().bg(Color::DarkGray).fg(Color::Yellow),
            ),
        ]));
        
        for ctx_line in &context.after {
            lines.push(Line::from(format!("{:4}: {}", ctx_line.number, ctx_line.content)));
        }
    } else if let Some(content) = &finding.line_content {
        // Just show the line content
        lines.push(Line::from(vec![
            Span::styled(
                format!("{:4}: ", finding.line),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(content),
        ]));
    } else {
        lines.push(Line::from("No code preview available"));
    }

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Code Preview"))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn render_recommendation(frame: &mut Frame, finding: &crate::app::Finding, area: Rect) {
    let text = if finding.false_positive {
        "This finding has been marked as a false positive.".to_string()
    } else if finding.ignored {
        "This finding has been ignored.".to_string()
    } else {
        finding.recommendation.clone()
    };

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Recommendation"))
        .wrap(Wrap { trim: true })
        .style(if finding.false_positive || finding.ignored {
            Style::default().fg(Color::Gray)
        } else {
            Style::default()
        });

    frame.render_widget(paragraph, area);
}

fn render_detail_actions(frame: &mut Frame, app: &App, area: Rect) {
    let finding = app.selected_finding();
    
    let actions = if let Some(f) = finding {
        if f.false_positive || f.ignored {
            vec![
                Span::raw("["),
                Span::styled("←", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("] Back  |  "),
                Span::styled("C", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" Clear Status"),
            ]
        } else {
            vec![
                Span::raw("["),
                Span::styled("←", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw("] Back  |  "),
                Span::styled("I", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" Ignore  |  "),
                Span::styled("F", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" False Positive  |  "),
                Span::styled("R", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" Rotate"),
            ]
        }
    } else {
        vec![
            Span::raw("["),
            Span::styled("←", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw("] Back"),
        ]
    };

    let paragraph = Paragraph::new(Line::from(actions))
        .block(Block::default().borders(Borders::ALL).title("Actions"));

    frame.render_widget(paragraph, area);
}
