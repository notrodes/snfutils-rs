use crate::EightPointThreeName;
use std::collections::HashMap;
#[derive(Debug)]
pub struct NameTracker {
    tracked: HashMap<String, u32>,
}

impl NameTracker {
    pub fn new() -> NameTracker {
        NameTracker {
            tracked: HashMap::new(),
        }
    }

    pub fn contains(&self, registrar: &EightPointThreeName) -> bool {
        self.tracked.contains_key(&registrar.first_six_chars)
    }

    pub fn get(&self, registrar: &EightPointThreeName) -> Option<u32> {
        match self.tracked.get(&registrar.first_six_chars) {
            Some(&value) => Some(value),
            None => None,
        }
    }

    pub fn register(&mut self, registrar: &EightPointThreeName) {
        let value;
        {
            value = *self.tracked
                .entry(registrar.first_six_chars.clone())
                .or_insert(0);
        }
        self.tracked
            .insert(registrar.first_six_chars.clone(), value + 1);
    }

    pub fn remove(&mut self, registrar: &EightPointThreeName) {
        let number_of_entries = *self.tracked.get(&registrar.first_six_chars).unwrap();
        if number_of_entries < 1 {
            self.tracked.remove(&registrar.first_six_chars).unwrap();
        } else {
            self.tracked
                .insert(registrar.first_six_chars.clone(), number_of_entries - 1);
        }
    }
}
