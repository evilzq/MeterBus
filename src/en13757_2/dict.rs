use super::{
    var_data::VariableDataQuantityUnit,
    vif::{VfiTableRecord, VifType},
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

pub const VIF_FIXED_TABLE: [VfiTableRecord; 61] = [
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

pub const VIF_VARIABLE_TABLE: [VfiTableRecord; 130] = [
    new_vfi_entry!(
        0x00,
        1.0e-3,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x01,
        1.0e-2,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x02,
        1.0e-1,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x03,
        1.0e0,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x04,
        1.0e1,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x05,
        1.0e2,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x06,
        1.0e3,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x07,
        1.0e4,
        "Wh",
        "Energy",
        VariableDataQuantityUnit::EnergyWh,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
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
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x09,
        1.0e1,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x0A,
        1.0e2,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x0B,
        1.0e3,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x0C,
        1.0e4,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x0D,
        1.0e5,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x0E,
        1.0e6,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x0F,
        1.0e7,
        "J",
        "Energy",
        VariableDataQuantityUnit::EnergyJ,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
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
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x11,
        1.0e-5,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x12,
        1.0e-4,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x13,
        1.0e-3,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x14,
        1.0e-2,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x15,
        1.0e-1,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x16,
        1.0e0,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x17,
        1.0e1,
        "m^3",
        "Volume",
        VariableDataQuantityUnit::VolumeM3,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
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
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x19,
        1.0e-2,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x1A,
        1.0e-1,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x1B,
        1.0e0,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x1C,
        1.0e1,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x1D,
        1.0e2,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x1E,
        1.0e3,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x1F,
        1.0e4,
        "kg",
        "Mass",
        VariableDataQuantityUnit::MassKg,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
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
        |b| (b & 0x03),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x21,
        60.0,
        "s",
        "On time",
        VariableDataQuantityUnit::OnTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x22,
        3600.0,
        "s",
        "On time",
        VariableDataQuantityUnit::OnTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x23,
        86400.0,
        "s",
        "On time",
        VariableDataQuantityUnit::OnTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
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
        |b| (b & 0x03),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x25,
        60.0,
        "s",
        "Operating time",
        VariableDataQuantityUnit::OperatingTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x26,
        3600.0,
        "s",
        "Operating time",
        VariableDataQuantityUnit::OperatingTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x27,
        86400.0,
        "s",
        "Operating time",
        VariableDataQuantityUnit::OperatingTime,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
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
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x29,
        1.0e-2,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x2A,
        1.0e-1,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x2B,
        1.0e0,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x2C,
        1.0e1,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x2D,
        1.0e2,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x2E,
        1.0e3,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x2F,
        1.0e4,
        "W",
        "Power",
        VariableDataQuantityUnit::PowerW,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
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
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x31,
        1.0e1,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x32,
        1.0e2,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x33,
        1.0e3,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x34,
        1.0e4,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x35,
        1.0e5,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x36,
        1.0e6,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
        ""
    ),
    new_vfi_entry!(
        0x37,
        1.0e7,
        "J/h",
        "Power",
        VariableDataQuantityUnit::PowerJPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07),
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
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x39,
        1.0e-5,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x3A,
        1.0e-4,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x3B,
        1.0e-3,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x3C,
        1.0e-2,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x3D,
        1.0e-1,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x3E,
        1.0e0,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
        ""
    ),
    new_vfi_entry!(
        0x3F,
        1.0e1,
        "m^3/h",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowM3PerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 6,
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
        |b| (b & 0x07) - 7,
        ""
    ),
    new_vfi_entry!(
        0x41,
        1.0e-6,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 7,
        ""
    ),
    new_vfi_entry!(
        0x42,
        1.0e-5,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 7,
        ""
    ),
    new_vfi_entry!(
        0x43,
        1.0e-4,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 7,
        ""
    ),
    new_vfi_entry!(
        0x44,
        1.0e-3,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 7,
        ""
    ),
    new_vfi_entry!(
        0x45,
        1.0e-2,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 7,
        ""
    ),
    new_vfi_entry!(
        0x46,
        1.0e-1,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 7,
        ""
    ),
    new_vfi_entry!(
        0x47,
        1.0e0,
        "m^3/min",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerMin,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 7,
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
        |b| (b & 0x07) - 9,
        ""
    ),
    new_vfi_entry!(
        0x49,
        1.0e-8,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 9,
        ""
    ),
    new_vfi_entry!(
        0x4A,
        1.0e-7,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 9,
        ""
    ),
    new_vfi_entry!(
        0x4B,
        1.0e-6,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 9,
        ""
    ),
    new_vfi_entry!(
        0x4C,
        1.0e-5,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 9,
        ""
    ),
    new_vfi_entry!(
        0x4D,
        1.0e-4,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 9,
        ""
    ),
    new_vfi_entry!(
        0x4E,
        1.0e-3,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 9,
        ""
    ),
    new_vfi_entry!(
        0x4F,
        1.0e-2,
        "m^3/s",
        "Volume flow",
        VariableDataQuantityUnit::VolumeFlowExtM3PerS,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 9,
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
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x51,
        1.0e-2,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x52,
        1.0e-1,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x53,
        1.0e0,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x54,
        1.0e1,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x55,
        1.0e2,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x56,
        1.0e3,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
        ""
    ),
    new_vfi_entry!(
        0x57,
        1.0e4,
        "kg/h",
        "Mass flow",
        VariableDataQuantityUnit::MassFlowKgPerH,
        VifType::PrimaryVIF,
        |b| (b & 0x07) - 3,
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
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x59,
        1.0e-2,
        "°C",
        "Flow temperature",
        VariableDataQuantityUnit::FlowTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x5A,
        1.0e-1,
        "°C",
        "Flow temperature",
        VariableDataQuantityUnit::FlowTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x5B,
        1.0e0,
        "°C",
        "Flow temperature",
        VariableDataQuantityUnit::FlowTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
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
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x5D,
        1.0e-2,
        "°C",
        "Return temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x5E,
        1.0e-1,
        "°C",
        "Return temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x5F,
        1.0e0,
        "°C",
        "Return temperature",
        VariableDataQuantityUnit::ReturnTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
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
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x61,
        1.0e-2,
        "K",
        "Temperature difference",
        VariableDataQuantityUnit::TemperatureDifferenceK,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x62,
        1.0e-1,
        "K",
        "Temperature difference",
        VariableDataQuantityUnit::TemperatureDifferenceK,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x63,
        1.0e0,
        "K",
        "Temperature difference",
        VariableDataQuantityUnit::TemperatureDifferenceK,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
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
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x65,
        1.0e-2,
        "°C",
        "External temperature",
        VariableDataQuantityUnit::ExternalTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x66,
        1.0e-1,
        "°C",
        "External temperature",
        VariableDataQuantityUnit::ExternalTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x67,
        1.0e0,
        "°C",
        "External temperature",
        VariableDataQuantityUnit::ExternalTemperatureC,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
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
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x69,
        1.0e-2,
        "bar",
        "Pressure",
        VariableDataQuantityUnit::PressureBar,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x6A,
        1.0e-1,
        "bar",
        "Pressure",
        VariableDataQuantityUnit::PressureBar,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
        ""
    ),
    new_vfi_entry!(
        0x6B,
        1.0e0,
        "bar",
        "Pressure",
        VariableDataQuantityUnit::PressureBar,
        VifType::PrimaryVIF,
        |b| (b & 0x03) - 3,
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
        |b| b & 0x01,
        ""
    ), /* n = 0        date, data type G */
    new_vfi_entry!(
        0x6D,
        1.0e0,
        "-",
        "Time point (date & time)",
        VariableDataQuantityUnit::TimePoint,
        VifType::PrimaryVIF,
        |b| b & 0x01,
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
        |b| (b & 0x03),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x71,
        60.0,
        "s",
        "Averaging Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x72,
        3600.0,
        "s",
        "Averaging Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x73,
        86400.0,
        "s",
        "Averaging Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
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
        |b| (b & 0x03),
        ""
    ), /* seconds */
    new_vfi_entry!(
        0x75,
        60.0,
        "s",
        "Actuality Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* minutes */
    new_vfi_entry!(
        0x76,
        3600.0,
        "s",
        "Actuality Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
        ""
    ), /* hours   */
    new_vfi_entry!(
        0x77,
        86400.0,
        "s",
        "Actuality Duration",
        VariableDataQuantityUnit::AveragingDuration,
        VifType::PrimaryVIF,
        |b| (b & 0x03),
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
