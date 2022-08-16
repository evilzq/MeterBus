use alloc::{string::ToString, vec::Vec};

use crate::mbus_network::{device_type::DeviceType, var_data::VariableDataLongFrame};

use super::variable_data_packet::{Record, UnitData, VariableDataPacket};

impl<'a> TryFrom<VariableDataLongFrame<'a>> for VariableDataPacket {
    type Error = &'a str;

    fn try_from(value: VariableDataLongFrame<'_>) -> Result<Self, Self::Error> {
        if let crate::mbus_network::control::ControlInformation::RespVariable =
            value.control_infomation
        {
            let mut record = Self {
                address: value.address,
                manufactor: (value.manufacterer[0] as u16) << 8 | (value.manufacterer[1] as u16),
                identification_no: (value.indentification_no[0] as u32) << 24
                    | (value.indentification_no[1] as u32) << 16
                    | (value.indentification_no[2] as u32) << 8
                    | (value.indentification_no[3] as u32),
                transmission_counter: value.transmission_counter,
                device_type: DeviceType::try_from(value.device_type).unwrap_or(DeviceType::Other),
                version: value.version,
                status: value.status,
                signature: (value.signature[0] as u16) << 8 | (value.signature[1] as u16),
                records: Vec::new(),
            };
            for entry in value.parts {
                let mut record_entry = Record {
                    record_type: entry.di_field.data,
                    function: entry.di_field.function,
                    storage_number: entry.di_field.storage_lsb as u64,
                    traiff: 0,
                    sub_unit: 0,
                    value_data_type: entry.di_field.data_type,
                    units: Vec::new(),
                    magnitide: 0,
                    offset: 0,
                    value_data: Vec::new(),
                };
                if let Some(dife) = entry.dife_field {
                    for (i, dife_entry) in dife.into_iter().enumerate() {
                        let sn_part = (dife_entry.storage_number as u64) << (i * 4 + 1);
                        record_entry.storage_number |= sn_part;
                        let t_part = (dife_entry.tariff as u32) << (i * 2);
                        record_entry.traiff |= t_part;
                        let su_part = (dife_entry.device as u16) << i;
                        record_entry.sub_unit |= su_part;
                    }
                }
                if let Some(vif) = entry.vi_field {
                    record_entry.units.push(UnitData {
                        units: vif.units,
                        magnitude: vif.magnitude,
                        quantity: vif.quantity.to_string(),
                    })
                }
                if let Some(vife) = entry.vife_field {
                    for vf in vife {
                        record_entry.units.push(UnitData {
                            units: vf.units,
                            magnitude: vf.magnitude,
                            quantity: vf.quantity.to_string(),
                        })
                    }
                }

                if let Some(d) = entry.data {
                    for b in d {
                        record_entry.value_data.push(b);
                    }
                }
                record.records.push(record_entry);
            }
            Ok(record)
        } else {
            Err("Required VRD Frame")
        }
    }
}
