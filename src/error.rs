extern crate hidapi;

use hidapi::HidError;

#[derive(Debug)]
pub enum QKError {
    QKApiError {},
    QKDeviceNotFound {},
    QKConnectionError {},
    QKHidError {},
    QKNoEvents {},
}

impl From<HidError> for QKError {
    fn from(_: HidError) -> Self {
        QKError::QKHidError { }
    }
}
