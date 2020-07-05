use std::fmt;
use std::str::Chars;
pub struct EightPointThreeName {
    lfnname: String,
    name: String,
}

impl EightPointThreeName {
    fn from(name: String) -> EightPointThreeName {
        convert(name)
    }
}

impl fmt::Display for EightPointThreeName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.lfnname)
    }
}

fn len_checks(iter: Chars) -> Chars {
    let name: String = iter.collect();
    let sections =  name.rsplitn(2, '.');

    name.chars()
}

pub fn convert(name: String) -> EightPointThreeName {
    let mut result = EightPointThreeName {
        lfnname: name.clone(),
        name: String::new(),
    };
    result.lfnname = name
    .to_ascii_uppercase()
    .chars()
    .filter(|p| p.is_ascii())
    .filter(|p| !p.is_whitespace())
    .collect();
    result
}

#[cfg(test)]
mod tests {
    use super::convert;

    #[test]
    fn test_convert() {
        const FILE_NAMES: [&str; 3] = ["ABCDEFGH.txt", "abcdefgh.txt", "ÄBCDEFGH.txt"];
        for name in FILE_NAMES.iter() {
            println!("{}", convert(name.to_string()));
        }
    }
}
