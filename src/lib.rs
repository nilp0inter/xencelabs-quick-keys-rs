#![allow(dead_code)]
extern crate hidapi;

use hidapi::{HidApi, HidDevice};

mod error;
mod msgs;

pub use error::QKError;
pub use msgs::*;

#[derive(Debug, PartialEq)]
pub enum ConnectionMode {
    Wired,
    Wireless,
    Auto,
}

pub type QKResult<T> = Result<T, QKError>;


pub struct QKDevice {
    device: HidDevice,
}

// Use to send and receive commands from a particular Quick Keys device
impl QKDevice {
    // Search and connect to a Quick Keys device using a HidApi instance
    pub fn open(hidapi: HidApi, mode: ConnectionMode) -> QKResult<Self> {
        let this = hidapi
            .device_list()
            .find(|&dev| {
                dev.vendor_id() == 0x28BD
                    && ((dev.product_id() == 0x5202 && mode != ConnectionMode::Wireless)
                        || (dev.product_id() == 0x5204 && mode != ConnectionMode::Wired))
                    && dev.usage() == 1
                    && dev.usage_page() == 0xff0a
            })
            .map(|dev| match dev.open_device(&hidapi) {
                Ok(d) => Ok(QKDevice { device: d }),
                Err(e) => Err(QKError::QKConnectionError {}),
            })
            .or_else(|| Some(Err(QKError::QKDeviceNotFound {})))
            .unwrap()?;
        this.init()?;
        Ok(this)
    }

    fn init(&self) -> QKResult<()> {
        self.device.write(&msg_subscribe_to_key_events())?;
        self.device.write(&msg_subscribe_to_battery())?;
        Ok(())
    }

    //
    // Output Api
    //
    pub fn set_screen_orientation(&self, orientation: ScreenOrientation) -> QKResult<()> {
        self.device.write(&msg_rotate_screen(orientation))?;
        Ok(())
    }
    pub fn set_screen_brightness(&self, level: ScreenBrightness) -> QKResult<()> {
        self.device.write(&msg_set_screen_brightness(level))?;
        Ok(())
    }
    pub fn set_wheel_speed(&self, speed: WheelSpeed) -> QKResult<()> {
        self.device.write(&msg_set_wheel_speed(speed))?;
        Ok(())
    }
    pub fn set_sleep_timeout(&self, minutes: u8) -> QKResult<()> {
        self.device.write(&msg_set_sleep_timeout(minutes))?;
        Ok(())
    }
    pub fn set_ring_color(&self, red: u8, green: u8, blue: u8) -> QKResult<()> {
        self.device.write(&msg_set_wheel_color(red, green, blue))?;
        Ok(())
    }
    pub fn set_key_text(&self, key: u8, text: &str) -> QKResult<()> {
        self.device.write(&msg_set_key_text(key, text))?;
        Ok(())
    }
    pub fn show_overlay_text(&self, text: &str, seconds: u8) -> QKResult<()> {
        for chunk in &msgs_show_overlay_text(seconds, text) {
            self.device.write(chunk)?;
        }
        Ok(())
    }

    //
    // Input Api
    //
    pub fn set_blocking_mode(&self, blocking: bool) -> QKResult<()> {
        Err(QKError::QKApiError {})
    }
    pub fn read(&self) -> QKResult<Event> {
        let mut buf = [0u8; 10];
        self.device.read(&mut buf[..])?;
        Ok(process_input(&buf))
    }
    pub fn read_timeout(&self, timeout: i32) -> QKResult<Event> {
        let mut buf = [0u8; 10];
        match self.device.read_timeout(&mut buf[..], timeout) {
            Ok(_) => Ok(process_input(&buf)),
            Err(_) => Err(QKError::QKHidError { }),
        }
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
