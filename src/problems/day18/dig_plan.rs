use crate::problems::day18::dig_instruction::{DigInstruction};
use crate::problems::shared::grid_2d_direction::Grid2dDirection;

pub struct DigPlan {
    pub instructions: Vec<DigInstruction>
}

impl DigPlan {
    pub fn parse(contents: &str, is_reversed: bool) -> Option<DigPlan> {
        Some(DigPlan {
            instructions: contents
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| DigInstruction::parse(l, is_reversed).unwrap())
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
        // https://en.wikipedia.org/wiki/Shoelace_formula
        // https://en.wikipedia.org/wiki/Pick's_theorem

        let mut intersection_points: Vec<(i64, i64)> = vec![];
        intersection_points.push((0, 0));
        let mut surface_area = 0;
        let mut determinant = 0;
        for i in 0..self.instructions.len() { // last node is {0,0} so ignore it
            // origin is (0, 0)
            surface_area += self.instructions[i].length;
            let length: i64 = self.instructions[i].length as i64;
            let (prev_row, prev_col) = intersection_points[intersection_points.len() - 1];
            let (cur_row, cur_col) = match self.instructions[i].direction {
                // 3
                //
                Grid2dDirection::Up => {
                    // (rowprev - length, colprev)
                    (prev_row - length, prev_col)
                },
                Grid2dDirection::Down => {
                    // (rowprev + length, colprev)
                    (prev_row + length, prev_col)
                },
                Grid2dDirection::Left => {
                    // (rowprev, colprev - length)
                    (prev_row, prev_col - length)
                },
                Grid2dDirection::Right => {
                    // (rowprev, colprev + length)
                    (prev_row, prev_col + length)
                },
            };

            intersection_points.push((cur_row, cur_col));
            // ad - bc
            // [ a row1 b row2 ]
            // [ c col1 d col2 ]
            determinant += (prev_row * cur_col) - (cur_row * prev_col);
        }
        let (first_row, first_col) = intersection_points[0];
        let (last_row, last_col) = intersection_points[intersection_points.len() - 1];
        determinant += (first_row * last_col) - (last_row * first_col);

        let inside_edge_surface_area = (determinant.abs() / 2) as u64;

        // A = i + b/2 - 1
        // i = A - b/2 + 1
        // i + b = A + b/2 + 1
        inside_edge_surface_area + (surface_area / 2) as u64 + 1
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
        let plan = DigPlan::parse(instructions, false).unwrap();
        assert_eq!(62, plan.count_dug_depth());
    }

    #[test]
    fn part2() {
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
        let plan = DigPlan::parse(instructions, true).unwrap();
        assert_eq!(952408144115, plan.count_dug_depth());
    }
}