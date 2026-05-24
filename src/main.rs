use std::{env, error::Error};

use crate::app::{App, AppWidget, FileError, load_saved_app};
pub mod app;
pub mod event;
pub mod generator;
pub mod resources;
pub mod shared_fn;
pub mod upgrades;
const XOR_MASK: &[u8] = b"ILoveMinecraft";
const VERSION: &str = "0.0.0";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args().skip(1);
    let mut make_new = false;
    let mut file_to_save = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--new" => make_new = true,
            "--save" => {
                if let Some(val) = args.next() {
                    file_to_save = Some(val.to_string());
                } else {
                    eprintln!("--save flag requires a file path");
                    std::process::exit(1)
                }
            }
            _ => {}
        }
    }
    let app_widget: AppWidget;
    let terminal = ratatui::init();
    if make_new {
        app_widget = AppWidget::new();
    } else {
        app_widget = match load_saved_app(file_to_save.clone()) {
            Ok(save) => AppWidget::from_save(save),
            Err(e) => {
                if let Some(file_e) = e.downcast_ref::<FileError>() {
                    return Err(Box::new(file_e.clone()));
                }
                AppWidget::new()
            }
        }
    }
    let app = App::new(app_widget, file_to_save);
    let result = app.run(terminal);
    ratatui::restore();
    result
}
