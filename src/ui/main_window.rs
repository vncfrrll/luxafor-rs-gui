use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length};
use crate::app::Message;
use crate::ui::styles;

pub fn view(_connected: bool, status: &str) -> Element<'_, Message> {
    let status_text = text(format!("Status: {}", status));

    let red_button = button("Red")
        .style(styles::rounded_button)
        .on_press(Message::SetColor(255, 0, 0));

    let green_button = button("Green")
        .style(styles::rounded_button)
        .on_press(Message::SetColor(0, 255, 0));

    let blue_button = button("Blue")
        .style(styles::rounded_button)
        .on_press(Message::SetColor(0, 0, 255));

    let off_button = button("Off")
        .style(styles::rounded_button)
        .on_press(Message::TurnOff);

    let color_row = row![red_button, green_button, blue_button, off_button]
        .spacing(20)
        .align_y(Alignment::Center);

    let content = column![
        status_text,
        color_row,
    ]
        .spacing(10)
        .align_x(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
}