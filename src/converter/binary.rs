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
        Binary(self.0.clone())
    }
}