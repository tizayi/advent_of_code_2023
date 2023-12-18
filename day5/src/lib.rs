#[derive(Debug)]
pub struct Range {
    destination_start: u32,
    source_start: u32,
    length: u32,
}

impl Range {
    pub fn new(destination_start: u32, source_start: u32, length: u32) -> Range {
        Range {
            destination_start,
            source_start,
            length,
        }
    }
    pub fn map(&self, input: u32) -> Option<u32> {
        if input < self.source_start || input >= self.source_start + self.length {
            return None;
        }
        Some(self.destination_start + (input - self.source_start))
    }
}
