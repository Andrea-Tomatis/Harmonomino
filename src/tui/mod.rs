mod app;
mod event_loop;
pub(crate) mod ui;
mod versus_app;
mod versus_ui;

pub use app::App;
pub use event_loop::{TuiApp, run_event_loop};
pub use ui::draw;
pub use versus_app::VersusApp;
pub use versus_ui::draw_versus;
