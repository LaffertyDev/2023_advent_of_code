use std::fmt;

#[derive(PartialEq)]
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

#[derive(PartialEq)]
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

    fn print(&self) {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                print!("{}", self.grid[row][col])
            }
            print!("\n");
        }
        print!("\n");
    }

    // idea 1 was using dynamic programming
    // but the load counts are never the same

    // idea 2 -- instead of iterating over the array N to determine cycle length
    // go node-by-node
        // if I hit a rounded node, it joins my movement group
        // update all nodes in the movement group at once

    pub fn spin(&mut self, cycles: usize) {
        let directions = vec![TiltDirection::Up, TiltDirection::Left, TiltDirection::Down, TiltDirection::Right];
        for cycle in 0..cycles {
            if cycle % 10000 == 0 {
                println!("Cycle {}%. {} / {}", (cycle as f64 / cycles as f64) * 100f64, cycle, cycles);
            }

            let direction = &directions[cycle % 4];
            self.tilt(direction);
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
        grid.print();
        grid.tilt(&TiltDirection::Up);
        grid.print();
        assert_eq!(136, grid.compute_load());
    }
}