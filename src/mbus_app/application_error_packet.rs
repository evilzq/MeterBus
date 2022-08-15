#[repr(u8)]
pub enum Codes {
    Unspecified = 0,     //Unspecified error: also if data field is missing
    UnimplementedCi = 1, //Unimplemented CI-Field
    BufferTooLong = 2,   //Buffer too long, truncated
    TooManyRecords = 3,  //Too many records
    PrematureEnd = 4,    //Premature end of record
    TooManyDIFEs = 5,    //More than 10 DIFE´s
    TooManyVIFEs = 6,    //More than 10 VIFE´s
    Reserved = 7,        //Reserved
    Busy = 8,            //Application too busy for handling readout request
    TooManyReadouts = 9, //Too many readouts(for slaves with limited readouts per time)
}

pub struct ApplicationErrorPacket {
    pub access_demand: bool,
    pub data_flow_control: bool,
    pub address: u8,
    pub code: Codes,
}
