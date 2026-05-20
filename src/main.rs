use crate::app::{App, AppWidget};
pub mod app;
pub mod event;
pub mod generator;
pub mod resources;
pub mod shared_fn;
pub mod upgrades;
fn main() -> color_eyre::Result<()> {
    let terminal = ratatui::init();
    let app = App::new(AppWidget::new());
    let result = app.run(terminal);
    ratatui::restore();
    result
}
