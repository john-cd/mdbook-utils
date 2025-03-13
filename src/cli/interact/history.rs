use dialoguer::BasicHistory;
use dialoguer::History;

pub(super) struct MyHistory {
    history: BasicHistory,
}

impl MyHistory {
    /// Creates a new history value
    pub(super) fn new() -> Self {
        Self::default()
    }
}

impl Default for MyHistory {
    fn default() -> Self {
        Self {
            history: BasicHistory::new().no_duplicates(true),
        }
    }
}

impl<T: ToString> History<T> for MyHistory {
    fn read(&self, pos: usize) -> Option<String> {
        <BasicHistory as History<T>>::read(&self.history, pos)
    }

    fn write(&mut self, val: &T) {
        self.history.write(val);
    }
}
