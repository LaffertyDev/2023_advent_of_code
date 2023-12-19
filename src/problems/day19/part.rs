pub struct Part {
    pub x: u64,
    pub m: u64,
    pub a: u64,
    pub s: u64
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