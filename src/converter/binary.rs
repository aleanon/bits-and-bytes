use super::Converter;

#[derive(Debug)]
pub struct Binary(pub String);

impl Binary {
    pub fn new() -> Self {
        Binary(String::new())
    }
}


impl Converter for Binary {
    fn to_binary(&self) -> Binary {
        let filtered: String  = self.0.chars()
            .filter(|c| !c.is_ascii_whitespace())
            .collect();
        let padding_len = if filtered.len()%8 == 0 {0} else {8 - filtered.len() % 8};
        let padding = String::from("0").repeat(padding_len);
        let padded = format!("{}{}", padding, filtered);

        Binary(padded)
    }
}