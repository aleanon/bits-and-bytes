use super::Converter;

#[derive(Debug)]
pub struct Binary(pub String);

impl Binary {

    pub fn new() -> Self {
        Binary(String::new())
    }
}


impl Converter for Binary {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
            .iter()
            .filter(|b|!(**b as char).is_ascii_whitespace())
            .copied()
            .collect::<Vec<u8>>()
            .chunks(8)
            .filter_map(|chunk| {
                let byte = u8::from_str_radix(std::str::from_utf8(chunk).ok()?, 2).ok()?;
                Some(byte)
            })
            .collect()
    }
}