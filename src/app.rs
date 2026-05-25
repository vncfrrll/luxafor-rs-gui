use std::time::Duration;
use iced::{Element, Task, Subscription, time};
use crate::luxafor::LuxaforDevice;

#[derive(Debug, Clone)]
pub enum Message {
    SetColor(u8, u8, u8),
    TurnOff,
    _CommandSent(Result<(), String>),
    Poll,
}

pub struct App {
    device: Option<LuxaforDevice>,
    status: String,
}

impl Default for App {
    fn default() -> Self {
        let device = LuxaforDevice::connect().ok();
        let status = if device.is_some() {
            "Device connected.".to_string()
        }
        else {
            "Device not found.".to_string()
        };
        Self {device, status}
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetColor(r, g, b) => {
                if let Some(device) = &self.device {
                    let result = device.set_color(r, g, b).map_err(|e| e.to_string());
                    self.status = match &result {
                        Ok(_) => "Color set.".to_string(),
                        Err(e) => format!("Color set error: {}", e),
                    };
                }
                Task::none()
            }
            Message::TurnOff => {
                if let Some(device) = &self.device {
                    let result = device.turn_off().map_err(|e| e.to_string());
                    self.status = match &result {
                        Ok(_) => "Turned off.".to_string(),
                        Err(e) => format!("Turn off error: {}", e),
                    };
                }
                Task::none()
            }
            Message::Poll => {
                match &self.device {
                    None => {
                        if let Ok(device) = LuxaforDevice::connect() {
                            self.device = Some(device);
                            self.status = "Device connected.".to_string();
                        }
                    }
                    Some(_) => {
                        if LuxaforDevice::is_connected().is_err() {
                            self.device = None;
                            self.status = "Device not found.".to_string();
                        }
                    }
                }
                Task::none()
            }
            Message::_CommandSent(result) => {
                self.status = match result {
                    Ok(_) => "Command sent.".to_string(),
                    Err(e) => format!("Command sent error: {}", e),
                };
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        crate::ui::main_window::view(self.device.is_some(), &self.status)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Poll)
    }
}