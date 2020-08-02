use crate::tracker::NameTracker;
use crate::EightPointThreeName;

// Character blacklist
const FILTER_PATTERN: [char; 17] = [
    '"', '\'', '*', '+', ',', '/', ':', ';', '<', '=', '>', '?', '\\', '[', ']', '|', ']',
];

pub fn convert(name: String, tracking: &mut NameTracker) -> EightPointThreeName {
    let mut sections: Vec<String> = name.rsplitn(2, '.').map(String::from).collect();
    sections.reverse();
    for section in &mut sections {
        section.retain(|p| p.is_ascii() && !FILTER_PATTERN.contains(&p));
        section.make_ascii_uppercase();
    }
    let file_extension = match sections.get(1) {
        Some(ext) => {
            let mut ext = ext.clone();
            ext.truncate(3);
            Some(ext)
        }
        None => None,
    };
    let first_six_chars = sections[0]
        .get(0..6)
        .unwrap_or_else(|| &sections[0])
        .to_string();
    let mut name = EightPointThreeName {
        long_name: name,
        short_name: String::with_capacity(12),
        first_six_chars: first_six_chars.clone(),
        file_extension,
    };
    tracking.register(&name);
    if sections[0].len() > 8 {
        name.short_name = format!(
            "{}~{}.{}",
            first_six_chars,
            tracking.get(&name).unwrap(),
            name.file_extension.clone().unwrap_or_default()
        );
        name
    } else {
        name.short_name = format!(
            "{}.{}",
            sections[0].clone(),
            name.file_extension.clone().unwrap_or_default()
        );
        name
    }
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
            "ABCDEFGH.TXTABCD",
            "A+BCDEFGH.TXT",
        ];
        println!("Test formatting conversion");
        for name in FILE_NAMES.iter() {
            let converted = convert(name.to_string(), &mut NameTracker::new());
            println!("{}", converted.short_name);
            assert_eq!(converted.short_name, "ABCDEFGH.TXT");
        }
        print!("\n");
    }

    #[test]
    fn test_name_change() {
        println!("Test name over eight characters conversion");
        let mut tracker = NameTracker::new();
        for i in 1..=6 {
            let converted = convert("ABCDEFGHI.TXT".to_string(), &mut tracker);
            println!("{}", converted.short_name);
            assert_eq!(converted.short_name, format!("ABCDEF~{}.TXT", i));
        }
        print!("\n");
    }

    #[test]
    fn test_no_extension() {
        println!("Test no extension in conversion\n");
        convert("ABCDEFGHI".to_string(), &mut NameTracker::new());
    }
}
