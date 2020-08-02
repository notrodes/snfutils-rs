mod conversion;
pub mod tracker;

use tracker::NameTracker;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct EightPointThreeName {
    pub long_name: String,
    pub short_name: String,
    pub first_six_chars: String,
    pub file_extension: String,
}

pub fn from(name: String) -> EightPointThreeName {
    conversion::convert(name, &mut NameTracker::new()).clone()
}

pub fn from_with_tracker(name: String, mut tracker: &mut NameTracker) -> EightPointThreeName {
    conversion::convert(name, &mut tracker)
}
