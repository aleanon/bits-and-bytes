use std::fmt::Debug;

use super::{Ascii, Binary, Decimal, Hex};

pub trait Converter: Sized + Debug {
    fn to_bytes(&self) -> Vec<u8>;

    fn to_binary(&self) -> Binary {
        use std::fmt::Write;
        let bytes = self.to_bytes();
        let value = bytes.iter()
            .fold(String::with_capacity(bytes.len() * 8), |mut acc, byte| {
                write!(&mut acc, "{:08b}", *byte).ok();
                acc
            });
        
        Binary(value)
    }

    fn to_ascii(&self) -> Ascii {
        use std::fmt::Write;
        let bytes = self.to_bytes();
        let value = bytes.iter()
            .fold(String::with_capacity(bytes.len() * 3), |mut acc, byte| {
                write!(&mut acc, "{}", *byte as char).ok();
                acc
            });
        
        Ascii(value)
    }


    fn to_hex(&self) -> Hex {
        use std::fmt::Write;
        let bytes = self.to_bytes();
        let value = bytes.iter()
            .fold(String::with_capacity(bytes.len() * 2), |mut acc, byte| {
                write!(&mut acc, "{:02X} ", byte).ok();
                acc
            });
        
        Hex(value)
    }

    fn to_decimal(&self) -> Decimal {
        use std::fmt::Write;
        let bytes = self.to_bytes();
        let value = bytes.iter()
            .fold(String::with_capacity(bytes.len() * 3), |mut acc, byte| {
                write!(&mut acc, "{:03} ", byte).ok();
                acc
            });
        
        Decimal(value)
    }
   
}
