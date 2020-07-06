
use crate::EightPointThreeName;
use std::collections::HashMap;
#[derive(Debug)]
pub struct NameTracker {
    tracked: HashMap<String, u32>,
}


impl NameTracker {
    pub fn new() -> NameTracker {
        NameTracker { tracked: HashMap::new() }
    }

    pub fn contains(&self, registrar: &EightPointThreeName) -> bool {
        self.tracked.contains_key(&registrar.first_six)
    }

    pub fn get(&self, registrar: &EightPointThreeName) -> Option<u32> {
        match self.tracked.get(&registrar.first_six) {
            Some(&value) => Some(value),
            None => None,
        }
    }

    pub fn register(&mut self, registrar: &EightPointThreeName) {
        let value;
        {
            value = *self.tracked.entry(registrar.first_six.clone()).or_insert(0);
        }
        self.tracked.insert(registrar.first_six.clone(), value + 1);
    }

    pub fn remove(&mut self, registrar: &EightPointThreeName) {
        self.tracked.insert(registrar.first_six.clone(), *self.tracked.get(&registrar.first_six).unwrap() - 1);
    }
}