#![allow(dead_code)]
extern crate hidapi;

use hidapi::{HidApi, HidDevice};

mod error;
mod msgs;

pub use error::QKError;
use msgs::*;

#[derive(Debug, PartialEq)]
pub enum ConnectionMode {
    Wired,
    Wireless,
    Auto,
}

pub type QKResult<T> = Result<T, QKError>;

// Search and connect to a Quick Keys device using a HidApi instance
pub fn open(hidapi: HidApi, mode: ConnectionMode) -> QKResult<QKDevice> {
    hidapi
        .device_list()
        .find(|&dev| {
            dev.vendor_id() == 0x28BD
                && ((dev.product_id() == 0x5202 && mode != ConnectionMode::Wireless)
                    || (dev.product_id() == 0x5202 && mode != ConnectionMode::Wired))
                && dev.usage() == 1
                && dev.usage_page() == 0xff0a
        })
        .map(|dev| match dev.open_device(&hidapi) {
            Ok(d) => Ok(QKDevice { device: d }),
            Err(e) => Err(QKError::QKConnectionError {}),
        })
        .or_else(|| Some(Err(QKError::QKDeviceNotFound {})))
        .unwrap()
}

pub struct QKDevice {
    device: HidDevice,
}

// Use to send and receive commands from a particular Quick Keys device
impl QKDevice {
    //
    // Output Api
    //
    pub fn set_screen_orientation(&self, orientation: ScreenOrientation) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn set_screen_brightness(&self, level: ScreenBrightness) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn set_wheel_speed(&self, speed: WheelSpeed) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn set_sleep_timeout(&self, minutes: u8) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn set_ring_color(&self, red: u8, green: u8, blue: u8) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn set_key_text(&self, key: u8, text: &str) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn show_overlay_text(&self, text: &str, seconds: u8) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }

    //
    // Input Api
    //
    pub fn set_blocking_mode(&self, blocking: bool) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn read(&self) -> QKResult<Event> {
        Err(QKError::QKNoEvents {})
    }
    pub fn read_timeout(&self, timeout: i32) -> QKResult<Event> {
        Err(QKError::QKNoEvents {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
