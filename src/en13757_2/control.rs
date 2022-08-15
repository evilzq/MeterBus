use num_enum::TryFromPrimitive;

use crate::checksum;

use super::{frame::Frame, MBUS_FRAME_CONTROL_START, MBUS_FRAME_STOP};

/// <summary>
/// Initialization of Slave                                  (SHORT FRAME)
/// </summary>
pub const SND_NKE: u8 = 0x40;

/// <summary>
/// Send User Data to Slave                                 (LONG/CONTROL FRAME)
/// </summary>
pub const SND_UD: u8 = 0x53;

/// <summary>
/// Request for Class 2 Data: 0x4b | 0x5b | 0x6b | 0x7b      (SHORT FRAME)
/// </summary>
pub const REQ_UD2: u8 = 0x7b;
/// <summary>
/// Request for Class 1 Data: 0x5a | 0x7a                    (SHORT FRAME)
/// </summary>
pub const REQ_UD1: u8 = 0x7a;

/// <summary>
/// Data Transfer from Slave: 0x08 | 0x18 | 0x28 | 0x38      (LONG/CONTROL FRAME)
/// </summary>
pub const RSP_UD: u8 = 0x08;

pub const FCB: u8 = 0x20;
pub const FCV: u8 = 0x10;

pub const ACD: u8 = 0x20;
pub const DFC: u8 = 0x10;

pub const DIR: u8 = 0x40;
pub const DIR_M2S: u8 = 0x40;
pub const DIR_S2M: u8 = 0x00;

#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
#[repr(u8)]
pub enum ControlInformation {
    //Mode 1 Mode 2                   Application                   Definition in
    // 51h    55h                       data send                    EN1434-3
    // 52h    56h                  selection of slaves           Usergroup July  ́93
    // 50h                          application reset           Usergroup March  ́94
    // 54h                          synronize action                 suggestion
    // B8h                     set baudrate to 300 baud          Usergroup July  ́93
    // B9h                     set baudrate to 600 baud          Usergroup July  ́93
    // BAh                    set baudrate to 1200 baud          Usergroup July  ́93
    // BBh                    set baudrate to 2400 baud          Usergroup July  ́93
    // BCh                    set baudrate to 4800 baud          Usergroup July  ́93
    // BDh                    set baudrate to 9600 baud          Usergroup July  ́93
    // BEh                   set baudrate to 19200 baud              suggestion
    // BFh                   set baudrate to 38400 baud              suggestion
    // B1h           request readout of complete RAM content     Techem suggestion
    // B2h          send user data (not standardized RAM write) Techem suggestion
    // B3h                 initialize test calibration mode      Usergroup July  ́93
    // B4h                           EEPROM read                 Techem suggestion
    // B6h                         start software test           Techem suggestion
    // 90h to 97h              codes used for hashing           longer recommended
    DataSend = 0x51,
    DataSendMsb = 0x55,
    SelectSlave = 0x52,
    SelectSlaveMsb = 0x56,
    ApplicationReset = 0x50,
    SyncAction = 0x54,
    SetBaudrate300 = 0xB8,
    SetBaudrate600 = 0xB9,
    SetBaudrate1200 = 0xBA,
    SetBaudrate2400 = 0xBB,
    SetBaudrate4800 = 0xBC,
    SetBaudrate9600 = 0xBD,
    SetBaudrate19200 = 0xBE,
    SetBaudrate38400 = 0xBF,
    RequestRamRead = 0xB1,
    SendUserData = 0xB2,
    InitTestCalib = 0xB3,
    EepromRead = 0xB4,
    SwTestStart = 0xB6,

    //Mode 1 Mode 2                   Application                   Definition in
    // 70h             report of general application errors     Usergroup March 94
    // 71h                      report of alarm status          Usergroup March 94
    // 72h   76h                variable data respond                EN1434-3
    // 73h   77h                 fixed data respond                  EN1434-3
    ErrorGeneral = 0x70,
    StatusAlarm = 0x71,

    RespFixed = 0x73,
    RespFixedMsb = 0x77,

    RespVariable = 0x72,
    RespVariableMsb = 0x76,
    #[num_enum(default)]
    Other = 0xFF,
}

pub struct ControlFrame {
    start: u8,
    control: u8,
    control_infomation: ControlInformation,
    address: u8,
    length: u8,
    crc: u8,
    stop: u8,
}

impl Frame for ControlFrame {}

impl ControlFrame {
    pub fn new(control: u8, control_information: u8, address: u8) -> Self {
        Self {
            start: MBUS_FRAME_CONTROL_START,
            control,
            control_infomation: ControlInformation::try_from(control_information).unwrap(),
            address,
            length: 0x03,
            crc: checksum(&[control, address, control_information]),
            stop: MBUS_FRAME_STOP,
        }
    }
}
