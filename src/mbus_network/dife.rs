use super::frame::Part;

pub struct DIFE {
    pub storage_number: u8,
    pub tariff: u8,
    pub device: u8,
    pub extension: bool,
    pub data: u8,
}

impl Part for DIFE {}

impl DIFE {
    pub fn new(data: u8) -> Self {
        Self {
            storage_number: data & 0x0F_u8,
            tariff: (data & 0x30) >> 4,
            device: (data & 0x40) >> 6,
            extension: (data & 0x80) != 0,
            data,
        }
    }
}
