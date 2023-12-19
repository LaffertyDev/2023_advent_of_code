use crate::problems::day18::dig_grid::DigGrid;
use crate::problems::day18::dig_instruction::{DigInstruction};
use crate::problems::shared::grid_2d_direction::Grid2dDirection;

pub struct DigPlan {
    pub instructions: Vec<DigInstruction>
}

impl DigPlan {
    pub fn parse(contents: &str) -> Option<DigPlan> {
        Some(DigPlan {
            instructions: contents
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| DigInstruction::parse(l).unwrap())
                .collect()
        })
    }

    pub fn get_max_down(&self) -> u64 {
        let instructions = self.instructions
            .iter()
            .filter(|i| i.direction == Grid2dDirection::Down || i.direction == Grid2dDirection::Up)
            .map(|i| {
                let multiplier: i64 = if i.direction == Grid2dDirection::Down { 1 } else { -1 };
                i.length as i64 * multiplier
            });

        let mut max_distance = 0;
        let mut current_distance = 0;
        for instruction in instructions {
            current_distance += instruction;
            max_distance = max_distance.max(current_distance);
        }

        max_distance.abs() as u64
    }

    pub fn get_max_up(&self) -> u64 {
        let instructions = self.instructions
            .iter()
            .filter(|i| i.direction == Grid2dDirection::Down || i.direction == Grid2dDirection::Up)
            .map(|i| {
                let multiplier: i64 = if i.direction == Grid2dDirection::Down { 1 } else { -1 };
                i.length as i64 * multiplier
            });

        let mut max_distance = 0;
        let mut current_distance = 0;
        for instruction in instructions {
            current_distance += instruction;
            max_distance = max_distance.min(current_distance);
        }

        max_distance.abs() as u64
    }

    pub fn get_max_left(&self) -> u64 {
        let instructions = self.instructions
            .iter()
            .filter(|i| i.direction == Grid2dDirection::Left || i.direction == Grid2dDirection::Right)
            .map(|i| {
                let multiplier: i64 = if i.direction == Grid2dDirection::Right { 1 } else { -1 };
                i.length as i64 * multiplier
            });

        let mut max_distance = 0;
        let mut current_distance = 0;
        for instruction in instructions {
            current_distance += instruction;
            max_distance = max_distance.min(current_distance);
        }

        max_distance.abs() as u64
    }

    pub fn get_max_right(&self) -> u64 {
        let instructions = self.instructions
            .iter()
            .filter(|i| i.direction == Grid2dDirection::Left || i.direction == Grid2dDirection::Right)
            .map(|i| {
                let multiplier: i64 = if i.direction == Grid2dDirection::Right { 1 } else { -1 };
                i.length as i64 * multiplier
            });

        let mut max_distance = 0;
        let mut current_distance = 0;
        for instruction in instructions {
            current_distance += instruction;
            max_distance = max_distance.max(current_distance);
        }

        max_distance.abs() as u64
    }

    pub fn count_dug_depth(&self) -> u64 {
        let mut grid = DigGrid::build_grid_from_plan(self);
        grid.count_inside()
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day18::dig_plan::DigPlan;

    #[test]
    fn part1() {
        let instructions = "

R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let plan = DigPlan::parse(instructions).unwrap();
        assert_eq!(62, plan.count_dug_depth());
    }
}