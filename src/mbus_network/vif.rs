use alloc::string::{String, ToString};

use crate::mbus_network::frame::Part;

use super::{
    dict::{VIFE_FB_TABLE, VIFE_FD_TABLE, VIF_VARIABLE_TABLE},
    unit_prefix::get_unit_prefix,
    var_data::{VariableDataQuantityUnit, VariableDataRecordType},
};

#[derive(Debug, Clone, Copy)]
pub enum VifType {
    PrimaryVIF, // E000 0000b .. E111 1011b
    // The unit and multiplier is taken from the table for primary VIF (chapter 8.4.3).
    PlainTextVIF, // E111 1100b
    /// In case of VIF = 7Ch / FCh the true VIF is represented by the following ASCII string with the length given in the first byte. Please note that the byte order of the characters after the length byte depends on the used byte sequence.This plain text VIF allows the user to code units that are not included in the VIF tables.
    LinearVIFExtensionFD, // FDh and FBh
    LinearVIFExtensionFB, // FDh and FBh
    // In case of VIF = FDh and VIF = FBh the true VIF is given by the next byte and the coding is taken from the table for secondary VIF (chapter 8.4.4). This extends the available VIF�s by another 256 codes.
    AnyVIF, // 7Eh / FEh
    // This VIF-Code can be used in direction master to slave for readout selection of all VIF�s.See chapter 6.4.3.
    ManufacturerSpecific, // 7Fh / FFh
}
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum TimePointMagnitudes {
    Date = 0,
    TimeDate = 1,
}
#[derive(Clone, Copy, Debug)]
pub struct VIF<'a> {
    pub types: VifType,
    pub extension: bool,
    pub units: VariableDataQuantityUnit,
    pub magnitude: i8,
    pub quantity: &'a str,
    pub name: &'a str,
    pub data: u8,
}

impl<'a> Part for VIF<'a> {}
impl<'a> VIF<'a> {
    pub fn new(data: u8) -> Option<Self> {
        let record = VIF_VARIABLE_TABLE
            .iter()
            .find(|x| x.vif == (data & (VariableDataRecordType::MbusDibDifWithoutExtension as u8)));
        record.map(|value| Self {
            types: value.types,
            extension: data & 0x80 != 0,
            units: value.units,
            magnitude: (value.magnitude)(data),
            quantity: value.quantity,
            name: value.name,
            data,
        })
    }
}

pub struct VfiTableRecord<'a> {
    pub vif: u8,
    pub exponent: f64,
    pub quantity: &'a str,
    pub unit: &'a str,
    pub units: VariableDataQuantityUnit,
    pub types: VifType,
    pub magnitude: fn(u8) -> i8,
    pub name: &'a str,
}

pub struct VIFE<'a> {
    pub extension: bool,
    pub units: VariableDataQuantityUnit,
    pub unit: &'a str,
    pub quantity: &'a str,
    pub prefix: String,
    pub magnitude: i8,
    pub data: u8,
}

impl Part for VIFE<'_> {}

impl<'a> VIFE<'a> {
    pub fn new_vife_fd(data: u8) -> Option<Self> {
        let record = VIFE_FD_TABLE.iter().find(|x| {
            x.vife == (data & (VariableDataRecordType::MbusDibDifWithoutExtension as u8))
        });
        record.map(|v| Self {
            extension: (data & 0x80) != 0,
            units: v.units,
            unit: v.unit,
            quantity: v.quantity,
            prefix: get_unit_prefix((v.magnitude)(data)),
            magnitude: (v.magnitude)(data),
            data,
        })
    }

    pub fn new_vife_fb(data: u8) -> Option<Self> {
        let record = VIFE_FB_TABLE.iter().find(|x| {
            x.vifb == (data & (VariableDataRecordType::MbusDibDifWithoutExtension as u8))
        });
        record.map(|v| Self {
            extension: (data & 0x80) != 0,
            units: v.units,
            unit: v.unit,
            quantity: v.quantity,
            prefix: get_unit_prefix((v.magnitude)(data)),
            magnitude: (v.magnitude)(data),
            data,
        })
    }

