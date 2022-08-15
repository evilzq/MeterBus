#![no_std]
#![allow(dead_code)]

extern crate alloc;
extern crate num_enum;

pub mod macros;
pub mod mbus_app;
pub mod mbus_network;

pub fn checksum(bytes: &[u8]) -> u8 {
    let iter = bytes.iter();
    let mut sum: u32 = 0;
    iter.for_each(|value| sum += *value as u32);
    (sum & 0xFF) as u8
}
#[cfg(test)]
mod tests {}
