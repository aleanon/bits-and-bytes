use super::{Binary, Converter};

#[derive(Debug)]
pub struct Hex(pub String);

impl Hex {
    pub fn new() -> Self {
        Hex(String::new())
    }
}

impl Converter for Hex {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes()
            .iter()
            .filter(|b| !(**b as char).is_ascii_whitespace())
            .copied()
            .collect::<Vec<u8>>()
            .chunks(2)
            .filter_map(|chunk| {
                let str = std::str::from_utf8(chunk).ok()?;
                Some(u8::from_str_radix(str, 16).ok()?)
            })
            .collect()
    }

}