use std::ops::Range;
use super::almanac_type::AlmanacType;
use super::map_range::MapRange;

pub struct AlmanacMapping {
    pub source_type: AlmanacType,
    pub destination_type: AlmanacType,
    pub ranges: Vec<MapRange>
}

impl AlmanacMapping {
    pub fn map_types(&self, id: &u64) -> u64 {
        if let Some(range) = self.ranges.iter().find(|range| range.does_apply(id)) {
            return range.apply_mapping(id);
        }

        id.clone()
    }

    pub fn map_range_to_destination(&self, range: &Range<u64>) -> Vec<Range<u64>> {
        let mut source_intersecting_ranges: Vec<&MapRange> = self.ranges.iter().filter(|map_range| map_range.does_source_intersect(range)).collect();
        source_intersecting_ranges.sort();

        let mut destination_ranges = vec![];
        let mut remaining_range = Range { start: range.start, end: range.end };
        for source_intersection in source_intersecting_ranges {
            assert!(range.end > source_intersection.source);
            assert!(range.start < source_intersection.source + source_intersection.range_length);

            let intersection_end_bound = source_intersection.source + source_intersection.range_length;
            if remaining_range.start < source_intersection.source {
                // this is in destination format already
                // because there was no intersection before this, source == destination
                destination_ranges.push(Range { start: remaining_range.start, end: source_intersection.source });
                remaining_range.start = source_intersection.source;
            }

            if remaining_range.end <= intersection_end_bound {
                // the entire remaining range is within this sequence, we are done
                destination_ranges.push(source_intersection.build_range(remaining_range.start, remaining_range.end));
                remaining_range.start = remaining_range.end;
            } else {
                // we have leftover
                destination_ranges.push(source_intersection.build_range(remaining_range.start, intersection_end_bound));
                remaining_range.start = intersection_end_bound;
            }
        }

        if !remaining_range.is_empty() {
            destination_ranges.push(remaining_range);
        }

        destination_ranges
    }

    pub fn map_ranges_to_destination(&self, ranges: &Vec<Range<u64>>) -> Vec<Range<u64>> {
        ranges.iter().map(|range| self.map_range_to_destination(range)).flatten().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day5::almanac_mapping::AlmanacMapping;
    use crate::problems::day5::almanac_type::AlmanacType;
    use crate::problems::day5::map_range::MapRange;

    #[test]
    fn map_range_to_destination_maps() {
        let mapping = AlmanacMapping {
            source_type: AlmanacType::Fertilizer,
            destination_type: AlmanacType::Fertilizer,
            ranges: vec![MapRange {
                source: 5,
                destination: 10,
                range_length: 5
            }]
        };

        assert_eq!(vec![10..15], mapping.map_range_to_destination(&(5..10)));
        assert_eq!(vec![0..5], mapping.map_range_to_destination(&(0..5)));
        assert_eq!(vec![10..15], mapping.map_range_to_destination(&(10..15)));
        assert_eq!(vec![0..5, 10..12], mapping.map_range_to_destination(&(0..7)));
        assert_eq!(vec![10..15, 10..15], mapping.map_range_to_destination(&(5..15)));
    }

    #[test]
    fn multiple_ranges_maps() {
        let mapping = AlmanacMapping {
            source_type: AlmanacType::Fertilizer,
            destination_type: AlmanacType::Fertilizer,
            ranges: vec![MapRange {
                source: 5,
                destination: 0,
                range_length: 5
            }, MapRange {
                source: 15,
                destination: 10,
                range_length: 5
            }]
        };

        assert_eq!(vec![10..15], mapping.map_range_to_destination(&(10..15)));
        assert_eq!(vec![0..5], mapping.map_range_to_destination(&(5..10)));
        assert_eq!(vec![10..15], mapping.map_range_to_destination(&(15..20)));
        assert_eq!(vec![0..5, 0..5, 10..15, 10..15, 20..25], mapping.map_range_to_destination(&(0..25)));
    }
}