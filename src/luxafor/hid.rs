use hidapi::{HidApi, HidDevice, HidError};

const VENDOR_ID: u16 = 0x04D8;
const PRODUCT_ID: u16 = 0xF372;
const REPORT_SIZE: usize = 9;

pub struct LuxaforDevice {
    device: HidDevice,
}

impl LuxaforDevice {
    pub fn connect() -> Result<Self, HidError> {
        let api = HidApi::new()?;
        let device = api.open(VENDOR_ID, PRODUCT_ID)?;
        Ok(Self { device })
    }

    pub fn is_connected() -> Result<bool, HidError> {
        let api = HidApi::new()?;
        api.device_list()
            .find(|d| d.vendor_id() == VENDOR_ID && d.product_id() == PRODUCT_ID)
            .map(|_| true)
            .ok_or(HidError::HidApiError {message: "Device not found.".to_string()})
    }

    pub fn send_command(&self, report: [u8; REPORT_SIZE]) -> Result<(), HidError> {
        self.device.write(&report)?;
        Ok(())
    }

    pub fn set_color(&self, r: u8, g: u8, b: u8) -> Result<(), HidError> {
        let report = [0x00, 0x01, 0xFF, r, g, b, 0x00, 0x00, 0x00];
        self.send_command(report)
    }

    pub fn turn_off(&self) -> Result<(), HidError> {
        self.set_color(0, 0, 0)
    }
}