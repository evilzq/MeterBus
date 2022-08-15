#![no_std]
#![allow(dead_code)]
#[macro_use]
extern crate alloc;
extern crate num_enum;
#[cfg(test)]
pub mod en13757_2;
pub mod en13757_3;

pub mod macros;

pub fn checksum(bytes: &[u8]) -> u8 {
    let iter = bytes.iter();
    let mut sum: u32 = 0;
    iter.for_each(|value| sum += *value as u32);
    (sum & 0xFF) as u8
}

mod tests {}
