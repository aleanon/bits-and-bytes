use std::fmt::{self, Display};

use super::{Binary, Converter};

#[derive(Debug)]
pub struct Ascii(pub String);

impl Ascii {
    pub fn new() -> Self {
        Ascii(String::new())
    }
}


impl Converter for Ascii {
    fn to_bytes(&self) -> Vec<u8> {
        self.0.as_bytes().to_owned()
    }

}


impl Display for Ascii {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}