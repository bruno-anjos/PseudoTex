use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Clone)]
pub enum Values {
    Boolean(bool),
    String(String),
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Values::Boolean(value) => write!(f, "{}", value),
            Values::String(value) => write!(f, "{}", value),
        }
    }
}
