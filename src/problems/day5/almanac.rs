use std::cmp::Ordering;
use std::ops::Range;

pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_ranges: Vec<Range<u64>>,
    pub maps: Vec<AlmanacMapping>
}

#[derive(Clone, Debug, PartialEq)]
pub enum AlmanacType {
    Seeds,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl AlmanacType {
    pub fn parse_from_string(almanac: &str) -> AlmanacType {
        match almanac {
            "seed" => AlmanacType::Seeds,
            "soil" => AlmanacType::Soil,
            "fertilizer" => AlmanacType::Fertilizer,
            "water" => AlmanacType::Water,
            "light" => AlmanacType::Light,
            "temperature" => AlmanacType::Temperature,
            "humidity" => AlmanacType::Humidity,
            "location" => AlmanacType::Location,
            val => panic!("Unsupported Value for Almanac: {}", val)
        }
    }
}

pub struct AlmanacMapping {
    source_type: AlmanacType,
    destination_type: AlmanacType,
    ranges: Vec<MapRange>
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



#[derive(Clone, Eq)]
struct MapRange {
    source: u64,
    destination: u64,
    range_length: u64
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

impl Almanac {
    pub fn parse_input(data: &str) -> Almanac {
        let mut data_iter = data.lines();

        let seeds: Vec<u64> = data_iter.next().unwrap().split_whitespace().skip(1).map(|seed| seed.parse::<u64>().unwrap()).collect();
        data_iter.next(); // clear empty line from input...

        let mut seed_ranges: Vec<Range<u64>> = vec![];
        for index in (0..seeds.len()).step_by(2) {
            seed_ranges.push(Range {
                start: seeds[index],
                end: seeds[index] + seeds[index + 1] // half-inclusive, so same as range
            })
        }

        let mut ranges = vec![];
        let mut start_almanac: Option<AlmanacType> = None;
        let mut end_almanac: Option<AlmanacType> = None;
        let mut almanac_ranges = vec![];
        for line in data_iter {
            if line.is_empty() {
                if start_almanac.is_some() {
                    // start a new almanac
                    let start = start_almanac.unwrap();
                    let end = end_almanac.unwrap();
                    ranges.push(AlmanacMapping {
                        source_type: start.clone(),
                        destination_type: end.clone(),
                        ranges: almanac_ranges.clone()
                    });

                    start_almanac = None;
                    end_almanac = None;
                    almanac_ranges.clear();
                }
            }
            else if start_almanac.is_none() {
                // next line is the start/stop range
                let mut word_iter = line.split(|c| c == '-' || c == ' ');
                start_almanac = Some(AlmanacType::parse_from_string(word_iter.next().unwrap()));
                word_iter.next(); // ignore 'to'
                end_almanac = Some(AlmanacType::parse_from_string(word_iter.next().unwrap()));
            } else {
                // we're finding ranges
                let data: Vec<u64> = line.split(' ').map(|entry| entry.parse::<u64>().unwrap()).collect();
                almanac_ranges.push(MapRange {
                    destination: data[0],
                    source: data[1],
                    range_length: data[2]
                })
            }
        }

        if start_almanac.is_some() {
            // start a new almanac
            let start = start_almanac.unwrap();
            let end = end_almanac.unwrap();
            ranges.push(AlmanacMapping {
                source_type: start.clone(),
                destination_type: end.clone(),
                ranges: almanac_ranges.clone()
            });
        }

        Almanac {
            seeds: seeds,
            seed_ranges: seed_ranges,
            maps: ranges
        }
    }

    pub fn map_source_to_destination(&self, source: AlmanacType, destination: AlmanacType, source_ids_to_map: &Vec<u64>) -> Vec<u64> {
        let map = self.maps
            .iter()
            .find(|m| m.source_type == source && m.destination_type == destination)
            .unwrap_or_else(|| panic!("No map found for {:?}/{:?} combo", source, destination));

        return source_ids_to_map.iter().map(|id| map.map_types(id)).collect();
    }

    pub fn map_source_ranges_to_destination(&self, source: AlmanacType, destination: AlmanacType, source_ranges: &Vec<Range<u64>>) -> Vec<Range<u64>> {
        let map = self.maps
            .iter()
            .find(|m| m.source_type == source && m.destination_type == destination)
            .unwrap_or_else(|| panic!("No map found for {:?}/{:?} combo", source, destination));

        return map.map_ranges_to_destination(source_ranges);
    }

    pub fn get_lowest_seed_location_from_seed_list(&self) -> u64 {
        let seeds = &self.seeds;

        let soil = self
            .map_source_to_destination(AlmanacType::Seeds, AlmanacType::Soil, seeds);

        let fertilizer = self
            .map_source_to_destination(AlmanacType::Soil, AlmanacType::Fertilizer, &soil);

        let water = self
            .map_source_to_destination(AlmanacType::Fertilizer, AlmanacType::Water, &fertilizer);

        let light = self
            .map_source_to_destination(AlmanacType::Water, AlmanacType::Light, &water);

        let temperature = self
            .map_source_to_destination(AlmanacType::Light, AlmanacType::Temperature, &light);

        let humidity = self
            .map_source_to_destination(AlmanacType::Temperature, AlmanacType::Humidity, &temperature);

        let location = self
            .map_source_to_destination(AlmanacType::Humidity, AlmanacType::Location, &humidity);

        return location.iter().min().unwrap().clone();
    }

    pub fn get_lowest_seed_location_from_seed_ranges(&self) -> u64 {
        let seeds = &self.seed_ranges;

        let soil = self
            .map_source_ranges_to_destination(AlmanacType::Seeds, AlmanacType::Soil, seeds);

        let fertilizer = self
            .map_source_ranges_to_destination(AlmanacType::Soil, AlmanacType::Fertilizer, &soil);

        let water = self
            .map_source_ranges_to_destination(AlmanacType::Fertilizer, AlmanacType::Water, &fertilizer);

        let light = self
            .map_source_ranges_to_destination(AlmanacType::Water, AlmanacType::Light, &water);

        let temperature = self
            .map_source_ranges_to_destination(AlmanacType::Light, AlmanacType::Temperature, &light);

        let humidity = self
            .map_source_ranges_to_destination(AlmanacType::Temperature, AlmanacType::Humidity, &temperature);

        let location = self
            .map_source_ranges_to_destination(AlmanacType::Humidity, AlmanacType::Location, &humidity);

        return location.iter().map(|r| r.start).min().unwrap().clone();
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day5::almanac::Almanac;

    #[test]
    fn test_input_functions() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let almanac = Almanac::parse_input(&input);
        assert_eq!(vec![79, 14, 55, 13], almanac.seeds);
        assert_eq!(2, almanac.seed_ranges.len());
        assert_eq!(79, almanac.seed_ranges[0].start);
        assert_eq!(93, almanac.seed_ranges[0].end);
        assert_eq!(55, almanac.seed_ranges[1].start);
        assert_eq!(68, almanac.seed_ranges[1].end);
        assert_eq!(35, almanac.get_lowest_seed_location_from_seed_list());
        assert_eq!(46, almanac.get_lowest_seed_location_from_seed_ranges());
    }
}