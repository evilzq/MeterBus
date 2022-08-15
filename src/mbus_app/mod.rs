pub mod alarm_status_packet;
pub mod application_error_packet;
pub mod empty_packet;
pub mod fixed_data_packet;
pub mod serializer;
pub mod time_magnitudes;
pub mod variable_data_packet;

pub trait Packet {}
