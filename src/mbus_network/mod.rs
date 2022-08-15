pub mod ack_frame;
pub mod control;
pub mod data_type;
pub mod device_type;
pub mod dict;
pub mod dif;
pub mod dife;
pub mod fixeddata;
pub mod frame;
pub mod function;
pub mod long_frame;
pub mod manufacturer;
pub mod meterbus_frame_serializer;
pub mod short_frame;
pub mod unit_prefix;
pub mod var_data;
pub mod vif;

pub const MBUS_FRAME_TYPE_ANY: u8 = 0x00;
pub const MBUS_FRAME_TYPE_ACK: u8 = 0x01;
pub const MBUS_FRAME_TYPE_SHORT: u8 = 0x02;
pub const MBUS_FRAME_TYPE_CONTROL: u8 = 0x03;
pub const MBUS_FRAME_TYPE_LONG: u8 = 0x04;

pub const MBUS_FRAME_ACK_BASE_SIZE: u8 = 1;
pub const MBUS_FRAME_SHORT_BASE_SIZE: u8 = 5;
pub const MBUS_FRAME_CONTROL_BASE_SIZE: u8 = 9;
pub const MBUS_FRAME_LONG_BASE_SIZE: u8 = 9;

pub const MBUS_FRAME_BASE_SIZE_ACK: u8 = 1;
pub const MBUS_FRAME_BASE_SIZE_SHORT: u8 = 5;
pub const MBUS_FRAME_BASE_SIZE_CONTROL: u8 = 9;
pub const MBUS_FRAME_BASE_SIZE_LONG: u8 = 9;

pub const MBUS_FRAME_FIXED_SIZE_ACK: u8 = 1;
pub const MBUS_FRAME_FIXED_SIZE_SHORT: u8 = 5;
pub const MBUS_FRAME_FIXED_SIZE_CONTROL: u8 = 6;
pub const MBUS_FRAME_FIXED_SIZE_LONG: u8 = 6;

//
// Frame start/stop bits
//
pub const MBUS_FRAME_ACK_START: u8 = 0xE5;
pub const MBUS_FRAME_SHORT_START: u8 = 0x10;
pub const MBUS_FRAME_CONTROL_START: u8 = 0x68;
pub const MBUS_FRAME_LONG_START: u8 = 0x68;
pub const MBUS_FRAME_STOP: u8 = 0x16;

//
//
//
pub const MBUS_MAX_PRIMARY_SLAVES: u8 = 250;

//
// Control field
//
pub const MBUS_CONTROL_FIELD_DIRECTION: u8 = 0x07;
pub const MBUS_CONTROL_FIELD_FCB: u8 = 0x06;
pub const MBUS_CONTROL_FIELD_ACD: u8 = 0x06;
pub const MBUS_CONTROL_FIELD_FCV: u8 = 0x05;
pub const MBUS_CONTROL_FIELD_DFC: u8 = 0x05;
pub const MBUS_CONTROL_FIELD_F3: u8 = 0x04;
pub const MBUS_CONTROL_FIELD_F2: u8 = 0x03;
pub const MBUS_CONTROL_FIELD_F1: u8 = 0x02;
pub const MBUS_CONTROL_FIELD_F0: u8 = 0x01;

pub const MBUS_CONTROL_MASK_SND_NKE: u8 = 0x40; // Initialization of Slave                                  (SHORT FRAME)
pub const MBUS_CONTROL_MASK_SND_UD: u8 = 0x53; // Send User Data to Slave                                  (LONG/CONTROL FRAME)
pub const MBUS_CONTROL_MASK_REQ_UD2: u8 = 0x7B; // Request for Class 2 Data: 0x4b | 0x5b | 0x6b | 0x7b      (SHORT FRAME)
pub const MBUS_CONTROL_MASK_REQ_UD1: u8 = 0x7A; // Request for Class 1 Data: 0x5a | 0x7a                     (SHORT FRAME)
pub const MBUS_CONTROL_MASK_RSP_UD: u8 = 0x08; // Data Transfer from Slave: 08 | 18 | 28 | 38              (LONG/CONTROL FRAME)

pub const MBUS_CONTROL_MASK_FCB: u8 = 0x20;
pub const MBUS_CONTROL_MASK_FCV: u8 = 0x10;

