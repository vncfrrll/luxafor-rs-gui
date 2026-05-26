pub mod mode;
pub use mode::{Mode, WaveType, WaveActiveColor};

use std::time::Duration;
use iced::{Element, Task, Subscription, time};
use crate::luxafor::LuxaforDevice;

#[derive(Debug, Clone)]
pub enum Message {
    RChanged(u8),
    GChanged(u8),
    BChanged(u8),
    HexChanged(String),
    ApplyColor,
    TurnOff,
    _CommandSent(Result<(), String>),
    Poll,
    ModeChanged(Mode),
    SpeedChanged(u8),
    RepeatChanged(u8),
    WaveTypeChanged(WaveType),
    WaveActiveColorChanged(WaveActiveColor),
}

pub struct App {
    device: Option<LuxaforDevice>,
    status: String,
    r: u8,
    g: u8,
    b: u8,
    hex_input: String,
    hex_valid: bool,
    mode: Mode,
    speed: u8,
    repeat: u8,
    wave_type: WaveType,
    wave_color_a: (u8, u8, u8),
    wave_color_b: (u8, u8, u8),
    wave_active_color: WaveActiveColor
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
        Self {
            device,
            status,
            r: 0,
            g: 0,
            b: 0,
            hex_input: "000000".to_string(),
            hex_valid: true,
            mode: Mode::Static,
            speed: 128,
            repeat: 1,
            wave_type: WaveType::Short,
            wave_color_a: (0, 0, 0),
            wave_color_b: (255, 255, 255),
            wave_active_color: WaveActiveColor::A
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::RChanged(r) => {
                self.update_channel(Some(r), None, None);
                Task::none()
            }
            Message::GChanged(g) => {
                self.update_channel(None, Some(g), None);
                Task::none()
            }
            Message::BChanged(b) => {
                self.update_channel(None, None, Some(b));
                Task::none()
            }
            Message::HexChanged(hex) => {
                self.hex_input = hex.clone();
                if hex.len() == 6 {
                    if let Ok(parsed) = u32::from_str_radix(&hex, 16) {
                        let r = ((parsed >> 16) & 0xFF) as u8;
                        let g = ((parsed >> 8) & 0xFF) as u8;
                        let b = (parsed & 0xFF) as u8;
                        if self.mode == Mode::Wave {
                            match self.wave_active_color {
                                WaveActiveColor::A => self.wave_color_a = (r, g, b),
                                WaveActiveColor::B => self.wave_color_b = (r, g, b),
                            }
                        }
                        else {
                            self.r = r;
                            self.g = g;
                            self.b = b;
                        }
                        self.hex_valid = true;
                    } else {
                        self.hex_valid = false;
                    }
                }
                else {
                    self.hex_valid = false;
                }
                Task::none()
            }
            Message::ApplyColor => {
                if let Some(device) = &self.device {
                    let result = match self.mode {
                        Mode::Static => device.set_color(self.r, self.g, self.b)
                            .map_err(|e| e.to_string()),
                        Mode::Fade => device.turn_off()
                            .and_then(|_| device.fade(self.r, self.g, self.b, self.speed))
                            .map_err(|e| e.to_string()),
                        Mode::Strobe => device.strobe(self.r, self.g, self.b, self.speed, self.repeat)
                            .map_err(|e| e.to_string()),
                        Mode::Wave => {
                            let wave_type_byte = self.wave_type.to_byte();
                            if self.wave_type.uses_two_colors() {
                                device.set_color(self.wave_color_a.0, self.wave_color_a.1, self.wave_color_a.2)
                                    .and_then(|_| device.wave(self.wave_color_b.0, self.wave_color_b.1, self.wave_color_b.2, wave_type_byte, self.speed, self.repeat))
                                    .map_err(|e| e.to_string())
                            }
                            else {
                                device.wave(self.wave_color_b.0, self.wave_color_b.1, self.wave_color_b.2, wave_type_byte, self.speed, self.repeat)
                                    .map_err(|e| e.to_string())
                            }

                        },
                    };
                    self.status = match &result {
                        Ok(_) => format!("Color set to #{:02X}{:02X}{:02X}", self.r, self.g, self.b),
                        Err(e) => format!("Error: {}", e),
                    };
                }
                Task::none()
            }
            Message::ModeChanged(mode) => {
                self.mode = mode;
                if mode == Mode::Wave {
                    let (r, g, b) = match self.wave_active_color {
                        WaveActiveColor::A => self.wave_color_a,
                        WaveActiveColor::B => self.wave_color_b,
                    };
                    self.hex_input = format!("{:02X}{:02X}{:02X}", r, g, b);
                    self.hex_valid = true;
                }
                else {
                    self.hex_input = format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b);
                    self.hex_valid = true;
                }
                Task::none()
            }
            Message::SpeedChanged(speed) => {
                self.speed = speed;
                Task::none()
            }
            Message::RepeatChanged(repeat) => {
                self.repeat = repeat;
                Task::none()
            }
            Message::WaveTypeChanged(wave_type) => {
                self.wave_type = wave_type;
                if !wave_type.uses_two_colors() {
                    self.wave_active_color = WaveActiveColor::B;
                    let (r, g, b) = self.wave_color_b;
                    self.hex_input = format!("{:02X}{:02X}{:02X}", r, g, b);
                    self.hex_valid = true;
                }
                Task::none()
            }
            Message::WaveActiveColorChanged(active) => {
                self.wave_active_color = active;
                let (r, g, b) = match active {
                    WaveActiveColor::A => self.wave_color_a,
                    WaveActiveColor::B => self.wave_color_b,
                };
                self.hex_input = format!("{:02X}{:02X}{:02X}", r, g, b);
                self.hex_valid = true;
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
        let (slider_r, slider_g, slider_b) = if self.mode == Mode::Wave {
            match self.wave_active_color {
                WaveActiveColor::A => self.wave_color_a,
                WaveActiveColor::B => self.wave_color_b,
            }
        }
        else {
            (self.r, self.g, self.b)
        };

        crate::ui::main_window::view(
            self.device.is_some(),
            &self.status,
            slider_r,
            slider_g,
            slider_b,
            &self.hex_input,
            self.hex_valid,
            &self.mode,
            self.speed,
            self.repeat,
            &self.wave_type,
            self.wave_color_a,
            self.wave_color_b,
            &self.wave_active_color,
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Poll)
    }

    fn update_channel(&mut self, r: Option<u8>, g: Option<u8>, b: Option<u8>) {
        if self.mode == Mode::Wave {
            let color = match self.wave_active_color {
                WaveActiveColor::A => &mut self.wave_color_a,
                WaveActiveColor::B => &mut self.wave_color_b,
            };
            if let Some(r) = r { color.0 = r; }
            if let Some(g) = g { color.1 = g; }
            if let Some(b) = b { color.2 = b; }
            let (r, g, b) = *color;
            self.hex_input = format!("{:02X}{:02X}{:02X}", r, g, b);
        }
        else {
            if let Some(r) = r { self.r = r; }
            if let Some(g) = g { self.g = g; }
            if let Some(b) = b { self.b = b; }
            self.hex_input = format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b);
        }
        self.hex_valid = true;
    }
}