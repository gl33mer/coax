//! Keyboard event handler for Coax TUI

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::crossterm::event;
use std::time::Duration;

use crate::app::{App, SortField, View};

/// Result of handling an event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventResult {
    /// Event was handled, continue running
    Handled,
    /// Event was not handled
    NotHandled,
    /// Quit the application
    Quit,
}

/// Handle keyboard events
pub fn handle_key_event(app: &mut App, key: KeyEvent) -> EventResult {
    // Handle search mode first
    if app.search_mode {
        return handle_search_input(app, key);
    }

    // Handle help view
    if app.show_help {
        if matches!(
            key.code,
            KeyCode::Char('q') | KeyCode::Esc | KeyCode::Char('?')
        ) {
            app.show_help = false;
        }
        return EventResult::Handled;
    }

    // Global keybindings
    match key.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => {
            return EventResult::Quit;
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            return EventResult::Quit;
        }
        KeyCode::Char('?') => {
            app.show_help = true;
            return EventResult::Handled;
        }
        _ => {}
    }

    // View-specific keybindings
    match app.view {
        View::Dashboard => handle_dashboard_keys(app, key),
        View::FindingList => handle_list_keys(app, key),
        View::FindingDetail => handle_detail_keys(app, key),
        View::Settings => handle_settings_keys(app, key),
        View::Help => EventResult::Handled,
    }
}

/// Handle search input mode
fn handle_search_input(app: &mut App, key: KeyEvent) -> EventResult {
    match key.code {
        KeyCode::Enter | KeyCode::Esc => {
            app.search_mode = false;
            if app.search_query.is_empty() {
                app.clear_search();
            }
            EventResult::Handled
        }
        KeyCode::Backspace => {
            app.search_query.pop();
            app.set_search_query(app.search_query.clone());
            EventResult::Handled
        }
        KeyCode::Char(c) => {
            app.search_query.push(c);
            app.set_search_query(app.search_query.clone());
            EventResult::Handled
        }
        _ => EventResult::NotHandled,
    }
}

/// Handle keybindings for dashboard view
fn handle_dashboard_keys(app: &mut App, key: KeyEvent) -> EventResult {
    match key.code {
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => {
            app.navigate_up();
            EventResult::Handled
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.navigate_down();
            EventResult::Handled
        }
        KeyCode::Enter => {
            if !app.filtered_results.is_empty() {
                app.switch_view(View::FindingDetail);
            }
            EventResult::Handled
        }

        // View switching
        KeyCode::Char('l') | KeyCode::Char('L') => {
            app.switch_view(View::FindingList);
            EventResult::Handled
        }
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.switch_view(View::Settings);
            EventResult::Handled
        }

        // Actions
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.scan();
            EventResult::Handled
        }
        KeyCode::Char('/') => {
            app.search_mode = true;
            app.search_query.clear();
            EventResult::Handled
        }

        _ => EventResult::NotHandled,
    }
}

/// Handle keybindings for finding list view
fn handle_list_keys(app: &mut App, key: KeyEvent) -> EventResult {
    match key.code {
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => {
            app.navigate_up();
            EventResult::Handled
        }
        KeyCode::Down | KeyCode::Char('j') => {
            app.navigate_down();
            EventResult::Handled
        }
        KeyCode::Enter => {
            if !app.filtered_results.is_empty() {
                app.switch_view(View::FindingDetail);
            }
            EventResult::Handled
        }
        KeyCode::Left | KeyCode::Char('h') => {
            app.go_back();
            EventResult::Handled
        }

        // Filtering (number keys)
        KeyCode::Char('1') => {
            app.toggle_severity_filter(None);
            EventResult::Handled
        }
        KeyCode::Char('2') => {
            use crate::app::Severity;
            app.toggle_severity_filter(Some(Severity::Critical));
            EventResult::Handled
        }
        KeyCode::Char('3') => {
            use crate::app::Severity;
            app.toggle_severity_filter(Some(Severity::High));
            EventResult::Handled
        }
        KeyCode::Char('4') => {
            use crate::app::Severity;
            app.toggle_severity_filter(Some(Severity::Medium));
            EventResult::Handled
        }
        KeyCode::Char('5') => {
            use crate::app::Severity;
            app.toggle_severity_filter(Some(Severity::Low));
            EventResult::Handled
        }

        // Sorting
        KeyCode::Char('s') | KeyCode::Char('S') => {
            app.set_sort_field(SortField::Severity);
            EventResult::Handled
        }
        KeyCode::Char('f') | KeyCode::Char('F') => {
            app.set_sort_field(SortField::File);
            EventResult::Handled
        }
        KeyCode::Char('l') | KeyCode::Char('L') => {
            app.set_sort_field(SortField::Line);
            EventResult::Handled
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            app.set_sort_field(SortField::Pattern);
            EventResult::Handled
        }

        // View switching
        KeyCode::Char('d') | KeyCode::Char('D') => {
            app.switch_view(View::Dashboard);
            EventResult::Handled
        }

        // Search
        KeyCode::Char('/') => {
            app.search_mode = true;
            app.search_query.clear();
            EventResult::Handled
        }

        // Actions
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.scan();
            EventResult::Handled
        }

        _ => EventResult::NotHandled,
    }
}

