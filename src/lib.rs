mod conversion;
mod tracker;
use tracker::NameTracker;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct EightPointThreeName {
    long_name: String,
    short_name: String,
    first_six_chars: String,
    file_extension: String,
    handle: u32
}

impl EightPointThreeName {
    pub fn from(name: String) -> EightPointThreeName {
        conversion::convert(name, &mut NameTracker::new())
    }
}
