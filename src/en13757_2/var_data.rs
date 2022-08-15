use alloc::vec::Vec;
use num_enum::TryFromPrimitive;

use crate::checksum;

use super::{
    control::ControlInformation,
    data_type::DataTypes,
    dif::DIF,
    dife::DIFE,
    vif::{VIF, VIFE},
    MBUS_FRAME_LONG_START, MBUS_FRAME_STOP,
};

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum VariableDataRecordType {
    MbusDibDifWithoutExtension = 0x7F,
    MbusDibDifExtensionBit = 0x80,
    MbusDibDifManufacturerSpecific = 0x0F,
    MbusDibDifMoreRecordsFollow = 0x1F,
    MbusDibDifIdleFiller = 0x2F,
}

#[repr(u16)]
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
    pub vi_field: VIF<'a>,
    pub vife_field: Option<Vec<VIFE<'a>>>,
    pub data: Option<Vec<u8>>,
}

pub struct VariableDataLongFrame<'a> {
    pub strat: u8,
    pub control: u8,
    pub address: u8,
    pub control_infomation: ControlInformation,
    pub indentification_no: [u8; 4],
    pub device_type: u8,
    pub transmission_counter: u8,
    pub status: u8,
    pub data: &'a [u8],
    pub length: u8,
    pub crc: u8,
    pub stop: u8,
    pub manufacterer: [u8; 2],
    pub version: u8,
    pub signature: [u8; 2],
    pub parts: Vec<VariableDataLongParts<'a>>,
}

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
            manufacterer: todo!(),
            version: todo!(),
            signature: todo!(),
            parts: todo!(),
        }
    }
}
