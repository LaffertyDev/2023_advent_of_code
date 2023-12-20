pub struct Part {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64
}

#[derive(Clone)]
pub struct PartRange {
    pub x_min: u64,
    pub x_max: u64,
    pub m_min: u64,
    pub m_max: u64,
    pub a_min: u64,
    pub a_max: u64,
    pub s_min: u64,
    pub s_max: u64
}

impl PartRange {
    pub fn new(min: u64, max: u64) -> PartRange {
        PartRange {
            x_min: min,
            x_max: max,
            m_min: min,
            m_max: max,
            a_min: min,
            a_max: max,
            s_min: min,
            s_max: max,
        }
    }
}

impl Part {
    pub fn parse(line: &str) -> Option<Part> {
        //
        let mut nums = vec![];
        for value in line.split([',', '=', '}']) {
            if let Ok(num) = value.parse::<u64>() {
                nums.push(num);
            }
        }

        if nums.len() != 4 {
            return None;
        }

        Some(Part {
            x: nums[0],
            m: nums[1],
            a: nums[2],
            s: nums[3],
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day19::part::Part;

    #[test]
    fn parses() {
        let part = Part::parse("{x=787,m=2655,a=1222,s=2876}").unwrap();
        assert_eq!(787, part.x);
        assert_eq!(2655, part.m);
        assert_eq!(1222, part.a);
        assert_eq!(2876, part.s);
    }
}