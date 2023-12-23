pub struct Range {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Range {
    pub fn new(destination_start: u64, source_start: u64, length: u64)->Range{
        Range {
            destination_start,
            source_start,
            length,
        }
    }
}
pub struct RangeCollection{
    ranges: Vec<Range>
}

impl RangeCollection {
    pub fn new(input_vec: Vec<Range>) -> RangeCollection {
        RangeCollection {
            ranges: input_vec
        }
    }
    pub fn translate_single(&self, input: u64) -> Option<u64> {
        for range in &self.ranges{
            if input < range.source_start || input >= range.source_start + range.length {
                continue;
            }
            return Some(range.destination_start + (input - range.source_start));
        }
        None
    }
}

