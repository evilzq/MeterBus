use crate::checksum;

use super::{MBUS_FRAME_SHORT_START, MBUS_FRAME_STOP};

pub struct ShortFrame {
    start: u8,
    control: u8,
    address: u8,
    crc: u8,
    stop: u8,
}

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
