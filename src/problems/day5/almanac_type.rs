
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