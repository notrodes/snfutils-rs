mod tracker;
mod conversion;
use tracker::NameTracker;
pub struct EightPointThreeName {
    long_name: String,
    short_name: String,
    first_six_chars: String,
    file_extension: String,
}

impl EightPointThreeName {
    pub fn from(name: String) -> EightPointThreeName {
        conversion::convert(name, &mut NameTracker::new())
    }
}
