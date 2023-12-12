
#[derive(PartialEq)]
enum Space {
    Galaxy,
    Empty
}

pub struct Universe {
    space: Vec<Vec<Space>>
}

impl Space {
    pub fn parse(c: char) -> Space {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => panic!("unsupported input")
        }
    }
}

impl Universe {
    pub fn parse(content: &str) -> Universe {
        Universe {
            space: content.lines().filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| Space::parse(c)).collect()).collect()
        }
    }

    fn map_galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies_by_id: Vec<(usize, usize)> = vec![];
        for row in 0..self.space.len() {
            for col in 0..self.space[row].len() {
                if self.space[row][col] == Space::Galaxy {
                    galaxies_by_id.push((row, col));
                }
            }
        }

        galaxies_by_id
    }

    fn map_expansion_rows(&self) -> Vec<usize> {
        let mut expansion_rows = vec![];
        for row in 0..self.space.len() {
            let contains_galaxy = self.space[row].contains(&Space::Galaxy);
            if !contains_galaxy {
                expansion_rows.push(row);
            }
        }

        expansion_rows
    }

    fn map_expansion_cols(&self) -> Vec<usize> {
        let mut expansion_cols = vec![];

        for col in 0..self.space[0].len() {
            let mut contains_galaxy = false;
            for row in 0..self.space.len() {
                if self.space[row][col] == Space::Galaxy {
                    contains_galaxy = true;
                    break;
                }
            }

            if !contains_galaxy {
                expansion_cols.push(col);
            }
        }

        expansion_cols
    }

    pub fn find_distance_between_pairs(&self, universe_expansion_factor: usize) -> usize {
        // first, find all galaxies
        let galaxies = self.map_galaxies();
        let expansion_rows = self.map_expansion_rows();
        let expansion_cols = self.map_expansion_cols();

        let mut distance_sum = 0;
        for galaxy_index in 0..galaxies.len() - 1 {
            for compare_index in galaxy_index + 1..galaxies.len() {
                let (g_row, g_col) = galaxies[galaxy_index];
                let (c_row, c_col) = galaxies[compare_index];

                let distance_row = g_row.abs_diff(c_row);
                let distance_col = g_col.abs_diff(c_col);

                let min_row = std::cmp::min(g_row, c_row);
                let max_row = std::cmp::max(g_row, c_row);

                let min_col = std::cmp::min(g_col, c_col);
                let max_col = std::cmp::max(g_col, c_col);

                let mut expansion_rows_to_galaxy = 0;
                for expansion_index in min_row..=max_row {
                    if expansion_rows.contains(&expansion_index) {
                        expansion_rows_to_galaxy += 1;
                    }
                }

                let mut expansion_cols_to_galaxy = 0;
                for expansion_index in min_col..=max_col {
                    if expansion_cols.contains(&expansion_index) {
                        expansion_cols_to_galaxy += 1;
                    }
                }

                distance_sum += distance_row + distance_col - expansion_cols_to_galaxy - expansion_rows_to_galaxy + (expansion_rows_to_galaxy * universe_expansion_factor) + (expansion_cols_to_galaxy * universe_expansion_factor);
            }
        }

        distance_sum
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day11::galaxy::Universe;

    #[test]
    fn part1_test() {
        let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let galaxy = Universe::parse(input);
        assert_eq!(374, galaxy.find_distance_between_pairs(2))
    }

    #[test]
    fn part1_expanded() {
        let input = "
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
        let galaxy = Universe::parse(input);
        assert_eq!(1030, galaxy.find_distance_between_pairs(10))
    }
}