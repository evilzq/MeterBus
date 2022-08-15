use alloc::vec::Vec;

use crate::mbus_network::{
    data_type::DataTypes,
    device_type::DeviceType,
    function::Function,
    var_data::{VariableDataQuantityUnit, VariableDataRecordType},
};

pub struct UnitData<'a> {
    pub units: VariableDataQuantityUnit,
    pub unit: &'a str,
    pub magnitude: i8,
    pub quantity: &'a str,
    pub vif_string: &'a str,
}

pub struct Record<'a> {
    pub record_type: VariableDataRecordType,
    pub function: Function,
    pub storage_number: u64,
    pub traiff: u32,
    pub sub_unit: u16,
    pub value_data_type: DataTypes,
    pub units: Vec<UnitData<'a>>,
    pub magnitide: i8,
    pub offset: i32,
}

pub struct VariableDataPacket<'a> {
    pub access_demand: bool,
    pub data_flow_control: bool,
    pub address: u8,

    pub manufactor: u16,
    pub identification_no: u32,
    pub transmission_counter: u8,
    pub device_type: DeviceType,

    pub version: u8,
    pub status: u8,
    pub signature: u16,
    pub records: Vec<Record<'a>>,
}