    pub fn new(data: u8) -> Self {
        let mut entity = Self {
            extension: data & 0x80 != 0,
            units: VariableDataQuantityUnit::ReservedFd71,
            unit: "",
            quantity: "",
            prefix: "".to_string(),
            magnitude: 0,
            data,
        };
        let payload = data & VariableDataRecordType::MbusDibDifWithoutExtension as u8;
        if (0..=0x1F).contains(&payload) {
            entity.units = VariableDataQuantityUnit::ErrorCodesVIFE;
            entity.magnitude = (payload & 0x1F).try_into().unwrap();
        } else if payload == 0x20 {
            entity.units = VariableDataQuantityUnit::PerSecond;
        } else if payload == 0x21 {
            entity.units = VariableDataQuantityUnit::PerMinute;
        } else if payload == 0x22 {
            entity.units = VariableDataQuantityUnit::PerHour;
        } else if payload == 0x23 {
            entity.units = VariableDataQuantityUnit::PerDay;
        } else if payload == 0x24 {
            entity.units = VariableDataQuantityUnit::PerWeek;
        } else if payload == 0x25 {
            entity.units = VariableDataQuantityUnit::PerMonth;
        } else if payload == 0x26 {
            entity.units = VariableDataQuantityUnit::PerYear;
        } else if payload == 0x27 {
            entity.units = VariableDataQuantityUnit::PerRevolutionMeasurement;
        } else if payload == 0x28 {
            entity.units = VariableDataQuantityUnit::IncrementPerInputPulseOnInputChannel0;
        } else if payload == 0x29 {
            entity.units = VariableDataQuantityUnit::IncrementPerInputPulseOnInputChannel1;
        } else if payload == 0x2a {
            entity.units = VariableDataQuantityUnit::IncrementPerOutputPulseOnOutputChannel0;
        } else if payload == 0x2b {
            entity.units = VariableDataQuantityUnit::IncrementPerOutputPulseOnOutputChannel1;
        } else if payload == 0x2c {
            entity.units = VariableDataQuantityUnit::PerLiter;
        } else if payload == 0x2d {
            entity.units = VariableDataQuantityUnit::PerM3;
        } else if payload == 0x2e {
            entity.units = VariableDataQuantityUnit::PerKg;
        } else if payload == 0x2f {
            entity.units = VariableDataQuantityUnit::PerKelvin;
        } else if payload == 0x30 {
            entity.units = VariableDataQuantityUnit::PerKWh;
        } else if payload == 0x31 {
            entity.units = VariableDataQuantityUnit::PerGj;
        } else if payload == 0x32 {
            entity.units = VariableDataQuantityUnit::PerKW;
        } else if payload == 0x33 {
            entity.units = VariableDataQuantityUnit::PerKelvinLiter;
        } else if payload == 0x34 {
            entity.units = VariableDataQuantityUnit::PerVolt;
        } else if payload == 0x35 {
            entity.units = VariableDataQuantityUnit::PerAmpere;
        } else if payload == 0x36 {
            entity.units = VariableDataQuantityUnit::MultipliedBySek;
        } else if payload == 0x37 {
            entity.units = VariableDataQuantityUnit::MultipliedBySekPerV;
        } else if payload == 0x38 {
            entity.units = VariableDataQuantityUnit::MultipliedBySekPerA;
        } else if payload == 0x39 {
            entity.units = VariableDataQuantityUnit::StartDateTimeOf;
        } else if payload == 0x3a {
            entity.units = VariableDataQuantityUnit::UncorrectedUnit;
        } else if payload == 0x3b {
            entity.units = VariableDataQuantityUnit::AccumulationPositive;
        } else if payload == 0x3c {
            entity.units = VariableDataQuantityUnit::AccumulationNegative;
        } else if (0x3d..=0x3f).contains(&payload) {
            entity.units = VariableDataQuantityUnit::ReservedVife3d;
            entity.magnitude = (payload - 0x3d).try_into().unwrap();
        } else if (payload == 0x40) || (payload == 0x48) {
            entity.units = VariableDataQuantityUnit::LimitValue;
            entity.magnitude = ((payload & 0x08) >> 3).try_into().unwrap();
        } else if (payload == 0x41) || (payload == 0x49) {
            entity.units = VariableDataQuantityUnit::NrOfLimitExceeds;
            entity.magnitude = ((payload & 0x08) >> 3).try_into().unwrap();
        } else if (payload & 0x72) == 0x42 {
            entity.units = VariableDataQuantityUnit::DateTimeOfLimitExceed;
            entity.magnitude = (payload & 0x0d).try_into().unwrap();
        } else if (0x50..=0x5f).contains(&payload) {
            entity.units = VariableDataQuantityUnit::DurationOfLimitExceed;
            entity.magnitude = (payload & 0x0f).try_into().unwrap();
        } else if (0x60..=0x67).contains(&payload) {
            entity.units = VariableDataQuantityUnit::DurationOfLimitAbove;
            entity.magnitude = (payload & 0x07).try_into().unwrap();
        } else if (payload & 0x7a) == 0x68 {
            entity.units = VariableDataQuantityUnit::ReservedVife68;
            entity.magnitude = (payload - 0x05).try_into().unwrap();
        } else if (data & 0x7a) == 0x6a {
            entity.units = VariableDataQuantityUnit::DateTimeOfLimitAbove;
            entity.magnitude = (payload & 0x05).try_into().unwrap();
        } else if (0x70..=0x77).contains(&payload) {
            entity.units = VariableDataQuantityUnit::MultiplicativeCorrectionFactor;
            entity.magnitude = ((payload & 0x07) - 6).try_into().unwrap();
        } else if (0x78..=0x7b).contains(&payload) {
            entity.units = VariableDataQuantityUnit::AdditiveCorrectionConstant;
            entity.magnitude = ((payload & 0x03) - 3).try_into().unwrap();
        } else if payload == 0x7c {
            entity.units = VariableDataQuantityUnit::ReservedVife7c;
        } else if payload == 0x7d {
            entity.units = VariableDataQuantityUnit::MultiplicativeCorrectionFactor1000;
            entity.magnitude = 3;
        } else if payload == 0x7e {
            entity.units = VariableDataQuantityUnit::ReservedVife7c;
        } else if payload == 0x7f {
            entity.units = VariableDataQuantityUnit::ManufacturerSpecific;
        } else {
        }
        entity.prefix = get_unit_prefix(entity.magnitude);
        entity
    }
}

pub struct VifeFbTableRecord<'a> {
    pub vifb: u8,
    pub units: VariableDataQuantityUnit,
    pub unit: &'a str,
    pub quantity: &'a str,
    pub magnitude: fn(u8) -> i8,
}

pub struct VifeFeTableRecord<'a> {
    pub vife: u8,
    pub units: VariableDataQuantityUnit,
    pub unit: &'a str,
    pub quantity: &'a str,
    pub magnitude: fn(u8) -> i8,
}
