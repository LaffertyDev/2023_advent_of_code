use std::collections::{HashSet};
use crate::problems::shared::grid_2d::{GridPointWithCost};
use crate::problems::shared::grid_2d_direction::Grid2dDirection;
use crate::problems::shared::grid_point_2d::GridPoint2D;

#[derive(PartialEq)]
enum ScenicTile {
    Path,
    Forest,
    Slope(Grid2dDirection)
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Path {
    nodes: Vec<GridPoint2D>,
    cost: usize
}

impl ScenicTile {
    fn parse(c: char) -> ScenicTile {
        match c {
            '.' => ScenicTile::Path,
            '#' => ScenicTile::Forest,
            '^' => ScenicTile::Slope(Grid2dDirection::Up),
            '>' => ScenicTile::Slope(Grid2dDirection::Right),
            '<' => ScenicTile::Slope(Grid2dDirection::Left),
            'v' => ScenicTile::Slope(Grid2dDirection::Down),
            _ => unreachable!()
        }
    }
}

pub struct ScenicPark {
    grid: Vec<Vec<ScenicTile>>,
    vertices: Vec<(GridPoint2D, Vec<GridPointWithCost>)>
}

impl ScenicPark {
    pub fn parse(contents: &str, are_sloped_climbable: bool) -> ScenicPark {
        let grid: Vec<Vec<ScenicTile>> = contents.lines().filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| ScenicTile::parse(c)).collect()).collect();

        let mut parsed_vertices: Vec<(GridPoint2D, Vec<GridPointWithCost>)> = vec![];
        for row in 0..grid.len() {
            for col in 0..grid[row].len() {
                if grid[row][col] != ScenicTile::Forest {
                    let mut adj = vec![];
                    if row > 0 {
                        // top
                        if grid[row - 1][col] != ScenicTile::Forest && (grid[row][col] == ScenicTile::Path || (are_sloped_climbable || grid[row][col] == ScenicTile::Slope(Grid2dDirection::Up))) {
                            adj.push(GridPointWithCost::new(GridPoint2D::new(row - 1, col), 1));
                        }
                    }

                    if row < grid.len() - 1 {
                        // bottom
                        if grid[row + 1][col] != ScenicTile::Forest && (grid[row][col] == ScenicTile::Path || (are_sloped_climbable || grid[row][col] == ScenicTile::Slope(Grid2dDirection::Down))) {
                            adj.push(GridPointWithCost::new(GridPoint2D::new(row + 1, col), 1));
                        }
                    }

                    if col > 0 {
                        // left
                        if grid[row][col - 1] != ScenicTile::Forest && (grid[row][col] == ScenicTile::Path || (are_sloped_climbable || grid[row][col] == ScenicTile::Slope(Grid2dDirection::Left))) {
                            adj.push(GridPointWithCost::new(GridPoint2D::new(row, col - 1), 1));
                        }
                    }

                    if col < grid[row].len() - 1 {
                        // right
                        if grid[row][col + 1] != ScenicTile::Forest && (grid[row][col] == ScenicTile::Path || (are_sloped_climbable || grid[row][col] == ScenicTile::Slope(Grid2dDirection::Right))) {
                            adj.push(GridPointWithCost::new(GridPoint2D::new(row, col + 1), 1));
                        }
                    }

                    parsed_vertices.push((GridPoint2D::new(row, col), adj));
                }
            }
        }

        let mut did_update = true;
        while did_update {
            did_update = false;
            let mut x = 0;
            while x < parsed_vertices.len() - 1 {
                if grid[parsed_vertices[x].0.x][parsed_vertices[x].0.y] == ScenicTile::Path && parsed_vertices[x].1.len() == 2 {
                    did_update = true;
                    let current_node = parsed_vertices[x].0.clone();
                    let neighbor1 = parsed_vertices[x].1[0];
                    let neighbor2 = parsed_vertices[x].1[1];

                    for n in 0..parsed_vertices.len() {
                        if parsed_vertices[n].0 == neighbor1.point {
                            // the vertex of neighbor1 matching self needs updated to be neighbor2
                            parsed_vertices[n].1.iter_mut().find(|neighbor| neighbor.point == current_node).and_then(|neighbor| {
                                neighbor.point = neighbor2.point;
                                // cost is equal to the cost from {prev -> curr + 1}
                                neighbor.cost = neighbor1.cost + neighbor2.cost;
                                return Some(neighbor);
                            });
                        }

                        if parsed_vertices[n].0 == neighbor2.point {
                            parsed_vertices[n].1.iter_mut().find(|neighbor| neighbor.point == current_node).and_then(|neighbor| {
                                neighbor.point = neighbor1.point;
                                neighbor.cost = neighbor1.cost + neighbor2.cost;
                                return Some(neighbor);
                            });
                        }
                    }

                    parsed_vertices.remove(x);
                } else {
                    x += 1;
                }
            }
        }

        ScenicPark {
            grid: grid,
            vertices: parsed_vertices
        }
    }

    pub fn get_start_tile(&self) -> GridPoint2D {
        return GridPoint2D::new(0, 1);
    }

    pub fn get_end_tile(&self) -> GridPoint2D {
        return GridPoint2D::new(self.grid.len() - 1, self.grid[self.grid.len() - 1].len() - 2);
    }

    pub fn find_most_scenic_route(&self) -> u64 {
        let start = self.get_start_tile();
        let end = self.get_end_tile();

        let mut paths = Vec::new();
        paths.push(Path {
            nodes: vec![start],
            cost: 0
        });

        let mut end_paths = vec![];
        let mut current_max = 0;

        while let Some(path) = paths.pop() {
            let last_node = path.nodes.last().unwrap();
            if last_node == &end {
                if path.cost > current_max {
                    current_max = path.cost;
                    println!("Found longer path: {}. Vertex Length: {}", current_max, path.nodes.len());
                }
                end_paths.push(path);
                continue;
            }

            // this is way too slow, but whatever.
            let (_, neighbors) = self.vertices.iter().find(|(p, _)| p == last_node).unwrap();

            for neighbor in neighbors {
                if path.nodes.contains(&neighbor.point) {
                    continue;
                }

                let mut new_path = path.nodes.clone();
                new_path.push(neighbor.point.clone());
                let new_path = Path {
                    nodes: new_path,
                    cost: path.cost + neighbor.cost as usize
                };
                paths.push(new_path);
            }
        }

        return end_paths.iter().map(|p| p.cost).max().unwrap() as u64;
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day23::scenery::ScenicPark;

    #[test]
    fn part1() {
        let input = "
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

        let park = ScenicPark::parse(input, false);
        assert_eq!(94, park.find_most_scenic_route());
    }
}