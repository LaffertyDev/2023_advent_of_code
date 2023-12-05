pub struct Almanac {
    pub seeds: Vec<u64>,
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
}

#[derive(Clone)]
struct MapRange {
    source: u64,
    destination: u64,
    range_length: u64
}

impl MapRange {
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
}

impl Almanac {
    pub fn parse_input(data: &str) -> Almanac {
        let mut data_iter = data.lines();

        let seed_line = data_iter.next().unwrap();
        data_iter.next(); // clear empty line from input...

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
            seeds: seed_line.split_whitespace().skip(1).map(|seed| seed.parse::<u64>().unwrap()).collect(),
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

    pub fn get_lowest_seed_location(&self) -> u64 {
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
        assert_eq!(35, almanac.get_lowest_seed_location());
    }
}