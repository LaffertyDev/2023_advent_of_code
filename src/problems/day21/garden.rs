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
                        println!("This costs less to go through my to get to my neighbor");
                        record.cost = potential_cost;
                        record.point = vertex.point;
                    }
                } else {
                    path_records[neighbor.x][neighbor.y] = Some(GridPointWithCost::new(neighbor, potential_cost));
                    vertices.push(GridPointWithCost::new(neighbor, potential_cost));
                }
            }
        }

        // count possible tokens within this tile
        // count number of repeat tiles (steps / len??)
        // then, count remainder steps remaining for final tile
        // multiple 1 and add the last

        let mut visitable_gardens = 0;
        for row in 0..path_records.len() {
            for col in 0..path_records[row].len() {
                if let Some(record) = path_records[row][col] {
                    if self.grid[record.point.x][record.point.y] != GardenTile::Rock
                        && (record.cost.abs() as u64) <= steps
                        && (record.cost.abs() as u64 % 2 == steps % 2) {
                        visitable_gardens += 1;
                    }
                }
            }
        }


        visitable_gardens
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

    #[test]
    fn part2() {
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
        assert_eq!(16733044, garden.count_garden_plots_reachable_in_steps(5000, true));
    }
}