use crate::EightPointThreeName;
use crate::NameTracker;

pub fn convert(name: String, mut tracking: NameTracker) -> EightPointThreeName {
    let mut sections: Vec<String> = name.rsplitn(2, '.').map(|p| String::from(p)).collect();
    sections.reverse();
    for section in &mut sections {
        section.make_ascii_uppercase()
    }
    sections = sections
        .iter()
        .map(|section| {
            section
                .chars()
                .filter(|p| p.is_ascii() && !p.is_whitespace() && !(*p == '.'))
                .collect()
        })
        .collect::<Vec<String>>();
    sections[0].truncate(8);
    sections[1].truncate(3);
    let mut first_six = sections[0].clone();
    first_six.truncate(6);
    let mut name = EightPointThreeName {
        lfnname: name,
        short_name: format!("{}.{}", sections[0], sections[1]),
        first_six: first_six,
        file_extension: sections[1].clone(),
    };
    tracking.register(&name);
    
    if  tracking.get(&name).unwrap() > 5 {
        let new_name = format!("~{}", tracking.get(&name).unwrap());
        name.short_name = name.first_six.clone() + &new_name;
    }
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
            let converted = convert(name.to_string(), NameTracker::new());
            println!("{}", converted.short_name);
            assert_eq!(converted.short_name, "ABCDEFGH.TXT");
        }
    }
}
