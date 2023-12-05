use std::ops::Range;
use super::almanac_mapping::AlmanacMapping;
use super::almanac_type::AlmanacType;
use super::map_range::MapRange;

pub struct Almanac {
    pub seeds: Vec<u64>,
    pub seed_ranges: Vec<Range<u64>>,
    pub maps: Vec<AlmanacMapping>
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