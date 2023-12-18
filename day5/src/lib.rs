#[derive(Debug)]
pub struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Range {
    pub fn new(destination_start: u64, source_start: u64, length: u64) -> Range {
        Range {
            destination_start,
            source_start,
            length,
        }
    }
    pub fn map(&self, input: u64) -> Option<u64> {
        if input < self.source_start || input >= self.source_start + self.length {
            return None;
        }
        Some(self.destination_start + (input - self.source_start))
    }
}
