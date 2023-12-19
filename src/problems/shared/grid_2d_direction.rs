#[derive(PartialEq, Debug)]
pub enum Grid2dDirection {
    Right,
    Down,
    Left,
    Up
}

impl Grid2dDirection {
    pub fn as_vector(&self) -> (i64, i64) {
        match self {
            Grid2dDirection::Down => (1, 0),
            Grid2dDirection::Left => (0, -1),
            Grid2dDirection::Up => (-1, 0),
            Grid2dDirection::Right => (0, 1),
        }
    }

    pub fn parse(c: char) -> Option<Grid2dDirection> {
        match c {
            'U' => Some(Grid2dDirection::Up),
            'L' => Some(Grid2dDirection::Left),
            'D' => Some(Grid2dDirection::Down),
            'R' => Some(Grid2dDirection::Right),
            _ => None
        }
    }

    pub fn parse_str(c: &str) -> Option<Grid2dDirection> {
        match c {
            "U" => Some(Grid2dDirection::Up),
            "L" => Some(Grid2dDirection::Left),
            "D" => Some(Grid2dDirection::Down),
            "R" => Some(Grid2dDirection::Right),
            _ => None
        }
    }
}