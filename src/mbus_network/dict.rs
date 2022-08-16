use super::{
    var_data::VariableDataQuantityUnit,
    vif::{VfiTableRecord, VifType, VifeFbTableRecord, VifeFeTableRecord},
};

#[macro_export]
macro_rules! new_vfi_entry {
    ($vif:expr,$exponent:expr,$unit:expr,$quantity:expr,$units:expr,$types:expr,$mag:expr,$name:expr) => {
        VfiTableRecord {
            vif: $vif,
            exponent: $exponent,
            quantity: $quantity,
            unit: $unit,
            units: $units,
            types: $types,
            magnitude: $mag,
            name: $name,
        }
    };
}
#[macro_export]
macro_rules! new_vifb_entry {
    ($viffb:expr, $units:expr, $magnitude:expr) => {
        VifeFbTableRecord {
            vifb: $viffb,
            units: $units,
            unit: "",
            quantity: "",
            magnitude: $magnitude,
        }
    };
}
#[macro_export]
macro_rules! new_vifd_entry {
    ($viffd:expr, $units:expr, $magnitude:expr) => {
        VifeFeTableRecord {
            vife: $viffd,
            units: $units,
            unit: "",
            quantity: "",
            magnitude: $magnitude,
        }
    };
}

pub static VIF_FIXED_TABLE: [VfiTableRecord; 61] = [
    new_vfi_entry!(
        0x02,
        1.0e0,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x03,
        1.0e1,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x04,
        1.0e2,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x05,
        1.0e3,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x06,
        1.0e4,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x07,
        1.0e5,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x08,
        1.0e6,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x09,
        1.0e7,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x0A,
        1.0e8,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x0B,
        1.0e3,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x0C,
        1.0e4,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x0D,
        1.0e5,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x0E,
        1.0e6,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x0F,
        1.0e7,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x10,
        1.0e8,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x11,
        1.0e9,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x12,
        1.0e10,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x13,
        1.0e11,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x14,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x15,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x16,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x17,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x18,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x19,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x1A,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x1B,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x1C,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x1D,
        1.0e3,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x1E,
        1.0e4,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x1F,
        1.0e5,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x20,
        1.0e6,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x21,
        1.0e7,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x22,
        1.0e8,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x23,
        1.0e9,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x24,
        1.0e10,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x25,
        1.0e11,
        "J/h",
        "Energy",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x26,
        1.0e-6,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x27,
        1.0e-5,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x28,
        1.0e-4,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x29,
        1.0e-3,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x2A,
        1.0e-2,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x2B,
        1.0e-1,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x2C,
        1.0e0,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x2D,
        1.0e1,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x2E,
        1.0e2,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x2F,
        1.0e-5,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x31,
        1.0e-4,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x32,
        1.0e-3,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x33,
        1.0e-2,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x34,
        1.0e-1,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x35,
        1.0e0,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x36,
        1.0e1,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x37,
        1.0e2,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x38,
        1.0e-3,
        "°C",
        "Temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x39,
        1.0e0,
        "Units for H.C.A.",
        "H.C.A.",
        VariableDataQuantityUnit::UnitsForHCA,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x3A,
        0.0,
        "Reserved",
        "Reserved",
        VariableDataQuantityUnit::Reserved,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x3B,
        0.0,
        "Reserved",
        "Reserved",
        VariableDataQuantityUnit::Reserved,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x3C,
        0.0,
        "Reserved",
        "Reserved",
        VariableDataQuantityUnit::Reserved,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x3D,
        0.0,
        "Reserved",
        "Reserved",
        VariableDataQuantityUnit::Reserved,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x3E,
        1.0e0,
        "",
        "historic",
        VariableDataQuantityUnit::Undefined,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x3F,
        1.0e0,
        "",
        "No units",
        VariableDataQuantityUnit::Undefined,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
];

pub static VIF_VARIABLE_TABLE: [VfiTableRecord; 130] = [
    new_vfi_entry!(
        0x00,
        1.0e-3,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x01,
        1.0e-2,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x02,
        1.0e-1,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x03,
        1.0e0,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x04,
        1.0e1,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x05,
        1.0e2,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x06,
        1.0e3,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x07,
        1.0e4,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    /* E000 1nnn    Energy  J (0.001kJ to 10000kJ) */
    new_vfi_entry!(
        0x08,
        1.0e0,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x09,
        1.0e1,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x0A,
        1.0e2,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x0B,
        1.0e3,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x0C,
        1.0e4,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x0D,
        1.0e5,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x0E,
        1.0e6,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x0F,
        1.0e7,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    /* E001 0nnn    Volume m^3 (0.001l to 10000l) */
    new_vfi_entry!(
        0x10,
        1.0e-6,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x11,
        1.0e-5,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x12,
        1.0e-4,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x13,
        1.0e-3,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x14,
        1.0e-2,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x15,
        1.0e-1,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x16,
        1.0e0,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x17,
        1.0e1,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    /* E001 1nnn    Mass kg (0.001kg to 10000kg) */
    new_vfi_entry!(
        0x18,
        1.0e-3,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x19,
        1.0e-2,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x1A,
        1.0e-1,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x1B,
        1.0e0,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x1C,
        1.0e1,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x1D,
        1.0e2,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x1E,
        1.0e3,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x1F,
        1.0e4,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    /* E010 00nn    On Time s */
    new_vfi_entry!(
        0x20,
        1.0,
        "s",
        "On time",
        VariableDataQuantityUnit::OnTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x21,
        60.0,
        "s",
        "On time",
        VariableDataQuantityUnit::OnTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x22,
        3600.0,
        "s",
        "On time",
        VariableDataQuantityUnit::OnTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x23,
        86400.0,
        "s",
        "On time",
        VariableDataQuantityUnit::OnTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* days    */
    /* E010 01nn    Operating Time s */
    new_vfi_entry!(
        0x24,
        1.0,
        "s",
        "Operating time",
        VariableDataQuantityUnit::OperatingTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x25,
        60.0,
        "s",
        "Operating time",
        VariableDataQuantityUnit::OperatingTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x26,
        3600.0,
        "s",
        "Operating time",
        VariableDataQuantityUnit::OperatingTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x27,
        86400.0,
        "s",
        "Operating time",
        VariableDataQuantityUnit::OperatingTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* days    */
    /* E010 1nnn    Power W (0.001W to 10000W) */
    new_vfi_entry!(
        0x28,
        1.0e-3,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x29,
        1.0e-2,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x2A,
        1.0e-1,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x2B,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x2C,
        1.0e1,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x2D,
        1.0e2,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x2E,
        1.0e3,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x2F,
        1.0e4,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    /* E011 0nnn    Power J/h (0.001kJ/h to 10000kJ/h) */
    new_vfi_entry!(
        0x30,
        1.0e0,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x31,
        1.0e1,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x32,
        1.0e2,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x33,
        1.0e3,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x34,
        1.0e4,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x35,
        1.0e5,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x36,
        1.0e6,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x37,
        1.0e7,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07).try_into().unwrap(),
        ""
    ),
    /* E011 1nnn    Volume Flow m3/h (0.001l/h to 10000l/h) */
    new_vfi_entry!(
        0x38,
        1.0e-6,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x39,
        1.0e-5,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x3A,
        1.0e-4,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x3B,
        1.0e-3,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x3C,
        1.0e-2,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x3D,
        1.0e-1,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x3E,
        1.0e0,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x3F,
        1.0e1,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 6).try_into().unwrap(),
        ""
    ),
    /* E100 0nnn     Volume Flow ext.  m^3/min (0.0001l/min to 1000l/min) */
    new_vfi_entry!(
        0x40,
        1.0e-7,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x41,
        1.0e-6,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x42,
        1.0e-5,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x43,
        1.0e-4,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x44,
        1.0e-3,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x45,
        1.0e-2,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x46,
        1.0e-1,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x47,
        1.0e0,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 7).try_into().unwrap(),
        ""
    ),
    /* E100 1nnn     Volume Flow ext.  m^3/s (0.001ml/s to 10000ml/s) */
    new_vfi_entry!(
        0x48,
        1.0e-9,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x49,
        1.0e-8,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x4A,
        1.0e-7,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x4B,
        1.0e-6,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x4C,
        1.0e-5,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x4D,
        1.0e-4,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x4E,
        1.0e-3,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x4F,
        1.0e-2,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 9).try_into().unwrap(),
        ""
    ),
    /* E101 0nnn     Mass flow kg/h (0.001kg/h to 10000kg/h) */
    new_vfi_entry!(
        0x50,
        1.0e-3,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x51,
        1.0e-2,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x52,
        1.0e-1,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x53,
        1.0e0,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x54,
        1.0e1,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x55,
        1.0e2,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x56,
        1.0e3,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x57,
        1.0e4,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| ((b & 0x07) - 3).try_into().unwrap(),
        ""
    ),
    /* E101 10nn     Flow Temperature °C (0.001°C to 1°C) */
    new_vfi_entry!(
        0x58,
        1.0e-3,
        "°C",
        "Flow temperature",
        VariableDataQuantityUnit::FlowTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x59,
        1.0e-2,
        "°C",
        "Flow temperature",
        VariableDataQuantityUnit::FlowTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x5A,
        1.0e-1,
        "°C",
        "Flow temperature",
        VariableDataQuantityUnit::FlowTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x5B,
        1.0e0,
        "°C",
        "Flow temperature",
        VariableDataQuantityUnit::FlowTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    /* E101 11nn Return Temperature °C (0.001°C to 1°C) */
    new_vfi_entry!(
        0x5C,
        1.0e-3,
        "°C",
        "Return temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x5D,
        1.0e-2,
        "°C",
        "Return temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x5E,
        1.0e-1,
        "°C",
        "Return temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x5F,
        1.0e0,
        "°C",
        "Return temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    /* E110 00nn    Temperature Difference  K   (mK to  K) */
    new_vfi_entry!(
        0x60,
        1.0e-3,
        "K",
        "Temperature difference",
        VariableDataQuantityUnit::TemperatureDifferenceK,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x61,
        1.0e-2,
        "K",
        "Temperature difference",
        VariableDataQuantityUnit::TemperatureDifferenceK,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x62,
        1.0e-1,
        "K",
        "Temperature difference",
        VariableDataQuantityUnit::TemperatureDifferenceK,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x63,
        1.0e0,
        "K",
        "Temperature difference",
        VariableDataQuantityUnit::TemperatureDifferenceK,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    /* E110 01nn     External Temperature °C (0.001°C to 1°C) */
    new_vfi_entry!(
        0x64,
        1.0e-3,
        "°C",
        "External temperature",
        VariableDataQuantityUnit::ExternalTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x65,
        1.0e-2,
        "°C",
        "External temperature",
        VariableDataQuantityUnit::ExternalTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x66,
        1.0e-1,
        "°C",
        "External temperature",
        VariableDataQuantityUnit::ExternalTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x67,
        1.0e0,
        "°C",
        "External temperature",
        VariableDataQuantityUnit::ExternalTemperatureC,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    /* E110 10nn     Pressure bar (1mbar to 1000mbar) */
    new_vfi_entry!(
        0x68,
        1.0e-3,
        "bar",
        "Pressure",
        VariableDataQuantityUnit::PressureBar,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x69,
        1.0e-2,
        "bar",
        "Pressure",
        VariableDataQuantityUnit::PressureBar,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x6A,
        1.0e-1,
        "bar",
        "Pressure",
        VariableDataQuantityUnit::PressureBar,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    new_vfi_entry!(
        0x6B,
        1.0e0,
        "bar",
        "Pressure",
        VariableDataQuantityUnit::PressureBar,
        VifType::PrimaryVIF,
        |b| ((b & 0x03) - 3).try_into().unwrap(),
        ""
    ),
    /* E110 110n     Time Point */
    new_vfi_entry!(
        0x6C,
        1.0e0,
        "-",
        "Time point (date)",
        VariableDataQuantityUnit::TimePoint,
        VifType::PrimaryVIF,
        |b| (b & 0x01).try_into().unwrap(),
        ""
    ), /* n = 0        date, data type G */
    new_vfi_entry!(
        0x6D,
        1.0e0,
        "-",
        "Time point (date & time)",
        VariableDataQuantityUnit::TimePoint,
        VifType::PrimaryVIF,
        |b| (b & 0x01).try_into().unwrap(),
        ""
    ), /* n = 1 time & date, data type F */
    /* E110 1110     Units for H.C.A. dimensionless */
    new_vfi_entry!(
        0x6E,
        1.0e0,
        "Units for H.C.A.",
        "H.C.A.",
        VariableDataQuantityUnit::UnitsForHCA,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    /* E110 1111     Reserved */
    new_vfi_entry!(
        0x6F,
        0.0,
        "Reserved",
        "Reserved",
        VariableDataQuantityUnit::Reserved,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    /* E111 00nn     Averaging Duration s */
    new_vfi_entry!(
        0x70,
        1.0,
        "s",
        "Averaging Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x71,
        60.0,
        "s",
        "Averaging Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x72,
        3600.0,
        "s",
        "Averaging Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x73,
        86400.0,
        "s",
        "Averaging Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* days    */
    /* E111 01nn     Actuality Duration s */
    new_vfi_entry!(
        0x74,
        1.0,
        "s",
        "Actuality Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x75,
        60.0,
        "s",
        "Actuality Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x76,
        3600.0,
        "s",
        "Actuality Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x77,
        86400.0,
        "s",
        "Actuality Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03).try_into().unwrap(),
        ""
    ), /* days    */
    /* Fabrication No */
    new_vfi_entry!(
        0x78,
        1.0,
        "",
        "Fabrication No",
        VariableDataQuantityUnit::FabricationNo,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    /* E111 1001 (Enhanced) Identification */
    new_vfi_entry!(
        0x79,
        1.0,
        "",
        "(Enhanced) Identification",
        VariableDataQuantityUnit::EnhancedIdentification,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    /* E111 1010 Bus Address */
    new_vfi_entry!(
        0x7a,
        1.0,
        "",
        "Bus Address",
        VariableDataQuantityUnit::BusAddress,
        VifType::PrimaryVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x7b,
        1.0,
        "",
        "Extension 7b",
        VariableDataQuantityUnit::Extension7b,
        VifType::LinearVIFExtensionFB,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x7c,
        1.0,
        "",
        "Custom VIF",
        VariableDataQuantityUnit::CustomVIF,
        VifType::PlainTextVIF,
        |_| 0,
        ""
    ),
    new_vfi_entry!(
        0x7d,
        1.0,
        "",
        "Extension 7d",
        VariableDataQuantityUnit::Extension7b,
        VifType::LinearVIFExtensionFD,
        |_| 0,
        ""
    ),
    /* Any VIF: 7Eh */
    new_vfi_entry!(
        0x7e,
        1.0,
        "",
        "Any VIF",
        VariableDataQuantityUnit::AnyVIF,
        VifType::AnyVIF,
        |_| 0,
        ""
    ),
    /* Manufacturer specific: 7Fh */
    new_vfi_entry!(
        0x7f,
        1.0,
        "",
        "Manufacturer specific",
        VariableDataQuantityUnit::ManufacturerSpecific,
        VifType::ManufacturerSpecific,
        |_| 0,
        ""
    ),
    /* Any VIF: 7Eh */
    new_vfi_entry!(
        0xfe,
        1.0,
        "",
        "Any VIF",
        VariableDataQuantityUnit::AnyVIF,
        VifType::AnyVIF,
        |_| 0,
        ""
    ),
    /* Manufacturer specific: FFh */
    new_vfi_entry!(
        0xff,
        1.0,
        "",
        "Manufacturer specific",
        VariableDataQuantityUnit::ManufacturerSpecific,
        VifType::ManufacturerSpecific,
        |_| 0,
        ""
    ),
];

pub static VIFE_FB_TABLE: [VifeFbTableRecord; 128] = [
    new_vifb_entry!(0x00, VariableDataQuantityUnit::EnergyMWh, |b| ((b & 0x01)
        - 1)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x01, VariableDataQuantityUnit::EnergyMWh, |b| ((b & 0x01)
        - 1)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x02, VariableDataQuantityUnit::ReservedVifeFb02, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x03, VariableDataQuantityUnit::ReservedVifeFb02, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x04, VariableDataQuantityUnit::ReservedVifeFb04, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x05, VariableDataQuantityUnit::ReservedVifeFb04, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x06, VariableDataQuantityUnit::ReservedVifeFb04, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x07, VariableDataQuantityUnit::ReservedVifeFb04, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x08, VariableDataQuantityUnit::EnergyGJ, |b| ((b & 0x01)
        - 1)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x09, VariableDataQuantityUnit::EnergyGJ, |b| ((b & 0x01)
        - 1)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x0a, VariableDataQuantityUnit::ReservedVifeFb0a, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x0b, VariableDataQuantityUnit::ReservedVifeFb0a, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x0c, VariableDataQuantityUnit::ReservedVifeFb0c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x0d, VariableDataQuantityUnit::ReservedVifeFb0c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x0e, VariableDataQuantityUnit::ReservedVifeFb0c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x0f, VariableDataQuantityUnit::ReservedVifeFb0c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x10, VariableDataQuantityUnit::VolumeM3, |b| ((b & 0x01)
        + 2)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x11, VariableDataQuantityUnit::VolumeM3, |b| ((b & 0x01)
        + 2)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x12, VariableDataQuantityUnit::ReservedVifeFb12, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x13, VariableDataQuantityUnit::ReservedVifeFb12, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x14, VariableDataQuantityUnit::ReservedVifeFb14, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x15, VariableDataQuantityUnit::ReservedVifeFb14, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x16, VariableDataQuantityUnit::ReservedVifeFb14, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x17, VariableDataQuantityUnit::ReservedVifeFb14, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x18, VariableDataQuantityUnit::MassT, |b| ((b & 0x01) + 2)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x19, VariableDataQuantityUnit::MassT, |b| ((b & 0x01) + 2)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x1a, VariableDataQuantityUnit::ReservedVifeFb1a, |b| (b
        - 0x1a)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x1b, VariableDataQuantityUnit::ReservedVifeFb1a, |b| (b
        - 0x1a)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x1c, VariableDataQuantityUnit::ReservedVifeFb1a, |b| (b
        - 0x1a)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x1d, VariableDataQuantityUnit::ReservedVifeFb1a, |b| (b
        - 0x1a)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x1e, VariableDataQuantityUnit::ReservedVifeFb1a, |b| (b
        - 0x1a)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x1f, VariableDataQuantityUnit::ReservedVifeFb1a, |b| (b
        - 0x1a)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x20, VariableDataQuantityUnit::ReservedVifeFb1a, |b| (b
        - 0x1a)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x21, VariableDataQuantityUnit::VolumeFeet3, |_| -1),
    new_vifb_entry!(0x22, VariableDataQuantityUnit::VolumeAmericanGallon, |b| (b
        - 0x23)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x23, VariableDataQuantityUnit::VolumeAmericanGallon, |b| (b
        - 0x23)
        .try_into()
        .unwrap()),
    new_vifb_entry!(
        0x24,
        VariableDataQuantityUnit::VolumeFlowAmericanGallonPerMin,
        |_| -3
    ),
    new_vifb_entry!(
        0x25,
        VariableDataQuantityUnit::VolumeFlowAmericanGallonPerMin,
        |_| 0
    ),
    new_vifb_entry!(
        0x26,
        VariableDataQuantityUnit::VolumeFlowAmericanGallonPerH,
        |_| 0
    ),
    new_vifb_entry!(0x27, VariableDataQuantityUnit::ReservedVifeFb27, |_| 0),
    new_vifb_entry!(0x28, VariableDataQuantityUnit::PowerW, |b| ((b & 0x01) - 1)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x29, VariableDataQuantityUnit::PowerW, |b| ((b & 0x01) - 1)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x2a, VariableDataQuantityUnit::ReservedVifeFb2a, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x2b, VariableDataQuantityUnit::ReservedVifeFb2a, |b| (b
        & 0x01)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x2c, VariableDataQuantityUnit::ReservedVifeFb2c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x2d, VariableDataQuantityUnit::ReservedVifeFb2c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x2e, VariableDataQuantityUnit::ReservedVifeFb2c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x2f, VariableDataQuantityUnit::ReservedVifeFb2c, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x30, VariableDataQuantityUnit::PowerGjPerH, |b| ((b
        & 0x01)
        - 1)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x31, VariableDataQuantityUnit::PowerGjPerH, |b| ((b
        & 0x01)
        - 1)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x32, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x33, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x34, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x35, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x36, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x37, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x38, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x39, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x3a, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x3b, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x3c, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x3d, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x3e, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x3f, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x40, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x41, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x42, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x43, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x44, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x45, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x46, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x47, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x48, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x49, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x4a, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x4b, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x4c, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x4d, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x4e, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x4f, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x50, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x51, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x52, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x53, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x54, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x55, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x56, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x57, VariableDataQuantityUnit::ReservedVifeFb32, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x58, VariableDataQuantityUnit::FlowTemperatureF, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x59, VariableDataQuantityUnit::FlowTemperatureF, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x5a, VariableDataQuantityUnit::FlowTemperatureF, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x5b, VariableDataQuantityUnit::FlowTemperatureF, |b| (b
        - 0x32)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x5c, VariableDataQuantityUnit::ReturnTemperatureF, |b| ((b
        & 0x03)
        - 3)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x5d, VariableDataQuantityUnit::ReturnTemperatureF, |b| ((b
        & 0x03)
        - 3)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x5e, VariableDataQuantityUnit::ReturnTemperatureF, |b| ((b
        & 0x03)
        - 3)
    .try_into()
    .unwrap()),
    new_vifb_entry!(0x5f, VariableDataQuantityUnit::ReturnTemperatureF, |b| ((b
        & 0x03)
        - 3)
    .try_into()
    .unwrap()),
    new_vifb_entry!(
        0x60,
        VariableDataQuantityUnit::TemperatureDifferenceF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x61,
        VariableDataQuantityUnit::TemperatureDifferenceF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x62,
        VariableDataQuantityUnit::TemperatureDifferenceF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x63,
        VariableDataQuantityUnit::TemperatureDifferenceF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(0x64, VariableDataQuantityUnit::ExternalTemperatureF, |b| {
        ((b & 0x03) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x65, VariableDataQuantityUnit::ExternalTemperatureF, |b| {
        ((b & 0x03) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x66, VariableDataQuantityUnit::ExternalTemperatureF, |b| {
        ((b & 0x03) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x67, VariableDataQuantityUnit::ExternalTemperatureF, |b| {
        ((b & 0x03) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x68, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x69, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x6a, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x6b, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x6c, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x6d, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x6e, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(0x6f, VariableDataQuantityUnit::ReservedVifeFb68, |b| (b
        & 0x07)
        .try_into()
        .unwrap()),
    new_vifb_entry!(
        0x70,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x71,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x72,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x73,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitF,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x74,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitC,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x75,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitC,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x76,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitC,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(
        0x77,
        VariableDataQuantityUnit::ColdWarmTemperatureLimitC,
        |b| ((b & 0x03) - 3).try_into().unwrap()
    ),
    new_vifb_entry!(0x78, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x79, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x7a, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x7b, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x7c, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x7d, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x7e, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
    new_vifb_entry!(0x7f, VariableDataQuantityUnit::CumulCountMaxPowerW, |b| {
        ((b & 0x07) - 3).try_into().unwrap()
    }),
];

pub static VIFE_FD_TABLE: [VifeFeTableRecord; 128] = [
    new_vifd_entry!(0x00, VariableDataQuantityUnit::Credit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x01, VariableDataQuantityUnit::Credit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x02, VariableDataQuantityUnit::Credit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x03, VariableDataQuantityUnit::Credit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x04, VariableDataQuantityUnit::Debit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x05, VariableDataQuantityUnit::Debit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x06, VariableDataQuantityUnit::Debit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x07, VariableDataQuantityUnit::Debit, |b| ((b & 0x03) - 3)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x08, VariableDataQuantityUnit::AccessNumber, |_| 0),
    new_vifd_entry!(0x09, VariableDataQuantityUnit::Medium, |_| 0),
    new_vifd_entry!(0x0a, VariableDataQuantityUnit::Manufacturer, |_| 0),
    new_vifd_entry!(
        0x0b,
        VariableDataQuantityUnit::EnhancedIdentification,
        |_| 0
    ),
    new_vifd_entry!(0x0c, VariableDataQuantityUnit::ModelVersion, |_| 0),
    new_vifd_entry!(0x0d, VariableDataQuantityUnit::HardwareVersionNr, |_| 0),
    new_vifd_entry!(0x0e, VariableDataQuantityUnit::FirmwareVersionNr, |_| 0),
    new_vifd_entry!(0x0f, VariableDataQuantityUnit::SoftwareVersionNr, |_| 0),
    new_vifd_entry!(0x10, VariableDataQuantityUnit::CustomerLocation, |_| 0),
    new_vifd_entry!(0x11, VariableDataQuantityUnit::Customer, |_| 0),
    new_vifd_entry!(0x12, VariableDataQuantityUnit::AccessCodeUser, |_| 0),
    new_vifd_entry!(0x13, VariableDataQuantityUnit::AccessCodeOperator, |_| 0),
    new_vifd_entry!(
        0x14,
        VariableDataQuantityUnit::AccessCodeSystemOperator,
        |_| 0
    ),
    new_vifd_entry!(0x15, VariableDataQuantityUnit::AccessCodeDeveloper, |_| 0),
    new_vifd_entry!(0x16, VariableDataQuantityUnit::Password, |_| 0),
    new_vifd_entry!(0x17, VariableDataQuantityUnit::ErrorFlags, |_| 0),
    new_vifd_entry!(0x18, VariableDataQuantityUnit::ErrorMask, |_| 0),
    new_vifd_entry!(0x19, VariableDataQuantityUnit::ReservedVifeFd19, |_| 0),
    new_vifd_entry!(0x1a, VariableDataQuantityUnit::DigitalOutput, |_| 0),
    new_vifd_entry!(0x1b, VariableDataQuantityUnit::DigitalInput, |_| 0),
    new_vifd_entry!(0x1c, VariableDataQuantityUnit::Baudrate, |_| 0),
    new_vifd_entry!(0x1d, VariableDataQuantityUnit::ResponseDelayTime, |_| 0),
    new_vifd_entry!(0x1e, VariableDataQuantityUnit::Retry, |_| 0),
    new_vifd_entry!(0x1f, VariableDataQuantityUnit::ReservedVifeFd1f, |_| 0),
    new_vifd_entry!(0x20, VariableDataQuantityUnit::FirstStorageNr, |_| 0),
    new_vifd_entry!(0x21, VariableDataQuantityUnit::LastStorageNr, |_| 0),
    new_vifd_entry!(0x22, VariableDataQuantityUnit::SizeOfStorage, |_| 0),
    new_vifd_entry!(0x23, VariableDataQuantityUnit::ReservedVifeFd23, |_| 0),
    new_vifd_entry!(0x24, VariableDataQuantityUnit::StorageInterval, |_| 0),
    new_vifd_entry!(0x25, VariableDataQuantityUnit::StorageInterval, |_| 0),
    new_vifd_entry!(0x26, VariableDataQuantityUnit::StorageInterval, |_| 0),
    new_vifd_entry!(0x27, VariableDataQuantityUnit::StorageInterval, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x28, VariableDataQuantityUnit::StorageIntervalMmnth, |_| 0),
    new_vifd_entry!(0x29, VariableDataQuantityUnit::StorageIntervalYear, |_| 0),
    new_vifd_entry!(0x2a, VariableDataQuantityUnit::ReservedVifeFb2a, |_| 0),
    new_vifd_entry!(0x2b, VariableDataQuantityUnit::ReservedVifeFd2b, |_| 0),
    new_vifd_entry!(
        0x2c,
        VariableDataQuantityUnit::DurationSinceLastReadout,
        |_| 0
    ),
    new_vifd_entry!(
        0x2d,
        VariableDataQuantityUnit::DurationSinceLastReadout,
        |_| 0
    ),
    new_vifd_entry!(
        0x2e,
        VariableDataQuantityUnit::DurationSinceLastReadout,
        |_| 0
    ),
    new_vifd_entry!(
        0x2f,
        VariableDataQuantityUnit::DurationSinceLastReadout,
        |_| 0
    ),
    new_vifd_entry!(0x30, VariableDataQuantityUnit::StartDateTimeOfTariff, |_| 0),
    new_vifd_entry!(0x31, VariableDataQuantityUnit::DurationOfTariff, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x32, VariableDataQuantityUnit::DurationOfTariff, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x33, VariableDataQuantityUnit::DurationOfTariff, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x34, VariableDataQuantityUnit::PeriodOfTariff, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x35, VariableDataQuantityUnit::PeriodOfTariff, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x36, VariableDataQuantityUnit::PeriodOfTariff, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x37, VariableDataQuantityUnit::PeriodOfTariff, |b| (b
        & 0x03)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x38, VariableDataQuantityUnit::PeriodOfTariffMonths, |_| 0),
    new_vifd_entry!(0x39, VariableDataQuantityUnit::PeriodOfTariffYear, |_| 0),
    new_vifd_entry!(0x3a, VariableDataQuantityUnit::Dimensionless, |_| 0),
    new_vifd_entry!(0x3b, VariableDataQuantityUnit::ReservedFd3b, |_| 0),
    new_vifd_entry!(0x3c, VariableDataQuantityUnit::ReservedFd3c, |_| 0),
    new_vifd_entry!(0x3d, VariableDataQuantityUnit::ReservedFd3c, |_| 0),
    new_vifd_entry!(0x3e, VariableDataQuantityUnit::ReservedFd3c, |_| 0),
    new_vifd_entry!(0x3f, VariableDataQuantityUnit::ReservedFd3c, |_| 0),
    new_vifd_entry!(0x40, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x41, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x42, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x43, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x44, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x45, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x46, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x47, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x48, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x49, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x4a, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x4b, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x4c, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x4d, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x4e, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x4f, VariableDataQuantityUnit::Volts, |b| ((b & 0x0f) - 9)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x50, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x51, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x52, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x53, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x54, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x55, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x56, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x57, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x58, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x59, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x5a, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x5b, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x5c, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x5d, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x5e, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x5f, VariableDataQuantityUnit::Ampers, |b| ((b & 0x0f)
        - 12)
        .try_into()
        .unwrap()),
    new_vifd_entry!(0x60, VariableDataQuantityUnit::ResetCounter, |_| 0),
    new_vifd_entry!(0x61, VariableDataQuantityUnit::CumulationCounter, |_| 0),
    new_vifd_entry!(0x62, VariableDataQuantityUnit::ControlSignal, |_| 0),
    new_vifd_entry!(0x63, VariableDataQuantityUnit::DayOfWeek, |_| 0),
    new_vifd_entry!(0x64, VariableDataQuantityUnit::WeekNumber, |_| 0),
    new_vifd_entry!(0x65, VariableDataQuantityUnit::TimePointOfDayChange, |_| 0),
    new_vifd_entry!(
        0x66,
        VariableDataQuantityUnit::StateOfParameterActivation,
        |_| 0
    ),
    new_vifd_entry!(
        0x67,
        VariableDataQuantityUnit::SpecialSupplierInformation,
        |_| 0
    ),
    new_vifd_entry!(
        0x68,
        VariableDataQuantityUnit::DurationSinceLastCumulation,
        |_| 0
    ),
    new_vifd_entry!(
        0x69,
        VariableDataQuantityUnit::DurationSinceLastCumulation,
        |_| 0
    ),
    new_vifd_entry!(
        0x6a,
        VariableDataQuantityUnit::DurationSinceLastCumulation,
        |_| 0
    ),
    new_vifd_entry!(
        0x6b,
        VariableDataQuantityUnit::DurationSinceLastCumulation,
        |_| 0
    ),
    new_vifd_entry!(0x6c, VariableDataQuantityUnit::OperatingTimeBattery, |_| 0),
    new_vifd_entry!(0x6d, VariableDataQuantityUnit::OperatingTimeBattery, |_| 0),
    new_vifd_entry!(0x6e, VariableDataQuantityUnit::OperatingTimeBattery, |_| 0),
    new_vifd_entry!(0x6f, VariableDataQuantityUnit::OperatingTimeBattery, |_| 0),
    new_vifd_entry!(
        0x70,
        VariableDataQuantityUnit::DateTimeOfBatteryChange,
        |_| 0
    ),
    new_vifd_entry!(0x71, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x72, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x73, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x74, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x75, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x76, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x77, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x78, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x79, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x7a, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x7b, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x7c, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x7d, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x7e, VariableDataQuantityUnit::ReservedFd71, |_| 0),
    new_vifd_entry!(0x7f, VariableDataQuantityUnit::ReservedFd71, |_| 0),
];
