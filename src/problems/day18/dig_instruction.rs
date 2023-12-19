use crate::problems::shared::grid_2d_direction::Grid2dDirection;

pub struct DigInstruction {
    pub direction: Grid2dDirection,
    pub length: u64,
    pub hex_code: [u8; 3]
}

impl DigInstruction {
    pub fn parse(instruction: &str, is_reversed: bool) -> Option<DigInstruction> {
        let mut split = instruction.split(' ');
        let direction = Grid2dDirection::parse_str(split.next()?)?;
        let length = split.next()?.parse::<u64>().unwrap();
        let hex = split.next()?;
        if is_reversed {
            let hex_length = u64::from_str_radix(&hex[2..=6], 16).ok()?;
            let hex_direction = u8::from_str_radix(&hex[7..=7], 16).ok()?;
            let hex_direction = match hex_direction {
                0 => Grid2dDirection::Right,
                1 => Grid2dDirection::Down,
                2 => Grid2dDirection::Left,
                3 => Grid2dDirection::Up,
                _ => return None
            };
            Some(DigInstruction {
                direction: hex_direction,
                length: hex_length,
                hex_code: [0,0,0]
            })
        } else {
            let r = u8::from_str_radix(&hex[2..=3], 16).ok()?;
            let g = u8::from_str_radix(&hex[4..=5], 16).ok()?;
            let b = u8::from_str_radix(&hex[6..=7], 16).ok()?;
            Some(DigInstruction {
                direction,
                length,
                hex_code: [r,g,b]
            })
        }

    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day18::dig_instruction::{DigInstruction};
    use crate::problems::shared::grid_2d_direction::Grid2dDirection;

    #[test]
    fn parses() {
        let instruction = DigInstruction::parse("D 10 (#218503)", false).unwrap();
        assert_eq!(10, instruction.length);
        assert_eq!(Grid2dDirection::Down, instruction.direction);
        assert_eq!([33, 133, 3], instruction.hex_code);

    }

    #[test]
    fn parses_reversed() {
        let instruction = DigInstruction::parse("D 10 (#70c710)", true).unwrap();
        assert_eq!(461937, instruction.length);
        assert_eq!(Grid2dDirection::Right, instruction.direction);
        assert_eq!([0,0,0], instruction.hex_code);

    }
}