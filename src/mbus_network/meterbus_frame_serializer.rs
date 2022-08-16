use super::{
    ack_frame::AckFrame,
    control::{ControlFrame, ControlInformation},
    short_frame::ShortFrame,
    var_data::VariableDataLongFrame,
    MBUS_FRAME_ACK_START, MBUS_FRAME_LONG_START, MBUS_FRAME_SHORT_START, MBUS_FRAME_STOP,
};
use crate::{checksum, checksum_vec};
use alloc::vec::Vec;

impl TryFrom<&[u8]> for AckFrame {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value[0] == MBUS_FRAME_ACK_START {
            Ok(AckFrame {})
        } else {
            Err("Not Qualified ACK Frame")
        }
    }
}

impl<'a> TryFrom<&'a [u8]> for VariableDataLongFrame<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let mut iter = value.iter();
        match iter.next() {
            Some(start) => {
                if *start == MBUS_FRAME_LONG_START {
                    let length1 = iter.next().unwrap_or(&0);
                    let length2 = iter.next().unwrap_or(&0);
                    if (*length1 < 3) | (*length1 != *length2) {
                        return Err("Invalid Data Frame");
                    }
                    let start2 = iter.next().unwrap_or(&0);
                    if *start2 != MBUS_FRAME_LONG_START {
                        return Err("Invalid Data Frame");
                    }
                    let control = iter.next().unwrap_or(&0);
                    let address = iter.next().unwrap_or(&0);
                    let control_infomation = iter.next().unwrap_or(&0);
                    if *length1 - 3 != 0 {
                        return Err("Invalid Control Frame");
                    }
                    let mut data_vec: Vec<u8> = Vec::new();
                    let mut crc_vec: Vec<u8> = Vec::new();
                    let mut i = 0;
                    while i < (*length1 - 3) {
                        match iter.next() {
                            Some(v) => data_vec.push(*v),
                            None => {
                                return Err("Invalid Data Frame");
                            }
                        }
                        i += 1;
                    }
                    if data_vec.len() != (*length1 - 3).into() {
                        return Err("Invalid Data Frame");
                    }
                    let crc = iter.next().unwrap_or(&0);
                    let stop = iter.next().unwrap_or(&0);
                    if *stop != MBUS_FRAME_STOP {
                        return Err("Invalid Data Frame");
                    }
                    if (*control == 0) & (*address == 0) & (*control_infomation == 0) {
                        return Err("Invalid Data Frame");
                    }
                    crc_vec.push(*control);
                    crc_vec.push(*address);
                    crc_vec.push(*control_infomation);
                    for v in data_vec.as_slice() {
                        crc_vec.push(*v);
                    }
                    if *crc != checksum_vec(crc_vec) {
                        return Err("Invalid CRC");
                    }
                    let ci = ControlInformation::try_from(*control_infomation)
                        .unwrap_or(ControlInformation::Other);
                    if ci == ControlInformation::RespVariable {
                        Ok(VariableDataLongFrame::new(
                            *control,
                            *control_infomation,
                            *address,
                            data_vec,
                            *length1,
                        )
                        .unwrap_or(VariableDataLongFrame {
                            control: 0,
                            address: 0,
                            control_infomation: ControlInformation::Other,
                            indentification_no: [0, 0, 0, 0],
                            device_type: 0,
                            transmission_counter: 0,
                            status: 0,
                            length: 0,
                            manufacterer: [0, 0],
                            version: 0,
                            signature: [0, 0],
                            parts: Vec::new(),
                        }))
                    } else {
                        Err("Not Qualified VRD Frame")
                    }
                } else {
                    Err("Not Qualified VRD Frame")
                }
            }
            None => Err("Invalid Data Frame"),
        }
    }
}

impl TryFrom<&[u8]> for ShortFrame {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut iter = value.iter();
        match iter.next() {
            Some(start) => {
                if *start == MBUS_FRAME_SHORT_START {
                    let control = iter.next().unwrap_or(&0);
                    if *control == 0 {
                        return Err("Invalid Data Frame");
                    }
                    let address = iter.next().unwrap_or(&0);
                    if *address == 0 {
                        return Err("Invalid Data Frame");
                    }
                    let crc = iter.next().unwrap_or(&0);
                    if *crc == 0 {
                        return Err("Invalid Data Frame");
                    }
                    if *crc != checksum(&[*control, *address]) {
                        return Err("Invalid CRC");
                    }
                    let stop = iter.next().unwrap_or(&0);
                    if *stop != MBUS_FRAME_STOP {
                        return Err("Invalid Data Frame");
                    }
                    Ok(ShortFrame::new(*control, *address))
                } else {
                    Err("Not Qualified Short Frame")
                }
            }
            None => Err("Invalid Data Frame"),
        }
    }
}

impl TryFrom<&[u8]> for ControlFrame {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut iter = value.iter();
        match iter.next() {
            Some(start) => {
                if *start == MBUS_FRAME_LONG_START {
                    let length1 = iter.next().unwrap_or(&0);
                    let length2 = iter.next().unwrap_or(&0);
                    if (*length1 < 3) | (*length1 != *length2) {
                        return Err("Invalid Data Frame");
                    }
                    let start2 = iter.next().unwrap_or(&0);
                    if *start2 != MBUS_FRAME_LONG_START {
                        return Err("Invalid Data Frame");
                    }
                    let control = iter.next().unwrap_or(&0);
                    let address = iter.next().unwrap_or(&0);
                    let control_infomation = iter.next().unwrap_or(&0);
                    if *length1 - 3 != 0 {
                        return Err("Invalid Control Frame");
                    }
                    /*let data_vec: Vec<u8> = Vec::new();
                    let mut i = 0;
                    while i < (*length1 - 3) {
                        match iter.next() {
                            Some(v) => data_vec.push(*v),
                            None => {
                                return Err("Invalid Data Frame");
                            }
                        }
                    }
                    if data_vec.len() != (*length1 - 3).into() {
                        return Err("Invalid Data Frame");
                    }*/
                    let crc = iter.next().unwrap_or(&0);
                    let stop = iter.next().unwrap_or(&0);
                    if *stop != MBUS_FRAME_STOP {
                        return Err("Invalid Data Frame");
                    }
                    if (*control == 0) & (*address == 0) & (*control_infomation == 0) {
                        return Err("Invalid Data Frame");
                    }
                    if *crc != checksum(&[*control, *address, *control_infomation]) {
                        return Err("Invalid CRC");
                    }
                    Ok(ControlFrame::new(*control, *control_infomation, *address))
                } else {
                    Err("Not Qualified Control Frame")
                }
            }
            None => Err("Invalid Data Frame"),
        }
    }
}
