use alloc::string::String;

use crate::mbus_network::frame::Part;

use super::var_data::VariableDataQuantityUnit;

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
pub enum TimePointMagnitudes {
    Date = 0,
    TimeDate = 1,
}

pub struct VIF<'a> {
    pub types: VifType,
    pub extension: bool,
    pub units: VariableDataQuantityUnit,
    pub magnitude: i8,
    pub quantity: &'a str,
    pub vif_string: &'a str,
    pub name: &'a str,
    pub data: u8,
}

impl<'a> Part for VIF<'a> {}
impl<'a> VIF<'a> {
    pub fn new(data: u8) -> Self {
        Self {
            types: todo!(),
            extension: todo!(),
            units: todo!(),
            magnitude: todo!(),
            quantity: todo!(),
            vif_string: todo!(),
            name: todo!(),
            data: todo!(),
        }
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
    pub prefix: &'a str,
    pub magnitude: i8,
    pub data: u8,
}

impl Part for VIFE<'_> {}

impl<'a> VIFE<'a> {
    pub fn new(data: u8) -> Self {
        Self {
            extension: todo!(),
            units: todo!(),
            unit: todo!(),
            quantity: todo!(),
            prefix: todo!(),
            magnitude: todo!(),
            data: todo!(),
        }
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
