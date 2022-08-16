use alloc::vec::Vec;
use num_enum::TryFromPrimitive;

use super::{
    control::ControlInformation,
    data_type::DataTypes,
    dif::DIF,
    dife::DIFE,
    frame::Frame,
    function::Function,
    vif::{VIF, VIFE},
};

#[derive(Debug, PartialEq, Eq, TryFromPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum VariableDataRecordType {
    MbusDibDifWithoutExtension = 0x7F,
    MbusDibDifExtensionBit = 0x80,
    MbusDibDifManufacturerSpecific = 0x0F,
    MbusDibDifMoreRecordsFollow = 0x1F,
    MbusDibDifIdleFiller = 0x2F,
}

#[repr(u16)]
#[derive(Debug, Clone, Copy)]
pub enum VariableDataQuantityUnit {
    Undefined,
    EnergyWh,
    EnergyJ,
    VolumeM3,
    MassKg,
    OnTime,
    OperatingTime,
    PowerW,
    PowerJPerH,
    VolumeFlowM3PerH,
    VolumeFlowExtM3PerMin,
    VolumeFlowExtM3PerS,
    MassFlowKgPerH,
    FlowTemperatureC,
    ReturnTemperatureC,
    TemperatureDifferenceK,
    ExternalTemperatureC,
    PressureBar,
    TimePoint,
    UnitsForHCA,
    Reserved,
    AveragingDuration,
    ActualityDuration,
    FabricationNo,
    EnhancedIdentification,
    BusAddress,
    Extension7b,
    VifString, // (length in first byte)
    Extension7d,
    AnyVIF,
    CustomVIF,
    ManufacturerSpecific,
    // VIFE
    ErrorCodesVIFE, // Reserved for object actions (master to slave): see table on page 75
    // or for error codes (slave to master): see table on page 74"
    PerSecond,
    PerMinute,
    PerHour,
    PerDay,
    PerWeek,
    PerMonth,
    PerYear,
    PerRevolutionMeasurement,
    IncrementPerInputPulseOnInputChannel0,
    IncrementPerInputPulseOnInputChannel1,
    IncrementPerOutputPulseOnOutputChannel0,
    IncrementPerOutputPulseOnOutputChannel1,
    PerLiter,
    PerM3,
    PerKg,
    PerKelvin,
    PerKWh,
    PerGj,
    PerKW,
    PerKelvinLiter,
    PerVolt,
    PerAmpere,
    MultipliedBySek,
    MultipliedBySekPerV,
    MultipliedBySekPerA,
    StartDateTimeOf,
    UncorrectedUnit,      //VIF contains uncorrected unit instead of corrected unit
    AccumulationPositive, //Accumulation only if positive contributions
    AccumulationNegative, //Accumulation of abs value only if negative contributions
    ReservedVife3d,       // 0x3d..0x3f - ReservedVIFE
    LimitValue,
    NrOfLimitExceeds,
    DateTimeOfLimitExceed,
    DurationOfLimitExceed,
    DurationOfLimitAbove,
    ReservedVife68,
    DateTimeOfLimitAbove,
    MultiplicativeCorrectionFactor,
    AdditiveCorrectionConstant,
    ReservedVife7c,
    MultiplicativeCorrectionFactor1000,
    ReservedVife7e,
    //ManufacturerSpecific,
    // VIFE_FB
    EnergyMWh,
    ReservedVifeFb02,
    ReservedVifeFb04,
    EnergyGJ,
    ReservedVifeFb0a,
    ReservedVifeFb0c,
    //Volume_m3,
    ReservedVifeFb12,
    ReservedVifeFb14,
    MassT,
    ReservedVifeFb1a,
    VolumeFeet3,
    VolumeAmericanGallon,
    VolumeFlowAmericanGallonPerMin,
    VolumeFlowAmericanGallonPerH,
    ReservedVifeFb27,
    PowerMw,
    ReservedVifeFb2a,
    ReservedVifeFb2c,
    PowerGjPerH,
    ReservedVifeFb32,
    FlowTemperatureF,
    ReturnTemperatureF,
    TemperatureDifferenceF,
    ExternalTemperatureF,
    ReservedVifeFb68,
    ColdWarmTemperatureLimitF,
    ColdWarmTemperatureLimitC,
    CumulCountMaxPowerW,
    // VIFE_FD
    Credit,       // Credit of 10nn-3 of the nominal local legal currency units
    Debit,        // Debit of 10nn-3 of the nominal local legal currency units
    AccessNumber, // Access Number (transmission count)
    Medium,       // Medium (as in fixed header)
    Manufacturer, // Manufacturer (as in fixed header)
    //EnhancedIdentification, // Parameter set identification
    ModelVersion, // Model / Version
    HardwareVersionNr,
    FirmwareVersionNr,
    SoftwareVersionNr,
    CustomerLocation,
    Customer,
    AccessCodeUser,
    AccessCodeOperator,
    AccessCodeSystemOperator,
    AccessCodeDeveloper,
    Password,
    ErrorFlags, // (binary)
    ErrorMask,
    ReservedVifeFd19,
    DigitalOutput,     // (binary)
    DigitalInput,      // (binary)
    Baudrate,          // [Baud]
    ResponseDelayTime, // [bittimes]
    Retry,
    ReservedVifeFd1f,
    FirstStorageNr, // for cyclic storage
    LastStorageNr,  // for cyclic storage
    SizeOfStorage,  // Size of storage block
    ReservedVifeFd23,
    StorageInterval,      // [sec(s)..day(s)]
    StorageIntervalMmnth, // month(s)
    StorageIntervalYear,  // year(s)
    ReservedVifeFd2a,
    ReservedVifeFd2b,
    DurationSinceLastReadout, // [sec(s)..day(s)]
    StartDateTimeOfTariff,
    DurationOfTariff,     // (nn = 01..11: min to days)
    PeriodOfTariff,       // [sec(s) to day(s)]
    PeriodOfTariffMonths, // months(s)
    PeriodOfTariffYear,   // year(s)
    Dimensionless,        // no VIF
    ReservedFd3b,
    ReservedFd3c,
    Volts,  // 10nnnn-9
    Ampers, // 10nnnn-12
    ResetCounter,
    CumulationCounter,
    ControlSignal,
    DayOfWeek,
    WeekNumber,
    TimePointOfDayChange,
    StateOfParameterActivation,
    SpecialSupplierInformation,
    DurationSinceLastCumulation, // [hour(s)..years(s)]
    OperatingTimeBattery,        // [hour(s)..years(s)]
    DateTimeOfBatteryChange,
    ReservedFd71,
}

