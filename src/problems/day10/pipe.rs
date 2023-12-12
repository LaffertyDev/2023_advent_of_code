#[derive(Clone, PartialEq)]
pub enum PipeTile {
    Vertical,
    Horizontal,
    BendNorthEast,
    BendNorthWest,
    BendSouthWest,
    BendSouthEast,
    Ground,
    StartPosition
}

impl PipeTile {
    pub fn parse(c: char) -> PipeTile {
        match c {
            '|' => PipeTile::Vertical,
            '-' => PipeTile::Horizontal,
            'L' => PipeTile::BendNorthEast,
            'J' => PipeTile::BendNorthWest,
            '7' => PipeTile::BendSouthWest,
            'F' => PipeTile::BendSouthEast,
            '.' => PipeTile::Ground,
            'S' => PipeTile::StartPosition,
            _ => panic!("Invalid pipe tile")
        }
    }

    pub fn has_connections(&self) -> bool {
        *self != PipeTile::Ground
    }

    pub fn travels_down(&self) -> bool {
        *self == PipeTile::Vertical || *self == PipeTile::BendSouthWest || *self == PipeTile::BendSouthEast
    }

    pub fn travels_up(&self) -> bool {
        *self == PipeTile::Vertical || *self == PipeTile::BendNorthWest || *self == PipeTile::BendNorthEast
    }

    pub fn travels_right(&self) -> bool {
        *self == PipeTile::Horizontal || *self == PipeTile::BendNorthEast || *self == PipeTile::BendSouthEast
    }

    pub fn travels_left(&self) -> bool {
        *self == PipeTile::Horizontal || *self == PipeTile::BendNorthWest || *self == PipeTile::BendSouthWest
    }
}