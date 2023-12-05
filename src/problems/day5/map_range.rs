use std::cmp::Ordering;
use std::ops::Range;

#[derive(Clone, Eq)]
pub struct MapRange {
    pub source: u64,
    pub destination: u64,
    pub range_length: u64
}

impl Ord for MapRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source.cmp(&other.source)
    }
}

impl PartialOrd for MapRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MapRange {
    fn eq(&self, other: &Self) -> bool {
        self.source == other.source
    }
}

impl MapRange {
    pub fn does_source_intersect(&self, range: &Range<u64>) -> bool {
        // 1 2 3 4 5 6 7 8 9 10
        //       4 5 6 7 8      <- MapRange Start
        // 1 2 3 4              yes
        //               8 9 10 yes
        //         5 6 7        yes
        // 1 2 3                no
        //                 9 10 no

        let start_index = self.source;
        let end_index = self.source + self.range_length - 1;

        // basically, is the beginning of the range within my set?
        // or is the end of the range in my set?
        (range.start >= start_index && range.start <= end_index) || (range.end <= end_index && range.end >= start_index)
    }

    pub fn does_apply(&self, id: &u64) -> bool {
        // 98, 2
        // contains 98 and 99
        // 100 is not in the range
        self.source <= *id && *id <= self.source + self.range_length - 1
    }

    pub fn apply_mapping(&self, id: &u64) -> u64 {
        // source 98
        // destination 50
        // 98 should go down 48
        if self.source < self.destination {
            // source goes up in value
            return id + self.destination - self.source;
        } else if self.source == self.destination {
            return *id;
        } else {
            // source goes down in value
            return id - (self.source - self.destination);
        }
    }

    pub fn build_range(&self, range_start: u64, range_end: u64) -> Range<u64> {
        // source 98
        // destination 50
        // 98 should go down 48
        if self.source < self.destination {
            // source goes up in value
            let delta = self.destination - self.source;
            return Range {
                start: range_start + delta,
                end: range_end + delta
            };
        } else if self.source == self.destination {
            // no-op
            return Range {
                start: range_start,
                end: range_end
            };
        } else {
            // source goes down in value
            let delta = self.source - self.destination;
            return Range {
                start: range_start - delta,
                end: range_end - delta
            };
        }
    }
}