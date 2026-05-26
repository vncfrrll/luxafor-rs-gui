#![windows_subsystem = "windows"]

mod app;
mod luxafor;
mod ui;

use crate::app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Luxafor Control App")
        .subscription(App::subscription)
        .run()
}
