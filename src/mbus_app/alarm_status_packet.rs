use super::Packet;

pub struct AlarmStatusPacket {
    pub access_demand: bool,
    pub data_flow_control: bool,
    pub address: u8,

    pub status: u8,
}

impl Packet for AlarmStatusPacket {}
