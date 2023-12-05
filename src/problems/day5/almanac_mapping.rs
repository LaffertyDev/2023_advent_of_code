use std::ops::Range;
use crate::problems::day5::almanac_type::AlmanacType;
use crate::problems::day5::map_range::MapRange;

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
        let mut intersecting_mappings: Vec<&MapRange> = self.ranges.iter().filter(|map_range| map_range.does_source_intersect(range)).collect();
        intersecting_mappings.sort();
        let mut resulting_ranges = vec![];
        let mut remaining_range = Range { start: range.start, end: range.end };
        for intersection in intersecting_mappings {
            let intersection_end_index = intersection.source + intersection.range_length - 1;
            // we exhaust all ranges simultaneously
            if remaining_range.start < intersection.source {
                // this is in destination format already
                // because there was no intersection before this, source == destination
                resulting_ranges.push(Range { start: remaining_range.start, end: intersection.source });
                remaining_range.start = intersection.source;
            }

            if remaining_range.end < intersection_end_index {
                // the entire remaining range is within this sequence, we are done
                resulting_ranges.push(intersection.build_range(remaining_range.start, remaining_range.end));
                remaining_range.start = remaining_range.end;
            } else {
                // we have leftover
                resulting_ranges.push(intersection.build_range(remaining_range.start, intersection_end_index));
                remaining_range.start = intersection_end_index;
            }
        }

        if !remaining_range.is_empty() {
            resulting_ranges.push(remaining_range);
        }

        resulting_ranges
    }

    pub fn map_ranges_to_destination(&self, ranges: &Vec<Range<u64>>) -> Vec<Range<u64>> {
        ranges.iter().map(|range| self.map_range_to_destination(range)).flatten().collect()
    }
}