/// Handle keybindings for finding detail view
fn handle_detail_keys(app: &mut App, key: KeyEvent) -> EventResult {
    match key.code {
        // Navigation
        KeyCode::Left | KeyCode::Char('h') | KeyCode::Esc => {
            app.go_back();
            EventResult::Handled
        }

        // Actions
        KeyCode::Char('i') | KeyCode::Char('I') => {
            app.ignore_selected();
            EventResult::Handled
        }
        KeyCode::Char('f') | KeyCode::Char('F') => {
            app.mark_false_positive();
            EventResult::Handled
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Rotate action - just show message for now
            app.status_message =
                Some("Rotate the credential in the respective service".to_string());
            EventResult::Handled
        }
        KeyCode::Char('c') | KeyCode::Char('C') => {
            // Clear status
            if let Some(finding) = app.scan_results.get_mut(app.selected_index) {
                finding.ignored = false;
                finding.false_positive = false;
            }
            app.status_message = Some("Status cleared".to_string());
            EventResult::Handled
        }

        _ => EventResult::NotHandled,
    }
}

/// Handle keybindings for settings view
fn handle_settings_keys(app: &mut App, key: KeyEvent) -> EventResult {
    match key.code {
        KeyCode::Left | KeyCode::Char('h') | KeyCode::Esc | KeyCode::Char('q') => {
            app.go_back();
            EventResult::Handled
        }
        _ => EventResult::NotHandled,
    }
}

/// Poll for and handle the next event
pub fn poll_and_handle_event(
    app: &mut App,
    timeout: Duration,
) -> Result<EventResult, std::io::Error> {
    if event::poll(timeout)? {
        if let event::Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press {
                return Ok(handle_key_event(app, key));
            }
        }
    }
    Ok(EventResult::NotHandled)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::{Finding, Severity};
    use std::path::PathBuf;

    fn create_test_app() -> App {
        let mut app = App::new(PathBuf::from("."));
        app.scan_results = vec![Finding {
            file: "test1.rs".to_string(),
            line: 1,
            column: None,
            pattern: "TEST".to_string(),
            severity: Severity::High,
            recommendation: "Test".to_string(),
            line_content: None,
            code_context: None,
            confidence: 100,
            ignored: false,
            false_positive: false,
            notes: None,
        }];
        app.apply_filters();
        app
    }

    #[test]
    fn test_quit_key() {
        let mut app = create_test_app();
        let key = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::empty());
        let result = handle_key_event(&mut app, key);
        assert_eq!(result, EventResult::Quit);
    }

    #[test]
    fn test_navigation() {
        let mut app = create_test_app();

        // Navigate down
        let key = KeyEvent::new(KeyCode::Down, KeyModifiers::empty());
        let result = handle_key_event(&mut app, key);
        assert_eq!(result, EventResult::Handled);

        // Navigate up
        let key = KeyEvent::new(KeyCode::Up, KeyModifiers::empty());
        let result = handle_key_event(&mut app, key);
        assert_eq!(result, EventResult::Handled);
    }

    #[test]
    fn test_view_switching() {
        let mut app = create_test_app();

        // Switch to list view
        let key = KeyEvent::new(KeyCode::Char('l'), KeyModifiers::empty());
        handle_key_event(&mut app, key);
        assert_eq!(app.view, View::FindingList);

        // Go back
        let key = KeyEvent::new(KeyCode::Left, KeyModifiers::empty());
        handle_key_event(&mut app, key);
        assert_eq!(app.view, View::Dashboard);
    }

    #[test]
    fn test_severity_filter() {
        let mut app = create_test_app();
        app.switch_view(View::FindingList);

        // Filter by critical
        let key = KeyEvent::new(KeyCode::Char('2'), KeyModifiers::empty());
        handle_key_event(&mut app, key);
        assert!(app.filter_severity.is_some());

        // Clear filter
        let key = KeyEvent::new(KeyCode::Char('1'), KeyModifiers::empty());
        handle_key_event(&mut app, key);
        assert!(app.filter_severity.is_none());
    }

    #[test]
    fn test_search_mode() {
        let mut app = create_test_app();

        // Enter search mode
        let key = KeyEvent::new(KeyCode::Char('/'), KeyModifiers::empty());
        handle_key_event(&mut app, key);
        assert!(app.search_mode);

        // Type search query
        let key = KeyEvent::new(KeyCode::Char('t'), KeyModifiers::empty());
        handle_key_event(&mut app, key);
        assert_eq!(app.search_query, "t");

        // Exit search mode
        let key = KeyEvent::new(KeyCode::Esc, KeyModifiers::empty());
        handle_key_event(&mut app, key);
        assert!(!app.search_mode);
    }
}
