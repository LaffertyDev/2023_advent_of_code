pub enum CamelDirection {
    Left,
    Right
}

impl CamelDirection {
    pub fn parse(c: char) -> CamelDirection {
        match c {
            'L' => CamelDirection::Left,
            'R' => CamelDirection::Right,
            _ => panic!("invalid direction"),
        }
    }
}