use super::{
    data_type::DataTypes, frame::Part, function::Function, var_data::VariableDataRecordType,
};

pub struct DIF {
    pub data: VariableDataRecordType,
    pub data_type: DataTypes,
    pub function: Function,
    pub storage_lsb: bool,
    pub extension: bool,
}

impl Part for DIF {}

impl DIF {
    pub fn new(data: u8) -> Self {
        Self {
            data: VariableDataRecordType::try_from(data).unwrap(),
            data_type: DataTypes::try_from(data & 0x0f_u8).unwrap(),
            function: Function::try_from(data & 0x30_u8).unwrap(),
            storage_lsb: (data & 0x40) != 0,
            extension: (data & 0x80) != 0,
        }
    }
}
