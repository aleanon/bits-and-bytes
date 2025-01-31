use std::fmt::Display;

use super::{Binary, Converter};

#[derive(Debug)]
pub struct Decimal(pub String);

impl Decimal {
    pub fn new() -> Self {
        Self(String::new())
    }
}


impl Converter for Decimal {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
            .iter()
            .filter(|b| !(**b as char).is_ascii_whitespace())
            .copied()
            .collect::<Vec<u8>>()
            .chunks(3)
            .filter_map(|chunk| {
                let str = std::str::from_utf8(chunk).ok()?;
                Some(u8::from_str_radix(str, 10).ok()?)
            })
            .collect()
    }

}


