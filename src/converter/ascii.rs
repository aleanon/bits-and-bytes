use super::{Binary, Converter};

#[derive(Debug)]
pub struct Ascii(pub String);

impl Ascii {
    pub fn new() -> Self {
        Ascii(String::new())
    }
}




impl Converter for Ascii {
    fn to_binary(&self) -> Binary {
        let binary = self.0.chars()
        .map(|char| format!("{:08b}", char as u8))
        .collect::<Vec<String>>()
        .join(" ");

        Binary(binary)
    }
}