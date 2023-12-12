use crate::problems::day10::position::Position;

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

    pub fn derive_tile(pos1: &Position, pos2: &Position) -> PipeTile {
        if pos1.row == pos2.row {
            return PipeTile::Horizontal;
        }

        if pos1.col == pos2.col {
            return PipeTile::Vertical;
        }

        // 0,0 either north east
        // 1,1 or south west

        // 0, 0 either north west
        // -1, -1 or south east

        // -1, 1 -> south east
        // 1, -1 -> north west
        // 1, 1 -> north east
        // -1, -1 -> south west


        let row_difference = (pos1.row as i32) - (pos2.row as i32);
        let col_difference = (pos1.col as i32) - (pos2.col as i32);

        match (row_difference, col_difference) {
            (-1, 1) => PipeTile::BendSouthEast,
            (1, -1) => PipeTile::BendNorthWest,
            (1, 1) => PipeTile::BendNorthEast,
            (-1, -1) => PipeTile::BendSouthWest,
            (_, _) => panic!()
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