pub const MBUS_CONTROL_MASK_ACD: u8 = 0x20;
pub const MBUS_CONTROL_MASK_DFC: u8 = 0x10;

pub const MBUS_CONTROL_MASK_DIR: u8 = 0x40;
pub const MBUS_CONTROL_MASK_DIR_M2S: u8 = 0x40;
pub const MBUS_CONTROL_MASK_DIR_S2M: u8 = 0x00;

//
// Address field
//
pub const MBUS_ADDRESS_BROADCAST_REPLY: u8 = 0xFE;
pub const MBUS_ADDRESS_BROADCAST_NOREPLY: u8 = 0xFF;
pub const MBUS_ADDRESS_NETWORK_LAYER: u8 = 0xFD;

//
// Control Information field
//
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

pub const MBUS_CONTROL_INFO_DATA_SEND: u8 = 0x51;
pub const MBUS_CONTROL_INFO_DATA_SEND_MSB: u8 = 0x55;
pub const MBUS_CONTROL_INFO_SELECT_SLAVE: u8 = 0x52;
pub const MBUS_CONTROL_INFO_SELECT_SLAVE_MSB: u8 = 0x56;
pub const MBUS_CONTROL_INFO_APPLICATION_RESET: u8 = 0x50;
pub const MBUS_CONTROL_INFO_SYNC_ACTION: u8 = 0x54;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_300: u8 = 0xB8;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_600: u8 = 0xB9;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_1200: u8 = 0xBA;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_2400: u8 = 0xBB;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_4800: u8 = 0xBC;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_9600: u8 = 0xBD;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_19200: u8 = 0xBE;
pub const MBUS_CONTROL_INFO_SET_BAUDRATE_38400: u8 = 0xBF;
pub const MBUS_CONTROL_INFO_REQUEST_RAM_READ: u8 = 0xB1;
pub const MBUS_CONTROL_INFO_SEND_USER_DATA: u8 = 0xB2;
pub const MBUS_CONTROL_INFO_INIT_TEST_CALIB: u8 = 0xB3;
pub const MBUS_CONTROL_INFO_EEPROM_READ: u8 = 0xB4;
pub const MBUS_CONTROL_INFO_SW_TEST_START: u8 = 0xB6;

//Mode 1 Mode 2                   Application                   Definition in
// 70h             report of general application errors     Usergroup March 94
// 71h                      report of alarm status          Usergroup March 94
// 72h   76h                variable data respond                EN1434-3
// 73h   77h                 fixed data respond                  EN1434-3
pub const MBUS_CONTROL_INFO_ERROR_GENERAL: u8 = 0x70;
pub const MBUS_CONTROL_INFO_STATUS_ALARM: u8 = 0x71;

pub const MBUS_CONTROL_INFO_RESP_FIXED: u8 = 0x73;
pub const MBUS_CONTROL_INFO_RESP_FIXED_MSB: u8 = 0x77;

pub const MBUS_CONTROL_INFO_RESP_VARIABLE: u8 = 0x72;
pub const MBUS_CONTROL_INFO_RESP_VARIABLE_MSB: u8 = 0x76;

//
// DATA BITS
//
pub const MBUS_DATA_FIXED_STATUS_FORMAT_MASK: u8 = 0x80;
pub const MBUS_DATA_FIXED_STATUS_FORMAT_BCD: u8 = 0x00;
pub const MBUS_DATA_FIXED_STATUS_FORMAT_: u8 = 0x80;
pub const MBUS_DATA_FIXED_STATUS_DATE_MASK: u8 = 0x40;
pub const MBUS_DATA_FIXED_STATUS_DATE_STORED: u8 = 0x40;
pub const MBUS_DATA_FIXED_STATUS_DATE_CURRENT: u8 = 0x00;

//
// data record fields
//
pub const MBUS_DATA_RECORD_DIF_MASK_INST: u8 = 0x00;
pub const MBUS_DATA_RECORD_DIF_MASK_MIN: u8 = 0x10;

