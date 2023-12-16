use std::collections::HashMap;
use std::fmt;

#[derive(PartialEq, Hash, Eq, Clone)]
pub enum Tile {
    CubeRock,
    Empty,
    RoundedRock,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Tile::CubeRock => '#',
            Tile::Empty => '.',
            Tile::RoundedRock => 'O'
        };
        write!(f, "{}", c)
    }
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum TiltDirection {
    Up,
    Down,
    Left,
    Right
}

impl Tile {
    pub fn parse(c: char) -> Option<Tile> {
        match c {
            '#' => Some(Tile::CubeRock),
            '.' => Some(Tile::Empty),
            'O' => Some(Tile::RoundedRock),
            _ => None
        }
    }
}

pub struct Platform {
    grid: Vec<Vec<Tile>>
}

impl Platform {
    pub fn parse(contents: &str) -> Option<Platform> {
        Some(Platform {
            grid: contents.lines().filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| Tile::parse(c).unwrap()).collect()).collect()
        })
    }

    pub fn compute_load(&self) -> usize {
        let mut load = 0;
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == Tile::RoundedRock {
                    load += self.grid.len() - row;
                }

            }
        }
        load
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                print!("{}", self.grid[row][col])
            }
            print!("\n");
        }
        print!("\n");
    }

    pub fn spin(&mut self, cycles_to_run: usize) {
        let mut cycle_solutions = HashMap::new();
        let mut cycle_index = 0;
        while cycle_index < cycles_to_run {
            let directions = vec![TiltDirection::Up, TiltDirection::Left, TiltDirection::Down, TiltDirection::Right];
            for direction in directions {
                self.tilt(&direction);
                if let Some(cycle_starts_at_idx) = cycle_solutions.get(&(self.grid.to_vec())) {
                    // advance the index to the index N cycles ahead
                    let cycle_length = cycle_index - cycle_starts_at_idx;
                    let number_of_cycles = (cycles_to_run - cycle_starts_at_idx) / cycle_length;
                    let final_cycle_start_index = number_of_cycles * cycle_length + cycle_starts_at_idx;
                    cycle_index = final_cycle_start_index;
                } else {
                    cycle_solutions.insert(self.grid.to_vec(), cycle_index);
                }
            }

            cycle_index += 1;
        }
    }

    pub fn tilt(&mut self, direction: &TiltDirection) {
        match direction {
            TiltDirection::Down => {
                for row in (0..self.grid.len()).rev() {
                    for col in 0..self.grid[row].len() {
                        if self.grid[row][col] == Tile::RoundedRock {
                            let mut tilt = row;
                            while tilt < self.grid.len() - 1 && self.grid[tilt + 1][col] == Tile::Empty {
                                self.grid[tilt + 1][col] = Tile::RoundedRock; // make new tile me
                                self.grid[tilt][col] = Tile::Empty; // make old tile empty
                                tilt += 1;
                            }
                        }
                    }
                }
            },
            TiltDirection::Up => {
                for row in 0..self.grid.len() {
                    for col in 0..self.grid[row].len() {
                       if self.grid[row][col] == Tile::RoundedRock {
                           let mut tilt = row;
                           while tilt > 0 && self.grid[tilt - 1][col] == Tile::Empty {
                               self.grid[tilt - 1][col] = Tile::RoundedRock; // make new tile me
                               self.grid[tilt][col] = Tile::Empty; // make old tile empty
                               tilt -= 1;
                           }
                       }
                    }
                }
            },
            TiltDirection::Left => {
                for row in 0..self.grid.len() {
                    for col in 0..self.grid[row].len() {
                        if self.grid[row][col] == Tile::RoundedRock {
                            let mut tilt = col;
                            while tilt > 0 && self.grid[row][tilt - 1] == Tile::Empty {
                                self.grid[row][tilt - 1] = Tile::RoundedRock; // make new tile me
                                self.grid[row][tilt] = Tile::Empty; // make old tile empty
                                tilt -= 1;
                            }
                        }
                    }
                }
            },
            TiltDirection::Right => {
                for row in 0..self.grid.len() {
                    for col in (0..self.grid[row].len()).rev() {
                        if self.grid[row][col] == Tile::RoundedRock {
                            let mut tilt = col;
                            while tilt < self.grid[row].len() - 1 && self.grid[row][tilt + 1] == Tile::Empty {
                                self.grid[row][tilt + 1] = Tile::RoundedRock; // make new tile me
                                self.grid[row][tilt] = Tile::Empty; // make old tile empty
                                tilt += 1;
                            }
                        }
                    }
                }
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::problems::day14::mirror_platform::{Platform, TiltDirection};

    #[test]
    fn part1() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let mut grid = Platform::parse(input).unwrap();
        grid.tilt(&TiltDirection::Up);
        assert_eq!(136, grid.compute_load());
    }

    #[test]
    fn part2() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
        let mut grid = Platform::parse(input).unwrap();
        grid.spin(1000000000);

        assert_eq!(64, grid.compute_load());
    }
}