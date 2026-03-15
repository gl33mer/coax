//! View modules for Coax TUI

pub mod dashboard;
pub mod finding_list;
pub mod finding_detail;
pub mod settings;

pub use dashboard::render_dashboard;
pub use finding_list::render_finding_list;
pub use finding_detail::render_finding_detail;
pub use settings::render_settings;
