use iced::widget::{button, column, container, row, text, slider, text_input};
use iced::{Alignment, Color, Element, Length};
use crate::app::Message;
use crate::ui::styles;

pub fn view<'a>(_connected: bool, status: &str, r: u8, g: u8, b: u8, hex_input: &str, hex_valid: bool) -> Element<'a, Message> {
    let preview_color = Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);

    let preview = container("")
        .width(Length::Fill)
        .height(50)
        .style(move |_theme| container::Style {
            background: Some(preview_color.into()),
            ..Default::default()
        });

    let r_slider = row![
        text("R").width(20),
        slider(0..=255, r, |v| Message::RChanged(v)),
        text(r.to_string()).width(40),
    ]
        .spacing(10)
        .align_y(Alignment::Center);

    let g_slider = row![
        text("G").width(20),
        slider(0..=255, g, |v| Message::GChanged(v)),
        text(g.to_string()).width(40),
    ]
        .spacing(10)
        .align_y(Alignment::Center);

    let b_slider = row![
        text("B").width(20),
        slider(0..=255, b, |v| Message::BChanged(v)),
        text(b.to_string()).width(40),
    ]
        .spacing(10)
        .align_y(Alignment::Center);

    let hex_field = row![
        text("#").size(16),
        text_input("000000", hex_input)
            .on_input(Message::HexChanged)
            .width(80),
    ]
        .spacing(5)
        .align_y(Alignment::Center);

    let apply_button = button("Apply")
        .on_press(Message::ApplyColor)
        .style(crate::ui::styles::rounded_button);

    let off_button = button("Off")
        .on_press(Message::TurnOff)
        .style(crate::ui::styles::rounded_button);

    let controls = row![hex_field, apply_button, off_button]
        .spacing(10)
        .align_y(Alignment::Center);

    let status_color = if hex_valid {
        Color::from_rgb(0.0, 0.0, 0.0)
    } else {
        Color::from_rgb(0.8, 0.0, 0.0)
    };

    let status_text = text(format!("Status: {}", status))
        .color(status_color);

    let content = column![
        preview,
        r_slider,
        g_slider,
        b_slider,
        controls,
        status_text,
    ]
        .spacing(20)
        .align_x(Alignment::Center)
        .padding(20);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}