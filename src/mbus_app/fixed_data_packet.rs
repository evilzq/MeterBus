use crate::mbus_network::{device_type::DeviceType, fixeddata::FixedDataUnits};

pub struct FixedDataPacket {
    pub access_demand: bool,
    pub data_flow_control: bool,
    pub address: u8,

    pub manufactor: u16,
    pub identification_no: u32,
    pub transmission_counter: u8,
    pub device_type: DeviceType,

    pub counters_fixed: bool,
    pub units1: FixedDataUnits,
    pub units2: FixedDataUnits,
    pub counter1: u32,
    pub counter2: u32,
}
