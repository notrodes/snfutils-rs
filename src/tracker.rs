use crate::EightPointThreeName;
use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};
#[derive(Debug)]
pub struct NameTracker {
    tracked: HashMap<String, Vec<EightPointThreeName>>,
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

    pub fn register(&mut self, registrar: EightPointThreeName) -> u32 {
        match self.tracked.get(&registrar.first_six_chars) {
            Some(names) => names.push(registrar),
            None => {
                self.tracked
                    .insert(registrar.first_six_chars, vec![registrar])
                    .unwrap();
            }
        }
        registrar.handle
    }
    pub fn get_vec(&self, registrar: &EightPointThreeName) -> Vec<EightPointThreeName> {
        self.tracked
            .get(&registrar.first_six_chars)
            .unwrap()
            .to_vec()
    }

    pub fn contains(&self, registrar: &EightPointThreeName) -> bool {
        match self.tracked.entry(registrar.first_six_chars) {
            Occupied(name_list) => {
                let name_list = name_list.get();
                name_list.contains(registrar)
            }
            Vacant(_) => false,
        }
    }

    pub fn remove(&mut self, registrar: EightPointThreeName) {
        let entries = *self.tracked.get_mut(&registrar.first_six_chars).unwrap();
        if entries.len() < 1 {
            self.tracked.remove(&registrar.first_six_chars).unwrap();
        } else {
            entries.remove(entries.binary_search(&registrar).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tracker::NameTracker;
    use crate::EightPointThreeName;

    fn debug_obj_maker() -> EightPointThreeName {
        EightPointThreeName {
            long_name: String::new(),
            short_name: String::new(),
            first_six_chars: String::from("ABCD"),
            file_extension: String::new(),
            handle: 0
        }
    }

    #[test]
    fn test_new() {
        NameTracker::new();
    }

    #[test]
    fn test_register() {
        let mut tracker = NameTracker::new();
        for _i in 0..2 {
            tracker.register(debug_obj_maker());
        }
        // assert_eq!(tracker.tracked, )
    }
}
