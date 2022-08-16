use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Clone, Copy)]
pub enum DataTypes {
    NoData = 0,              // No data
    _8BitInteger = 1,        // 8 Bit Integer
    _16BitInteger = 2,       // 16 Bit Integer
    _24BitInteger = 3,       // 24 Bit Integer
    _32BitInteger = 4,       // 32 Bit Integer
    _32BitReal = 5,          // 32 Bit Real
    _48BitInteger = 6,       // 48 Bit Integer
    _64BitInteger = 7,       // 64 Bit Integer
    SelectionForReadout = 8, // Selection for Readout
    _2DigitBcd = 9,          // 2 digit BCD
    _4DigitBcd = 10,         // 4 digit BCD
    _6DigitBcd = 11,         // 6 digit BCD
    _8DigitBcd = 12,         // 8 digit BCD
    VariableLength = 13,     // variable length
    _12DigitBcd = 14,        // 12 digit BCD
    #[num_enum(default)]
    Unknown = 15, // Special Functions
}