pub const MBUS_DATA_RECORD_DIF_MASK_TYPE_32: u8 = 0x04;
pub const MBUS_DATA_RECORD_DIF_MASK_DATA: u8 = 0x0F;
pub const MBUS_DATA_RECORD_DIF_MASK_FUNCTION: u8 = 0x30;
pub const MBUS_DATA_RECORD_DIF_MASK_STORAGE_NO: u8 = 0x40;
pub const MBUS_DATA_RECORD_DIF_MASK_EXTENTION: u8 = 0x80;
pub const MBUS_DATA_RECORD_DIF_MASK_NON_DATA: u8 = 0xF0;

pub const MBUS_DATA_RECORD_DIFE_MASK_STORAGE_NO: u8 = 0x0F;
pub const MBUS_DATA_RECORD_DIFE_MASK_TARIFF: u8 = 0x30;
pub const MBUS_DATA_RECORD_DIFE_MASK_DEVICE: u8 = 0x40;
pub const MBUS_DATA_RECORD_DIFE_MASK_EXTENSION: u8 = 0x80;

//
// GENERAL APPLICATION ERRORS
//
pub const MBUS_ERROR_DATA_UNSPECIFIED: u8 = 0x00;
pub const MBUS_ERROR_DATA_UNIMPLEMENTED_CI: u8 = 0x01;
pub const MBUS_ERROR_DATA_BUFFER_TOO_LONG: u8 = 0x02;
pub const MBUS_ERROR_DATA_TOO_MANY_RECORDS: u8 = 0x03;
pub const MBUS_ERROR_DATA_PREMATURE_END: u8 = 0x04;
pub const MBUS_ERROR_DATA_TOO_MANY_DIFES: u8 = 0x05;
pub const MBUS_ERROR_DATA_TOO_MANY_VIFES: u8 = 0x06;
pub const MBUS_ERROR_DATA_RESERVED: u8 = 0x07;
pub const MBUS_ERROR_DATA_APPLICATION_BUSY: u8 = 0x08;
pub const MBUS_ERROR_DATA_TOO_MANY_READOUTS: u8 = 0x09;

//
// FIXED DATA FLAGS
//

//
// VARIABLE DATA FLAGS
//
pub const MBUS_VARIABLE_DATA_MEDIUM_OTHER: u8 = 0x00;
pub const MBUS_VARIABLE_DATA_MEDIUM_OIL: u8 = 0x01;
pub const MBUS_VARIABLE_DATA_MEDIUM_ELECTRICITY: u8 = 0x02;
pub const MBUS_VARIABLE_DATA_MEDIUM_GAS: u8 = 0x03;
pub const MBUS_VARIABLE_DATA_MEDIUM_HEAT_OUT: u8 = 0x04;
pub const MBUS_VARIABLE_DATA_MEDIUM_STEAM: u8 = 0x05;
pub const MBUS_VARIABLE_DATA_MEDIUM_HOT_WATER: u8 = 0x06;
pub const MBUS_VARIABLE_DATA_MEDIUM_WATER: u8 = 0x07;
pub const MBUS_VARIABLE_DATA_MEDIUM_HEAT_COST: u8 = 0x08;
pub const MBUS_VARIABLE_DATA_MEDIUM_COMPR_AIR: u8 = 0x09;
pub const MBUS_VARIABLE_DATA_MEDIUM_COOL_OUT: u8 = 0x0A;
pub const MBUS_VARIABLE_DATA_MEDIUM_COOL_IN: u8 = 0x0B;
pub const MBUS_VARIABLE_DATA_MEDIUM_HEAT_IN: u8 = 0x0C;
pub const MBUS_VARIABLE_DATA_MEDIUM_HEAT_COOL: u8 = 0x0D;
pub const MBUS_VARIABLE_DATA_MEDIUM_BUS: u8 = 0x0E;
pub const MBUS_VARIABLE_DATA_MEDIUM_UNKNOWN: u8 = 0x0F;
pub const MBUS_VARIABLE_DATA_MEDIUM_COLD_WATER: u8 = 0x16;
pub const MBUS_VARIABLE_DATA_MEDIUM_DUAL_WATER: u8 = 0x17;
pub const MBUS_VARIABLE_DATA_MEDIUM_PRESSURE: u8 = 0x18;
pub const MBUS_VARIABLE_DATA_MEDIUM_ADC: u8 = 0x19;
