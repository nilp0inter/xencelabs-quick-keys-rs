#![allow(dead_code)]
extern crate hidapi;

use hidapi::{HidApi, HidDevice};

mod error;
mod msgs;

pub use error::QKError;
pub use msgs::{ButtonState, Event, ScreenOrientation, WheelDirection, ScreenBrightness, WheelSpeed};
use msgs::*;

/// Connection method (cable, wireless, or automatic...)
#[derive(Debug, PartialEq)]
pub enum ConnectionMode {
    Wired,
    Wireless,  // NOTE: Not implemented yet
    Auto,
}

pub type QKResult<T> = Result<T, QKError>;


/// Use to send and receive commands from a particular Quick Keys device.
pub struct QKDevice {
    device: HidDevice,
}

impl QKDevice {
    /// Search and connect to a Quick Keys device using a HidApi instance.
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
                Err(_) => Err(QKError::QKConnectionError),
            })
            .or_else(|| Some(Err(QKError::QKDeviceNotFound)))
            .unwrap()?;
        this.device.write(&msg_subscribe_to_key_events())?;
        this.device.write(&msg_subscribe_to_battery())?;
        Ok(this)
    }

    //
    // Output Api
    //

    /// Rotate the screen to the given angle.
    pub fn set_screen_orientation(&self, orientation: ScreenOrientation) -> QKResult<()> {
        self.device.write(&msg_rotate_screen(orientation))?;
        Ok(())
    }

    /// Set screen brightness to the given level.
    pub fn set_screen_brightness(&self, level: ScreenBrightness) -> QKResult<()> {
        self.device.write(&msg_set_screen_brightness(level))?;
        Ok(())
    }


    /// Set the wheel speed to the given value.
    pub fn set_wheel_speed(&self, speed: WheelSpeed) -> QKResult<()> {
        self.device.write(&msg_set_wheel_speed(speed))?;
        Ok(())
    }

    /// Switch off the device after the given amount of minutes (after connection is lost).
    pub fn set_sleep_timeout(&self, minutes: u8) -> QKResult<()> {
        self.device.write(&msg_set_sleep_timeout(minutes))?;
        Ok(())
    }

    /// Set the color of the LED ring of the wheel to the given RGB value.
    pub fn set_ring_color(&self, red: u8, green: u8, blue: u8) -> QKResult<()> {
        self.device.write(&msg_set_wheel_color(red, green, blue))?;
        Ok(())
    }

    /// Set the text label for the given key (0-7).  (max 8 characters, ATM).
    pub fn set_key_text(&self, key: u8, text: &str) -> QKResult<()> {
        self.device.write(&msg_set_key_text(key, text))?;
        Ok(())
    }


    /// Show a text overlay for a fix amount of time (max 32 characters).
    pub fn show_overlay_text(&self, text: &str, seconds: u8) -> QKResult<()> {
        for chunk in &msgs_show_overlay_text(seconds, text) {
            self.device.write(chunk)?;
        }
        Ok(())
    }

    //
    // Input Api
    //

    /// Set the blocking mode (see hidapi for details).
    pub fn set_blocking_mode(&self, blocking: bool) -> QKResult<()> {
        self.device.set_blocking_mode(blocking).map_err(QKError::QKHidError)
    }

    /// Read the next Event.  By default in blocks (unless set_blocking_mode(false)).
    pub fn read(&self) -> QKResult<Event> {
        let mut buf = [0u8; 10];
        self.device.read(&mut buf[..])?;
        Ok(process_input(&buf))
    }


    /// Try to read the next Event in the next number of milliseconds. 
    pub fn read_timeout(&self, timeout: i32) -> QKResult<Event> {
        let mut buf = [0u8; 10];
        match self.device.read_timeout(&mut buf[..], timeout) {
            Ok(_) => Ok(process_input(&buf)),
            Err(e) => Err(QKError::QKHidError(e)),
        }
    }
}
