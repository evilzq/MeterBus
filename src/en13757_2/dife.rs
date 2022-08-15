use super::frame::Part;

pub struct DIFE {
    storage_number: u8,
    tariff: u8,
    device: u8,
    extension: bool,
    data: u8,
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
