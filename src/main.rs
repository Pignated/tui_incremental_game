use tui_widget_list::ListState;

use crate::app::{App, AppWidget};
pub mod app;
pub mod event;
pub mod generator;
pub mod resources;
fn main() -> color_eyre::Result<()> {
    let terminal = ratatui::init();
    let app = App::new(AppWidget::new(), ListState::default());
    let result = app.run(terminal);
    ratatui::restore();
    result
}
