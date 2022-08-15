use crate::mbus_network::fixeddata::FixedDataLongFrame;
use crate::mbus_network::var_data::VariableDataLongFrame;

use super::fixed_data_packet::FixedDataPacket;
use super::variable_data_packet::VariableDataPacket;

impl<'a> TryFrom<VariableDataLongFrame<'a>> for VariableDataPacket<'a> {
    type Error = &'a str;

    fn try_from(value: VariableDataLongFrame<'_>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> TryFrom<FixedDataLongFrame<'a>> for VariableDataPacket<'a> {
    type Error = &'a str;

    fn try_from(value: FixedDataLongFrame<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> TryFrom<VariableDataLongFrame<'a>> for FixedDataPacket {
    type Error = &'a str;

    fn try_from(value: VariableDataLongFrame<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> TryFrom<FixedDataLongFrame<'a>> for FixedDataPacket {
    type Error = &'a str;

    fn try_from(value: FixedDataLongFrame<'a>) -> Result<Self, Self::Error> {
        todo!()
    }
}
