use super::{Binary, Converter};

#[derive(Debug)]
pub struct Hex(pub String);

impl Hex {
    pub fn new() -> Self {
        Hex(String::new())
    }
}

impl Converter for Hex {
    fn to_binary(&self) -> Binary {
        let binary = self.0.chars()
            .filter(|&c|!(c == ' '))
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| {
                let string = chunk.iter().collect::<String>();
                let hex = u16::from_str_radix(&string, 16).expect("Failed to convert hex to binary");
                format!("{:08b}", hex)
            })
            .collect::<Vec<String>>()
            .join(" ");

        Binary(binary)
    }
}