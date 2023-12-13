#[derive(PartialEq, Debug)]
pub enum Ground {
    Ash,
    Rocks
}

impl Ground {
    pub fn parse(c: char) -> Ground {
        match c {
            '.' => Ground::Ash,
            '#' => Ground::Rocks,
            _ => panic!("unsupported input")
        }
    }
}

pub struct Pattern {
    layout: Vec<Vec<Ground>>
}

impl Pattern {
    pub fn parse(data: &str) -> Pattern {
        let layout = data.lines().filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| Ground::parse(c)).collect()).collect();

        Pattern {
            layout
        }
    }

    pub fn find_vertical_reflection_index_before(&self, reflection_tolerance: usize) -> Option<usize> {
        let num_columns = self.layout[0].len();
        let lower_vertical_reflection = if reflection_tolerance > 0 { self.find_vertical_reflection_index_before(reflection_tolerance - 1) } else { None };

        // go until we think we've found a reflection or until the left_index is at the end
        for left_index in 0..num_columns - 1 {
            if let Some(previous_index) = lower_vertical_reflection {
                if previous_index == left_index {
                    continue; // skip this index
                }
            }
            let mut scan_left = left_index;
            let mut scan_right = left_index + 1;
            loop {
                let mut violating_differences = 0;
                for row in 0..self.layout.len() {
                    if self.layout[row][scan_left] != self.layout[row][scan_right] {
                        violating_differences += 1; // if we changed either to the other, then this could still be a reflection
                    }
                }

                if violating_differences <= reflection_tolerance {
                    if scan_left == 0 || scan_right == num_columns - 1 {
                        // we hit an edge, this is the actual point
                        return Some(left_index);
                    } else {
                        // check next index
                        scan_left -= 1;
                        scan_right += 1;
                    }
                } else {
                    break; // check something else
                }
            }
        }

        None
    }


    pub fn find_horizontal_reflection_index_before(&self, reflection_tolerance: usize) -> Option<usize> {
        // go until we think we've found a reflection or until the left_index is at the end
        let lower_horizontal_reflection = if reflection_tolerance > 0 { self.find_horizontal_reflection_index_before(reflection_tolerance - 1) } else { None };
        for top_index in 0..self.layout.len() - 1 {
            if let Some(previous_index) = lower_horizontal_reflection {
                if previous_index == top_index {
                    continue; // skip this index
                }
            }
            let mut scan_top = top_index;
            let mut scan_bottom = top_index + 1;
            loop {
                let mut violating_differences = 0;
                for col in 0..self.layout[top_index].len() {
                    if self.layout[scan_top][col] != self.layout[scan_bottom][col] {
                        violating_differences += 1;
                    }
                }

                if violating_differences <= reflection_tolerance {
                    if scan_top == 0 || scan_bottom == self.layout.len() - 1 {
                        // we hit an edge, this is the actual point
                        return Some(top_index);
                    } else {
                        // check next index
                        scan_top -= 1;
                        scan_bottom += 1;
                    }
                } else {
                    break; // check something else
                }
            }
        }

        None
    }

    pub fn find_mirror_value(&self, reflection_tolerance: usize) -> usize {
        let vertical_reflection = self.find_vertical_reflection_index_before(reflection_tolerance);
        let horizontal_reflection = self.find_horizontal_reflection_index_before(reflection_tolerance);

        match (vertical_reflection, horizontal_reflection) {
            (None, None) => panic!(),
            (Some(vertical), None) => vertical + 1,
            (None, Some(horizontal)) => (horizontal + 1) * 100,
            (Some(vertical), Some(horizontal)) => {
                // whichever one is closer to the edge wins
                if vertical < horizontal {
                    return vertical + 1;
                } else if vertical > horizontal {
                    return (horizontal + 1) * 100;
                } else {
                    return (horizontal + 1) * 100;
                }
            }
        }
    }
}

pub struct Observation {
    patterns: Vec<Pattern>
}

impl Observation {
    pub fn parse(data: &str) -> Observation {
        let patterns = data.split("\n\n").filter(|p| !p.is_empty());
        let patterns = patterns.map(|p| Pattern::parse(p)).collect();
        Observation {
            patterns
        }
    }

    pub fn find_mirror_values(&self, reflection_tolerance: usize) -> usize {
        self.patterns.iter().map(|p| p.find_mirror_value(reflection_tolerance)).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day13::mirror::Pattern;

    #[test]
    fn part1_test_input() {
        let vertical_input = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let pattern = Pattern::parse(vertical_input);
        assert_eq!(5, pattern.find_mirror_value(0));

        let horizontal_input = "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let pattern = Pattern::parse(horizontal_input);
        assert_eq!(400, pattern.find_mirror_value(0));
    }

    #[test]
    fn part2_test_input() {
        let input1 = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let pattern = Pattern::parse(input1);
        assert_eq!(300, pattern.find_mirror_value(1));

        let input2 = "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let pattern = Pattern::parse(input2);
        assert_eq!(100, pattern.find_mirror_value(1));
    }
}