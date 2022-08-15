use crate::checksum;

use super::{
    control::ControlInformation, data_type::DataTypes, MBUS_FRAME_LONG_START, MBUS_FRAME_STOP,
};

pub struct LongFrame<'a> {
    strat: u8,
    control: u8,
    address: u8,
    control_infomation: ControlInformation,
    indentification_no: [u8; 4],
    device_type: u8,
    transmission_counter: u8,
    status: u8,
    data: &'a [u8],
    length: u8,
    crc: u8,
    stop: u8,
}

impl<'a> LongFrame<'a> {
    pub fn get_length_in_bit_table(types: DataTypes) -> u8 {
        match types {
            DataTypes::NoData => 0,
            DataTypes::_8BitInteger => 8,
            DataTypes::_16BitInteger => 16,
            DataTypes::_24BitInteger => 24,
            DataTypes::_32BitInteger => 32,
            DataTypes::_32BitReal => 32,
            DataTypes::_48BitInteger => 48,
            DataTypes::_64BitInteger => 64,
            DataTypes::SelectionForReadout => 0,
            DataTypes::_2DigitBcd => 8,
            DataTypes::_4DigitBcd => 16,
            DataTypes::_6DigitBcd => 24,
            DataTypes::_8DigitBcd => 32,
            DataTypes::VariableLength => 0,
            DataTypes::_12DigitBcd => 48,
            DataTypes::Unknown => 64,
        }
    }

    pub fn new(
        control: u8,
        control_infomation: u8,
        address: u8,
        data: &'a [u8],
        length: u8,
    ) -> Self {
        Self {
            strat: MBUS_FRAME_LONG_START,
            control,
            address,
            control_infomation: ControlInformation::try_from(control_infomation).unwrap(),
            indentification_no: [0, 0, 0, 0],
            device_type: 0,
            transmission_counter: 0,
            status: 0,
            data,
            length,
            crc: checksum(&[&[control, address, control_infomation], data].concat()),
            stop: MBUS_FRAME_STOP,
        }
    }
}
