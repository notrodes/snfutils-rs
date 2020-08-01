use crate::EightPointThreeName;
use crate::NameTracker;

pub fn convert(name: String, tracking: &mut NameTracker) -> EightPointThreeName {
    let mut sections: Vec<String> = name.rsplitn(2, '.').map(String::from).collect();
    sections.reverse();
    for section in &mut sections {
        *section = section
            .to_ascii_uppercase()
            .chars()
            .filter(|p| p.is_ascii() && !p.is_whitespace() && !(*p == '.'))
            .collect::<String>();
    }
    sections[0].truncate(8);
    sections[1].truncate(3);
    let mut first_six = sections[0].clone();
    first_six.truncate(6);
    let name = EightPointThreeName {
        long_name: name,
        short_name: format!("{}.{}", sections[0], sections[1]),
        first_six_chars: first_six,
        file_extension: sections[1].clone(),
    };
    tracking.register(name.clone());
    name
}

#[cfg(test)]
mod tests {
    use super::convert;
    use crate::tracker::NameTracker;

    #[test]
    fn test_convert() {
        const FILE_NAMES: [&str; 5] = [
            "ABCDEFGH.TXT",
            "abcdefgh.TXT",
            "ÄABCDEFGH.TXT",
            "ABCDEFGHIJ.TXT",
            "ABCDEFGHI.TXTABCD",
        ];
        for name in FILE_NAMES.iter() {
            let converted = convert(name.to_string(), &mut NameTracker::new());
            println!("{}", converted.short_name);
            assert_eq!(converted.short_name, "ABCDEFGH.TXT");
        }
    }

    #[test]
    fn test_name_change() {
        let mut tracker = NameTracker::new();
        for i in 1..=6 {
            let mut converted = convert("ABCDEFGH.TXT".to_string(), &mut tracker);
            assert_eq!(converted.get_short_name(&tracker), format!("ABCDEF~{}.TXT", i));
        }
    }
}
