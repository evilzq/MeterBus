use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum DeviceType {
    Other = 0x00,          //Other
    Oil = 0x01,            //Oil
    Electricity = 0x02,    //Electricity
    Gas = 0x03,            //Gas
    Heat = 0x04,           //Heat (Volume measured at return temperature: outlet)
    Steam = 0x05,          //Steam
    HotWater = 0x06,       //Hot Water
    Water = 0x07,          //Water
    HCA = 0x08,            //Heat Cost Allocator.
    CompressedAir = 0x09,  //Compressed Air
    CLM = 0x0A,            //Cooling load meter (Volume measured at return temperature: outlet)
    CLM2 = 0x0B,           //Cooling load meter (Volume measured at flow temperature: inlet)
    Heat2 = 0x0C,          //Heat (Volume measured at flow temperature: inlet)
    Water2 = 0x0D,         //Heat / Cooling load meter
    HCA2 = 0x0E,           //Wärme/Kühlung
    Unknown = 0x0F,        //Unknown Medium
    Reserved0x10 = 0x10,   //Reserved
    Reserved0x11 = 0x11,   //Reserved
    Reserved0x12 = 0x12,   //Reserved
    Reserved0x13 = 0x13,   //Reserved
    CalorificValue = 0x14, //Calorific value
    BoilingWater = 0x15,   //Boiling water
    ColdWater = 0x16,      //Cold Water
    DualWater = 0x17,      //Dual Water
    Pressure = 0x18,       //Pressure
    ADConverter = 0x19,    //A/D Converter
    SmokeDetector = 0x1A,  //Smoke detector
    RoomSensor = 0x1B,     //Room sensor
    GasDetector = 0x1C,    //Gas detector
    Reserved0x1d = 0x1D,   //Reserved for sensors
    Reserved0x1e = 0x1E,   //Reserved for sensors
    Reserved0x1f = 0x1F,   //Reserved for sensors
    CircuitBreaker = 0x20, //Circuit breaker
    Ventil = 0x21,         //Ventil
    Reserved0x22 = 0x22,   //Reserved
    Reserved0x23 = 0x23,   //Reserved
    Reserved0x24 = 0x24,   //Reserved
    Display = 0x25,        //Display
    Reserved0x26 = 0x26,   //Reserved
    Reserved0x27 = 0x27,   //Reserved
    Sewage = 0x28,         //Sewage
    Waste = 0x29,          //Waste
    CarbonDioxide = 0x2A,  //Carbon dioxide
    #[num_enum(default)]
    Reserved0x2b = 0x2B, //Reserved
}
