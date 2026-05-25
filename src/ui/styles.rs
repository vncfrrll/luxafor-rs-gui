use iced::widget::button;
use iced::{Border, Theme};

pub fn rounded_button(theme: &Theme, status: button::Status) -> button::Style {
    button::Style {
        border: Border {
            radius: 8.0.into(),
            ..Default::default()
        },
        ..button::primary(theme, status)
    }
}