use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum Function {
    Instantaneous = 0x00, //Instantaneous value

    Maximum = 0x10, //Maximum value

    Minimum = 0x20, //Minimum value

    ValueDuringError = 0x30, //Value during error state
}
