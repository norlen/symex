#[derive(Debug)]
pub struct Stats {
    /// Number of instructions the executor has processed in total.
    pub instructions_processed: usize,
}

impl Default for Stats {
    fn default() -> Self {
        Self::new()
    }
}

impl Stats {
    pub fn new() -> Self {
        Self {
            instructions_processed: 0,
        }
    }
}
