use std::fmt;
pub struct EightPointThreeName {
    lfnname: String,
    name: String,
}

impl EightPointThreeName {
    pub fn from(name: String) -> EightPointThreeName {
        convert(name)
    }
}

impl fmt::Display for EightPointThreeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lfnname)
    }
}
