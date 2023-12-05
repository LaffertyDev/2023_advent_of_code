use std::cmp::Ordering;
use std::ops::Range;

#[derive(Clone, Debug, Eq)]
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

        if range.is_empty() { return false; }

        // ranges are half-open, so need to account for that
        let source_range_start_index = self.source;
        let source_range_last_index = self.source + self.range_length - 1;

        // does set completely enclose?
        // does start lie within my range?
        // does end lie within my range?

        // basically, is the beginning of the range within my set?
        // or is the end of the range in my set?
        (range.start <= source_range_start_index && range.end >= source_range_last_index) || (range.start >= source_range_start_index && range.start <= source_range_last_index) || (range.end - 1 <= source_range_last_index && range.end - 1 >= source_range_start_index)
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
            // source goes up in value (s50, d98, up 48)
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
            // source goes down in value (s98, d50, down 48)
            let delta = self.source - self.destination;
            return Range {
                start: range_start - delta,
                end: range_end - delta
            };
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::problems::day5::map_range::MapRange;

    #[test]
    fn does_source_intersect_zero() {
        let range = MapRange {
            source: 0, // 0,1,2,3,4,5,6,7,8,9
            destination: 10000,
            range_length: 10
        };

        assert!(range.does_source_intersect(&(0..10)));
        assert!(range.does_source_intersect(&(0..1)));
        assert!(range.does_source_intersect(&(1..5)));
        assert!(range.does_source_intersect(&(2..5)));
        assert!(range.does_source_intersect(&(7..10)));
        assert!(range.does_source_intersect(&(7..12)));
        assert!(range.does_source_intersect(&(9..12)));
        assert!(range.does_source_intersect(&(0..2000)));
        assert!(!range.does_source_intersect(&(0..0)));
        assert!(!range.does_source_intersect(&(10..13)));
        assert!(!range.does_source_intersect(&(15..22)));
        assert!(!range.does_source_intersect(&(10..10)));
    }

    #[test]
    fn does_source_intersect_nonzero() {
        let range = MapRange {
            source: 5,
            destination: 10000,
            range_length: 10
        };

        assert!(!range.does_source_intersect(&(0..4)));
        assert!(!range.does_source_intersect(&(0..5)));
        assert!(range.does_source_intersect(&(0..6)));
        assert!(range.does_source_intersect(&(5..10)));

        assert!(range.does_source_intersect(&(0..30)));

        assert!(range.does_source_intersect(&(6..10)));
        assert!(range.does_source_intersect(&(6..7)));
        assert!(range.does_source_intersect(&(5..15)));

        assert!(range.does_source_intersect(&(13..14)));
        assert!(range.does_source_intersect(&(13..15)));
        assert!(range.does_source_intersect(&(13..200)));
        assert!(range.does_source_intersect(&(14..16)));
        assert!(!range.does_source_intersect(&(15..16)));
        assert!(!range.does_source_intersect(&(20..22)));
    }

    #[test]
    fn build_range_greater_builds() {
        let range = MapRange {
            source: 5,
            destination: 50,
            range_length: 10
        };

        assert_eq!(50..60, range.build_range(5, 15));
        assert_eq!(55..59, range.build_range(10, 14));
    }

    #[test]
    fn build_range_smaller_builds() {
        let range = MapRange {
            source: 50,
            destination: 5,
            range_length: 10
        };

        assert_eq!(5..15, range.build_range(50, 60));
        assert_eq!(5..10, range.build_range(50, 55));
    }

    #[test]
    fn build_range_same_same() {
        let range = MapRange {
            source: 50,
            destination: 50,
            range_length: 10
        };

        assert_eq!(50..60, range.build_range(50, 60));
        assert_eq!(10..20, range.build_range(10, 20));
    }
}