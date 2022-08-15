use super::{
    ack_frame::AckFrame, control::ControlFrame, long_frame::LongFrame, short_frame::ShortFrame,
    var_data::VariableDataLongFrame,
};

impl<'a> TryFrom<&'a [u8]> for LongFrame<'a> {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for AckFrame {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl<'a> TryFrom<&'a [u8]> for VariableDataLongFrame<'a> {
    type Error = &'static str;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for ShortFrame {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&[u8]> for ControlFrame {
    type Error = &'static str;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        todo!()
    }
}
