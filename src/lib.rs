mod tracker;
mod conversion;
use tracker::NameTracker;
pub struct EightPointThreeName {
    lfnname: String,
    short_name: String,
    first_six: String,
    file_extension: String,
}

impl EightPointThreeName {
    pub fn from(name: String) -> EightPointThreeName {
        conversion::convert(name, NameTracker::new())
    }
}
