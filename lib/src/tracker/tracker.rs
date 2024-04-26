use super::Tracker;

impl Tracker {
    /// Create a new instance of `Tracker`
    pub fn new(url: String) -> Self {
        Self {
            url,
            ..Default::default()
        }
    }
}
