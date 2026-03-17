//! Coax TUI - Terminal User Interface for Coax Security Scanner
//!
//! This crate provides a terminal-based dashboard for viewing and managing
//! security scan results from the coax scanner.
//!
//! # Features
//!
//! - Dashboard view with scan statistics
//! - Scrollable finding list with filtering
//! - Finding detail view with code preview
//! - Settings panel for configuration
//! - Full keyboard navigation
//!
//! # Example
//!
//! ```rust,no_run
//! use coax_tui::App;
//! use crossterm::{
//!     terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
//!     ExecutableCommand,
//! };
//! use ratatui::Terminal;
//! use std::io;
//! use std::path::PathBuf;
//!
//! fn main() -> io::Result<()> {
//!     terminal::enable_raw_mode()?;
//!     let mut stdout = io::stdout();
//!     stdout.execute(EnterAlternateScreen)?;
//!     let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(stdout))?;
//!
//!     let mut app = App::new(PathBuf::from("."));
//!     app.scan();
//!
//!     loop {
//!         terminal.draw(|frame| coax_tui::ui::render(frame, &mut app))?;
//!         
//!         use crossterm::event::{self, Event, KeyEventKind};
//!         if event::poll(std::time::Duration::from_millis(100))? {
//!             if let Event::Key(key) = event::read()? {
//!                 if key.kind == KeyEventKind::Press {
//!                     match coax_tui::events::handle_key_event(&mut app, key) {
//!                         coax_tui::events::EventResult::Quit => break Ok(()),
//!                         _ => {}
//!                     }
//!                 }
//!             }
//!         }
//!     }
//! }
//! ```

pub mod app;
pub mod components;
pub mod events;
pub mod ui;
pub mod views;

pub use app::{App, SortField, SortOrder, View};
pub use events::handler::handle_key_event;

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
