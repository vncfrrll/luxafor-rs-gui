mod app;
mod luxafor;
mod ui;

use crate::app::App;

fn main() -> iced::Result {
    iced::run(App::update, App::view)
}
