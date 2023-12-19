extern crate hidapi;

use hidapi::HidError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum QKError {
    #[error("Quick Keys device not found")]
    QKDeviceNotFound,
    #[error("Quick Keys connection error")]
    QKConnectionError,
    #[error("Quick Keys HID error: {0}")]
    QKHidError(#[from] HidError),
}
