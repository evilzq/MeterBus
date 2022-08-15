use crate::checksum;

use super::{frame::Frame, MBUS_FRAME_SHORT_START, MBUS_FRAME_STOP};

pub struct ShortFrame {
    pub start: u8,
    pub control: u8,
    pub address: u8,
    pub crc: u8,
    pub stop: u8,
}

impl Frame for ShortFrame {}

impl ShortFrame {
    pub fn new(control: u8, address: u8) -> Self {
        Self {
            start: MBUS_FRAME_SHORT_START,
            control,
            address,
            crc: checksum(&[control, address]),
            stop: MBUS_FRAME_STOP,
        }
    }
}
