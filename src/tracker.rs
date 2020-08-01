use crate::eight_point_three_name::EightPointThreeName;
use std::collections::HashMap;

#[derive(Debug)]
pub struct NameTracker {
    tracked: HashMap<String, u8>,
}

impl NameTracker {
    pub fn new() -> NameTracker {
        NameTracker {
            tracked: HashMap::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.tracked.len()
    }

    pub fn register(&mut self, registrar: &EightPointThreeName) {
        match self.tracked.get(&registrar.first_six_chars).to_owned() {
            Some(&names) => {
                self.tracked
                    .insert(registrar.first_six_chars.clone(), names + 1);
            }
            None => {
                self.tracked.insert(registrar.first_six_chars.clone(), 1);
            }
        }
    }

    pub fn get(&self, registrar: &EightPointThreeName) -> Option<u8> {
        match self.tracked.get(&registrar.first_six_chars) {
            Some(&count) => Some(count),
            None => None,
        }
    }

    pub fn remove(&mut self, registrar: &EightPointThreeName) -> Result<(), &str> {
        match self.tracked.get(&registrar.first_six_chars) {
            Some(&value) => {
                self.tracked
                    .insert(registrar.first_six_chars.clone(), value - 1);
                Ok(())
            }
            None => Err("No name with first six characters registered"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::eight_point_three_name::EightPointThreeName;
    use crate::tracker::NameTracker;

    fn debug_obj_maker(first_six_chars: String) -> EightPointThreeName {
        EightPointThreeName {
            long_name: String::new(),
            short_name: String::new(),
            first_six_chars,
            file_extension: String::new(),
        }
    }

    #[test]
    fn test_new() {
        NameTracker::new();
    }

    #[test]
    fn test_len() {
        let mut tracker = NameTracker::new();
        for i in 1..=10 {
            println!("{}", i);
            tracker.register(&debug_obj_maker(i.to_string()));
            assert_eq!(tracker.len(), i);
        }
    }
}

/*    #[test]
fn test_register() {
    let mut tracker = NameTracker::new();
    for _i in 0..2 {
        tracker.register(debug_obj_maker());
    }
    assert_eq!(tracker.tracked, )
}*/
