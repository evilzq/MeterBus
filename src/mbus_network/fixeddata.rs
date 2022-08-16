use num_enum::TryFromPrimitive;

use super::{control::ControlInformation, frame::Frame};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum FixedDataUnits {
    Hms = 0x0,              // h,m,s
    DMY = 0x1,              // D,M,Y
    Wh = 0x2,               // Wh
    Wh10 = 0x3,             // Wh * 10
    Wh100 = 0x4,            // Wh * 100
    KWh = 0x5,              // kWh
    KWh10 = 0x6,            // kWh * 10
    KWh100 = 0x7,           // kWh * 100
    MWh = 0x8,              // MWh
    MWh10 = 0x9,            // MWh * 10
    MWh100 = 0x0A,          // MWh * 100
    KJ = 0x0B,              // kJ
    KJ10 = 0x0C,            // kJ * 10
    KJ100 = 0x0D,           // kJ * 100
    MJ = 0x0E,              // MJ
    MJ10 = 0x0F,            // MJ * 10
    MJ100 = 0x10,           // MJ * 100
    GJ = 0x11,              // GJ
    GJ10 = 0x12,            // GJ * 10
    GJ100 = 0x13,           // GJ * 100
    W = 0x14,               // W
    W10 = 0x15,             // W * 10
    W100 = 0x16,            // W * 100
    KW = 0x17,              // kW
    KW10 = 0x18,            // kW * 10
    KW100 = 0x19,           // kW * 100
    MW = 0x1A,              // MW
    MW10 = 0x1B,            // MW * 10
    MW100 = 0x1C,           // MW * 100
    KJPerH = 0x1D,          // kJ/h
    KJPerH10 = 0x1E,        // kJ/h * 10
    KJPerH100 = 0x1F,       // kJ/h * 100
    MjPerH = 0x20,          // MJ/h
    MjPerH10 = 0x21,        // MJ/h * 10
    MjPerH100 = 0x22,       // MJ/h * 100
    GjPerH = 0x23,          // GJ/h
    GjPerH10 = 0x24,        // GJ/h * 10
    GjPerH100 = 0x25,       // GJ/h * 100
    Ml = 0x26,              // ml
    Ml10 = 0x27,            // ml * 10
    Ml100 = 0x28,           // ml * 100
    L = 0x29,               // l
    L10 = 0x2A,             // l * 10
    L100 = 0x2B,            // l * 100
    M3 = 0x2C,              // m3
    M310 = 0x2D,            // m3 * 10
    M3100 = 0x2E,           // m3 * 100
    MlPerH = 0x2F,          // ml/h
    MlPerH10 = 0x30,        // ml/h * 10
    MlPerH100 = 0x31,       // ml/h * 100
    LPerH = 0x32,           // l/h
    LPerH10 = 0x33,         // l/h * 10
    LPerH100 = 0x34,        // l/h * 100
    M3PerH = 0x35,          // m3/h
    M3PerH10 = 0x36,        // m3/h * 10
    M3PerH100 = 0x37,       // m3/h * 100
    C001 = 0x38,            // �C * 10-3
    UnitsForHCA = 0x39,     // units for HCA
    Reserved3a = 0x3A,      // reserved
    Reserved3b = 0x3B,      // reserved
    Reserved3c = 0x3C,      // reserved
    Reserved3d = 0x3D,      // reserved
    SameButHistoric = 0x3E, // same but historic
    WithoutUnits = 0x3F,    // without units
}

pub struct FixedDataLongFrame<'a> {
    pub control: u8,
    pub address: u8,
    pub control_infomation: ControlInformation,
    pub indentification_no: [u8; 4],
    pub device_type: u8,
    pub transmission_counter: u8,
    pub status: u8,
    pub data: &'a [u8],
    pub length: u8,

    pub counters_fixed: bool,
    pub units1: FixedDataUnits,
    pub units2: FixedDataUnits,
    pub counter1: u32,
    pub counter2: u32,
}

impl Frame for FixedDataLongFrame<'_> {}
