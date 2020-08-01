mod conversion;
mod tracker;
use tracker::NameTracker;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct EightPointThreeName {
    long_name: String,
    short_name: String,
    first_six_chars: String,
    file_extension: String,
}

impl EightPointThreeName {
    pub fn from(name: String) -> EightPointThreeName {
        conversion::convert(name, &mut NameTracker::new()).clone()
    }

    pub fn get_short_name(&mut self, tracking: &NameTracker) -> String {
        let number_of_siblings = tracking.get_vec(self).len();
        if number_of_siblings <= 6 {
            self.short_name = format!(
                "{}~{}.{}",
                self.first_six_chars.clone(),
                number_of_siblings,
                self.file_extension
            )
        }
        self.short_name.clone()
    }
}
