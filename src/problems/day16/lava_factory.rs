use crate::problems::day16::mirror::MirrorTile;

pub struct LavaFactory {
    grid: Vec<Vec<MirrorTile>>
}

#[derive(Clone, PartialEq)]
struct LaserPoint {
    row: usize,
    col: usize,
    row_vel: i32,
    col_vel: i32
}

impl LavaFactory {
    pub fn parse(contents: &str) -> Option<LavaFactory> {
        let mut grid: Vec<Vec<MirrorTile>> = vec![];

        for line in contents
            .lines()
            .filter(|l| !l.is_empty()) {
            let mut row = vec![];
            for c in line.chars() {
                if let Some(tile) = MirrorTile::parse(c) {
                    row.push(tile);
                } else {
                    return None;
                }
            }
            grid.push(row);
        }

        Some(LavaFactory {
            grid
        })
    }

    pub fn compute_maximum_energy(&self) -> usize {
        let mut maximum_energy = 0;
        for row in 0..self.grid.len() {
            maximum_energy = std::cmp::max(maximum_energy, self.compute_energized_with_start(LaserPoint {
                row,
                col: 0,
                row_vel: 0,
                col_vel: 1
            }));

            maximum_energy = std::cmp::max(maximum_energy, self.compute_energized_with_start(LaserPoint {
                row,
                col: self.grid[row].len() - 1,
                row_vel: 0,
                col_vel: -1
            }));
        }

        for col in 0..self.grid.len() {
            maximum_energy = std::cmp::max(maximum_energy, self.compute_energized_with_start(LaserPoint {
                row: 0,
                col: col,
                row_vel: 1,
                col_vel: 0
            }));

            maximum_energy = std::cmp::max(maximum_energy, self.compute_energized_with_start(LaserPoint {
                row: self.grid.len() - 1,
                col: 0,
                row_vel: -1,
                col_vel: 0
            }));
        }

        maximum_energy
    }

    fn compute_energized_with_start(&self, starting_node: LaserPoint) -> usize {
        let mut energized_grid_points: Vec<LaserPoint> = vec![];
        let mut active_laser_paths = vec![starting_node];
        while let Some(active_laser) = active_laser_paths.pop() {
            if let Some(_) = energized_grid_points.iter().find(|visited_laser| **visited_laser == active_laser) {
                continue; // this path has been explored already. It could end up being recursive, so ignore it -- its already defined
            }

            // I visit this node!
            energized_grid_points.push(active_laser.clone());

            let node = &self.grid[active_laser.row][active_laser.col];
            let new_paths = node.encounter((active_laser.row_vel, active_laser.col_vel));
            for (row_vel, col_vel) in new_paths {
                if (row_vel < 0 && active_laser.row == 0) || (row_vel > 0 && active_laser.row == self.grid.len() - 1) {
                    continue;
                }

                if (col_vel < 0 && active_laser.col == 0) || (col_vel > 0 && active_laser.col == self.grid[active_laser.row].len() - 1) {
                    continue;
                }

                let next_row = (active_laser.row as i32 + row_vel) as usize;
                let next_col = (active_laser.col as i32 + col_vel) as usize;
                active_laser_paths.push(LaserPoint {
                    row: next_row,
                    col: next_col,
                    row_vel,
                    col_vel
                })
            }
        }

        let mut energized_tiles = energized_grid_points.iter().map(|l| (l.row, l.col)).collect::<Vec<(usize, usize)>>();
        energized_tiles.sort();
        energized_tiles.dedup();
        energized_tiles.len()
    }

    pub fn compute_energized(&self) -> usize {
        self.compute_energized_with_start(LaserPoint {
            row: 0,
            col: 0,
            row_vel: 0,
            col_vel: 1
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day16::lava_factory::LavaFactory;

    #[test]
    fn part1() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        let factory = LavaFactory::parse(input).unwrap();
        assert_eq!(46, factory.compute_energized());
    }

    #[test]
    fn part2() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        let factory = LavaFactory::parse(input).unwrap();
        assert_eq!(51, factory.compute_maximum_energy());
    }
}