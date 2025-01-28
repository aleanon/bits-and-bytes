use std::fmt::Debug;

use super::{Ascii, Binary, Decimal, Hex};

pub trait Converter: Sized + Debug {
    fn to_binary(&self) -> Binary;

    fn to_ascii(&self) -> Ascii {
        let binary = self.to_binary().0;
        let value = Self::from_binary(&binary, |chunk| {
            let string = chunk.iter().collect::<String>();
            let ascii = u8::from_str_radix(&string, 2).ok()?;
            Some(format!("{}", ascii as char))
        }).join("");     

        Ascii(value)
    }

    fn to_hex(&self) -> Hex {
        let binary = self.to_binary().0;
        let value = Self::from_binary(&binary, |chunk|{
            let string = chunk.iter().collect::<String>();
            let hex = u16::from_str_radix(&string, 2).ok()?;
            Some(format!("{:02X}", hex))
        })
        .join(" ");

        Hex(value)
    }

    fn to_decimal(&self) -> Decimal {
        let binary = self.to_binary().0;
        let value = Self::from_binary(&binary, |chunk|{
            let string = chunk.iter().collect::<String>();
            let dec = u32::from_str_radix(&string, 2).ok()?;
            Some(format!("{:03}", dec))
        })
       .join(" ");
    
        Decimal(value)
    }

    fn from_binary<F>(input: &str, from_binary: F) -> Vec<String> 
        where F: Fn(&[char]) -> Option<String>
    {
        input.chars()
            .filter(|&c| !(c == ' '))
            .collect::<Vec<_>>()
            .chunks(8)
            .filter_map(from_binary)
            .collect()
    }
}
