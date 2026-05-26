use iced::widget::{button, column, container, radio, row, slider, text, text_input};
use iced::{Alignment, Color, Element, Length};
use crate::app::{Message, mode::{Mode, WaveType, WaveActiveColor}};

pub fn view<'a>(
    _connected: bool,
    status: &'a str,
    r: u8,
    g: u8,
    b: u8,
    hex_input: &'a str,
    hex_valid: bool,
    mode: &'a Mode,
    speed: u8,
    repeat: u8,
    wave_type: &'a WaveType,
    wave_color_a: (u8, u8, u8),
    wave_color_b: (u8, u8, u8),
    wave_active_color: &'a WaveActiveColor,
) -> Element<'a, Message> {
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

    let mode_selector = row![
        radio("Static", Mode::Static, Some(*mode), Message::ModeChanged),
        radio("Fade", Mode::Fade, Some(*mode), Message::ModeChanged),
        radio("Strobe", Mode::Strobe, Some(*mode), Message::ModeChanged),
        radio("Wave", Mode::Wave, Some(*mode), Message::ModeChanged),
    ]
        .spacing(15)
        .align_y(Alignment::Center);

    let speed_row = row![
        text("Speed").width(60),
        slider(0..=255, speed, Message::SpeedChanged),
        text(speed.to_string()).width(40),
    ]
        .spacing(10)
        .align_y(Alignment::Center);

    let repeat_row = row![
        text("Repeat").width(60),
        slider(0..=255, repeat, Message::RepeatChanged),
        text(repeat.to_string()).width(40),
    ]
        .spacing(10)
        .align_y(Alignment::Center);

    let mode_controls = match mode {
        Mode::Static => column![].spacing(10),
        Mode::Fade => column![speed_row].spacing(10),
        Mode::Strobe => column![speed_row, repeat_row].spacing(10),
        Mode::Wave => {
            let color_a_preview = color_preview(wave_color_a);

            let color_b_preview = color_preview(wave_color_b);

            let color_selector = if wave_type.uses_two_colors() {
                row![
                radio("Color 1", WaveActiveColor::A, Some(*wave_active_color), Message::WaveActiveColorChanged),
                color_a_preview,
                radio("Color 2", WaveActiveColor::B, Some(*wave_active_color), Message::WaveActiveColorChanged),
                color_b_preview,
            ]
                    .spacing(10)
                    .align_y(Alignment::Center)
            } else {
                row![
                    radio("Color", WaveActiveColor::B, Some(*wave_active_color), Message::WaveActiveColorChanged),
                    color_b_preview,
                ]
                    .spacing(10)
                    .align_y(Alignment::Center)
            };

            let wave_type_row = row(
                WaveType::all().into_iter().map(|wt| {
                    radio(
                        wt.label(),
                        wt,
                        Some(*wave_type),
                        Message::WaveTypeChanged,
                    ).into()
                }).collect::<Vec<_>>()
            )
                .spacing(10)
                .align_y(Alignment::Center);

            column![
                color_selector,
                speed_row,
                repeat_row,
                wave_type_row,
            ]
                .spacing(10)
        }
    };

    let apply_button = button("Apply")
        .on_press(Message::ApplyColor)
        .style(crate::ui::styles::rounded_button);

    let off_button = button("Off")
        .on_press(Message::TurnOff)
        .style(crate::ui::styles::rounded_button);

    let controls = row![hex_field, apply_button, off_button]
        .spacing(10)
        .align_y(Alignment::Center);

    let status_text = if hex_valid {
        text(format!("Status: {}", status))
    } else {
        text(format!("Status: {}", status))
            .color(Color::from_rgb(0.8, 0.0, 0.0))
    };

    let content = column![
        preview,
        r_slider,
        g_slider,
        b_slider,
        controls,
        mode_selector,
        mode_controls,
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

fn color_preview(color: (u8, u8, u8)) -> container::Container<'static, Message> {
    container("")
        .width(60)
        .height(30)
        .style(move |_theme| container::Style {
            background: Some(Color::from_rgb(
                color.0 as f32 / 255.0,
                color.1 as f32 / 255.0,
                color.2 as f32 / 255.0,
            ).into()),
            ..Default::default()
        })
}