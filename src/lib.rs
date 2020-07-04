use std::fmt;

struct EightPointThreeName {
    name: String
}

impl EightPointThreeName {
    fn from(name: String) -> EightPointThreeName {
        EightPointThreeName { name: name }
    }
}

impl fmt::Display for EightPointThreeName {
    fn fmt(&self, f: &mut fmt::Formatter) ->  fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn convert(name: String) -> EightPointThreeName {
    
    EightPointThreeName::from(String::new())
}