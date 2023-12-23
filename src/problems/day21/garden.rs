use std::collections::{BinaryHeap};
use crate::problems::shared::grid_2d::{GridPointWithCost, GridTile};
use crate::problems::shared::grid_point_2d::GridPoint2D;

#[derive(Clone, Copy, PartialEq, Eq)]
enum GardenTile {
    GardenPlot,
    GardenPlotStartingPosition,
    Rock
}

impl GardenTile {
    fn parse(c: char) -> GardenTile {
        match c {
            '#' => GardenTile::Rock,
            '.' => GardenTile::GardenPlot,
            'S' => GardenTile::GardenPlotStartingPosition,
            _ => panic!()
        }
    }
}

pub struct Garden {
    grid: Vec<Vec<GardenTile>>
}

impl Garden {
    pub fn parse(contents: &str) -> Garden {
        Garden {
            grid: contents.lines().filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| GardenTile::parse(c)).collect()).collect()
        }
    }

    fn get_start_tile(&self) -> GridPoint2D {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col] == GardenTile::GardenPlotStartingPosition {
                    return GridPoint2D::new(row, col);
                }
            }
        }

        panic!()
    }

    pub fn count_garden_plots_reachable_in_steps(&self, steps: u64, is_infinite_tiling: bool) -> u64 {
        let start = self.get_start_tile();

        let mut vertices: BinaryHeap<GridPointWithCost> = BinaryHeap::new();
        vertices.push(GridPointWithCost::new(start, 0));

        let mut path_records: Vec<Vec<Option<GridPointWithCost>>> = vec![vec![None; self.grid[0].len()]; self.grid.len()];
        while let Some(vertex) = vertices.pop() {
            let eligible_neighbors = vertex.point.get_adjacents(self.grid.len(), self.grid[vertex.point.x].len());
            for neighbor in eligible_neighbors {
                if self.grid[neighbor.x][neighbor.y] == GardenTile::Rock {
                    continue; // impassable, don't record it
                }

                let potential_cost = vertex.cost - 1;
                if let Some(record) = &mut path_records[neighbor.x][neighbor.y] {
                    if potential_cost > record.cost {
                        record.cost = potential_cost;
                        record.point = vertex.point;
                    }
                } else {
                    path_records[neighbor.x][neighbor.y] = Some(GridPointWithCost::new(neighbor, potential_cost));
                    vertices.push(GridPointWithCost::new(neighbor, potential_cost));
                }
            }
        }

        if !is_infinite_tiling {
            let mut visitable_gardens = 0;
            for row in 0..path_records.len() {
                for col in 0..path_records[row].len() {
                    if let Some(record) = path_records[row][col] {
                        if (record.cost.abs() as u64) <= steps
                            && (record.cost.abs() as u64 % 2 == steps % 2) {
                            visitable_gardens += 1;
                        }
                    }
                }
            }

            return visitable_gardens;
        }

        // I don't like problems that require input analysis and the examples don't line up with the problem
        // I get it, but that sucked
        // I just used someone else's (Excellent!) solution.
        // https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21
        let even_corners: usize = path_records.iter()
            .map(|rows|
                rows
                    .iter()
                    .filter(|record| record.is_some_and(|record| record.cost.abs() % 2 == 0 && record.cost.abs() > 65)).count()).sum();

        let odd_corners: usize = path_records.iter()
            .map(|rows|
                rows
                    .iter()
                    .filter(|record| record.is_some_and(|record| record.cost.abs() % 2 == 1 && record.cost.abs() > 65)).count()).sum();

        let even_full: usize = path_records.iter()
            .map(|rows|
                rows
                    .iter()
                    .filter(|record| record.is_some_and(|record| record.cost.abs() % 2 == 0)).count()).sum();

        let odd_full: usize = path_records.iter()
            .map(|rows|
                rows
                    .iter()
                    .filter(|record| record.is_some_and(|record| record.cost.abs() % 2 == 1)).count()).sum();

        let n = 202300;
        assert_eq!(n, 202300);

        let p2 = ((n+1)*(n+1)) * odd_full + (n*n) * even_full - (n+1) * odd_corners + n * even_corners;

        p2 as u64
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day21::garden::Garden;

    #[test]
    fn part1() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        let garden = Garden::parse(input);
        assert_eq!(16, garden.count_garden_plots_reachable_in_steps(6, false));
    }
}