pub struct VariableDataLongParts<'a> {
    pub di_field: DIF,
    pub dife_field: Option<Vec<DIFE>>,
    pub vi_field: Option<VIF<'a>>,
    pub vife_field: Option<Vec<VIFE<'a>>>,
    pub data: Option<Vec<u8>>,
}

pub struct VariableDataLongFrame<'a> {
    pub control: u8,
    pub address: u8,
    pub control_infomation: ControlInformation,
    pub indentification_no: [u8; 4],
    pub device_type: u8,
    pub transmission_counter: u8,
    pub status: u8,
    pub length: u8,
    pub manufacterer: [u8; 2],
    pub version: u8,
    pub signature: [u8; 2],
    pub parts: Vec<VariableDataLongParts<'a>>,
}

impl Frame for VariableDataLongFrame<'_> {}

impl<'a> VariableDataLongFrame<'a> {
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
        data: Vec<u8>,
        length: u8,
    ) -> Option<Self> {
        let mut iter = data.iter();
        if (ControlInformation::RespVariable as u8) != control_infomation {
            return None;
        }
        let mut entity = Self {
            control,
            address,
            control_infomation: ControlInformation::try_from(control_infomation).unwrap(),
            indentification_no: [0, 0, 0, 0],
            device_type: 0,
            transmission_counter: 0,
            status: 0,
            length,
            manufacterer: [0, 0],
            version: 0,
            signature: [0, 0],
            parts: Vec::new(),
        };
        let mut i = 0;
        while i < entity.indentification_no.len() {
            let u8_byte = iter.next();
            match u8_byte {
                Some(value) => entity.indentification_no[i] = *value,
                None => {
                    return None;
                }
            }
            i += 1;
        }
        i = 0;
        while i < entity.manufacterer.len() {
            let u8_byte = iter.next();
            match u8_byte {
                Some(value) => entity.manufacterer[i] = *value,
                None => {
                    return None;
                }
            };
            i += 1;
        }

        match iter.next() {
            Some(value) => entity.version = *value,
            None => {
                return None;
            }
        };

        match iter.next() {
            Some(value) => entity.device_type = *value,
            None => {
                return None;
            }
        };

        match iter.next() {
            Some(value) => entity.transmission_counter = *value,
            None => {
                return None;
            }
        };

        match iter.next() {
            Some(value) => entity.status = *value,
            None => {
                return None;
            }
        };

        i = 0;
        while i < entity.signature.len() {
            let u8_byte = iter.next();
            match u8_byte {
                Some(value) => entity.signature[i] = *value,
                None => {
                    return None;
                }
            };
            i += 1;
        }

        while let Some(raw_value) = iter.next() {
            if let Ok(value) = VariableDataRecordType::try_from(*raw_value) {
                match value {
                    VariableDataRecordType::MbusDibDifIdleFiller => {
                        entity.parts.push(VariableDataLongParts {
                            di_field: DIF::new(*raw_value),
                            dife_field: None,
                            vi_field: None,
                            vife_field: None,
                            data: None,
                        });
                    }
                    VariableDataRecordType::MbusDibDifManufacturerSpecific => {
                        let mut data_vec: Vec<u8> = Vec::new();
                        for a in iter.by_ref() {
                            data_vec.push(*a);
                        }
                        entity.parts.push(VariableDataLongParts {
                            di_field: DIF {
                                data: value,
                                data_type: DataTypes::NoData,
                                function: Function::Instantaneous,
                                storage_lsb: false,
                                extension: false,
                            },
                            dife_field: None,
                            vi_field: None,
                            vife_field: None,
                            data: Some(data_vec),
                        });
                    }
                    VariableDataRecordType::MbusDibDifMoreRecordsFollow => {
                        entity.parts.push(VariableDataLongParts {
                            di_field: DIF::new(*raw_value),
                            dife_field: None,
                            vi_field: None,
                            vife_field: None,
                            data: None,
                        });
                    }
                    _ => {
                        let dif = DIF::new(*raw_value);
                        let mut dife_vec: Vec<DIFE> = Vec::new();
                        let mut vife_vec: Vec<VIFE> = Vec::new();
                        let mut data_vec: Vec<u8> = Vec::new();

                        let mut ext_flag = dif.extension;
                        let mut i = 0;
                        loop {
                            if (i >= 10) | !ext_flag {
                                break;
                            }
                            match iter.next() {
                                Some(value) => {
                                    let dife = DIFE::new(*value);
                                    ext_flag = dife.extension;
                                    dife_vec.push(dife);
                                }
                                None => return None,
                            }
                            i += 1;
                        }
                        #[allow(clippy::needless_late_init)]
                        let vif: Option<VIF>;
                        match iter.next() {
                            Some(value) => vif = VIF::new(*value),
                            None => return None,
                        };
                        if let Some(vf) = vif {
                            ext_flag = vf.extension;
                            i = 0;
                            loop {
                                if (i >= 10) | !ext_flag {
                                    break;
                                }
                                match vf.types {
                                    super::vif::VifType::PrimaryVIF => match iter.next() {
                                        Some(value) => {
                                            let vife = VIFE::new(*value);
                                            ext_flag = vife.extension;
                                            vife_vec.push(vife);
                                        }
                                        None => return None,
                                    },
                                    super::vif::VifType::PlainTextVIF => match iter.next() {
                                        Some(value) => {
                                            let vife = VIFE::new(*value);
                                            ext_flag = vife.extension;
                                            vife_vec.push(vife);
                                        }
                                        None => return None,
                                    },
                                    super::vif::VifType::LinearVIFExtensionFD => {
                                        match iter.next() {
                                            Some(value) => {
                                                let vife = VIFE::new_vife_fd(*value);
                                                if let Some(v) = vife {
                                                    ext_flag = v.extension;
                                                    vife_vec.push(v);
                                                }
                                            }
                                            None => return None,
                                        }
                                    }
                                    super::vif::VifType::LinearVIFExtensionFB => {
                                        match iter.next() {
                                            Some(value) => {
                                                let vife = VIFE::new_vife_fb(*value);
                                                if let Some(v) = vife {
                                                    ext_flag = v.extension;
                                                    vife_vec.push(v);
                                                }
                                            }
                                            None => return None,
                                        }
                                    }
                                    super::vif::VifType::AnyVIF => match iter.next() {
                                        Some(value) => {
                                            let vife = VIFE::new(*value);
                                            ext_flag = vife.extension;
                                            vife_vec.push(vife);
                                        }
                                        None => return None,
                                    },
                                    super::vif::VifType::ManufacturerSpecific => {
                                        match iter.next() {
                                            Some(value) => {
                                                let vife = VIFE::new(*value);
                                                ext_flag = vife.extension;
                                                vife_vec.push(vife);
                                            }
                                            None => return None,
                                        }
                                    }
                                }
                                i += 1;
                            }
                        }
                        if dif.data_type != DataTypes::VariableLength {
                            let length =
                                VariableDataLongFrame::get_length_in_bit_table(dif.data_type) / 8;
                            i = 0;
                            while i < length {
                                match iter.next() {
                                    Some(v) => data_vec.push(*v),
                                    None => return None,
                                }
                                i += 1;
                            }
                        }
                        entity.parts.push(VariableDataLongParts {
                            di_field: dif,
                            dife_field: Some(dife_vec),
                            vi_field: vif,
                            vife_field: Some(vife_vec),
                            data: Some(data_vec),
                        })
                    }
                }
            }
        }
        Some(entity)
    }
}
