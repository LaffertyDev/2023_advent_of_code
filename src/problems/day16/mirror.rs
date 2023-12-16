pub enum MirrorTile {
    // `.`
    EmptySpace,
    // `/`
    Mirror45,
    // `\`
    Mirror135,
    // `|`
    VerticalSplitter,
    // `-`
    HorizontalSplitter
}

impl MirrorTile {
    pub fn parse(c: char) -> Option<MirrorTile> {
        match c {
            '.' => Some(MirrorTile::EmptySpace),
            '/' => Some(MirrorTile::Mirror45),
            '\\' => Some(MirrorTile::Mirror135),
            '-' => Some(MirrorTile::HorizontalSplitter),
            '|' => Some(MirrorTile::VerticalSplitter),
            _ => None
        }
    }


    pub fn encounter(&self, direction: (i32, i32)) -> Vec<(i32, i32)> {
        let (row_movement, col_movement) = direction;
        match self {
            MirrorTile::EmptySpace => {
                return vec![(row_movement, col_movement)];
            },
            MirrorTile::Mirror45 => {
                return vec![(col_movement * -1, row_movement * -1)];
            },
            MirrorTile::Mirror135 => {
                return vec![(col_movement, row_movement)];
            },
            MirrorTile::VerticalSplitter => {
                // if coming from top or bottom, return normal
                if row_movement != 0 {
                    return vec![(row_movement, col_movement)];
                }

                return vec![(-1, 0), (1, 0)];
            },
            MirrorTile::HorizontalSplitter => {
                // if coming from left or right, return normal
                if col_movement != 0 {
                    return vec![(row_movement, col_movement)];
                }

                return vec![(0, -1), (0, 1)];
            },
        }
    }
}