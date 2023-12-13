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

    pub fn find_vertical_reflection_index_before(&self) -> Option<usize> {
        let num_columns = self.layout[0].len();

        // go until we think we've found a reflection or until the left_index is at the end
        for left_index in 0..num_columns - 1 {
            let mut scan_left = left_index;
            let mut scan_right = left_index + 1;
            loop {
                let mut is_reflection = true;
                for row in 0..self.layout.len() {
                    is_reflection &= self.layout[row][scan_left] == self.layout[row][scan_right];
                }

                if is_reflection {
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


    pub fn find_horizontal_reflection_index_before(&self) -> Option<usize> {
        // go until we think we've found a reflection or until the left_index is at the end
        for top_index in 0..self.layout.len() - 1 {
            let mut scan_top = top_index;
            let mut scan_bottom = top_index + 1;
            loop {
                let mut is_reflection = true;
                for col in 0..self.layout[top_index].len() {
                    is_reflection &= self.layout[scan_top][col] == self.layout[scan_bottom][col];
                }

                if is_reflection {
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

    pub fn find_mirror_value(&self) -> usize {
        if let Some(vertical) = self.find_vertical_reflection_index_before() {
            return vertical + 1;
        }

        if let Some(horizontal) = self.find_horizontal_reflection_index_before() {
            return (horizontal + 1) * 100;
        }

        0
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

    pub fn find_mirror_values(&self) -> usize {
        self.patterns.iter().map(|p| p.find_mirror_value()).sum()
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
        assert_eq!(5, pattern.find_mirror_value());

        let horizontal_input = "
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let pattern = Pattern::parse(horizontal_input);
        assert_eq!(400, pattern.find_mirror_value());
    }
}