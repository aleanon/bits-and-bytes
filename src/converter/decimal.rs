use super::{Binary, Converter};

#[derive(Debug)]
pub struct Decimal(pub String);

impl Decimal {
    pub fn new() -> Self {
        Self(String::new())
    }
}


impl Converter for Decimal {
    fn to_binary(&self) -> Binary {
        let binary = self.0.chars()
            .filter(|&c|!(c == ' '))
            .collect::<Vec<_>>()
            .chunks(3)
            .filter_map(|chunk| {
                let string = chunk.iter().collect::<String>();
                let dec = u8::from_str_radix(&string, 10).ok()?;
                Some(format!("{:08b}", dec))
            })
            .collect::<Vec<String>>()
            .join(" ");
        
        Binary(binary)
    }
}