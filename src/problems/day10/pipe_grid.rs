use crate::problems::day10::pipe::PipeTile;
use crate::problems::day10::pipe_node::PipeNode;
use crate::problems::day10::position::Position;

pub struct PipeGrid {
    grid: Vec<Vec<PipeNode>>
}

#[derive(Clone)]
enum GridStatus {
    Path,
    LeftSide,
    RightSide,
    Unknown
}

impl PipeGrid {
    pub fn get_node(&self, position: &Position) -> &PipeNode {
        &self.grid[position.row][position.col]
    }
    pub fn parse(input: &str) -> PipeGrid {
        let lines = input.lines().filter(|l| !l.is_empty());
        let unconnected_grid: Vec<Vec<PipeTile>> = lines.map(|l| l.chars().map(|c| PipeTile::parse(c)).collect()).collect();
        let mut connection_grid: Vec<Vec<PipeNode>> = vec![];

        for row in 0..unconnected_grid.len() {
            connection_grid.push(vec![]);
            for col in 0..unconnected_grid[row].len() {
                let tile = &unconnected_grid[row][col];
                let tile_pos = Position::new(row, col);

                let connections = match tile {
                    PipeTile::Vertical => {
                        let mut connections = vec![];
                        // top
                        if row > 0 && unconnected_grid[row - 1][col].has_connections() {
                            connections.push(Position::new(row - 1, col));
                        }

                        // down
                        if row < unconnected_grid.len() - 1 && unconnected_grid[row + 1][col].has_connections() {
                            connections.push(Position::new(row + 1, col));
                        }

                        connections
                    },
                    PipeTile::Horizontal => {
                        let mut connections = vec![];
                        // left
                        if col > 0 && unconnected_grid[row][col - 1].has_connections() {
                            connections.push(Position::new(row, col - 1));
                        }

                        // right
                        if col < unconnected_grid[row].len() - 1 && unconnected_grid[row][col + 1].has_connections() {
                            connections.push(Position::new(row, col + 1));
                        }

                        connections
                    },
                    PipeTile::BendNorthEast => {
                        let mut connections = vec![];
                        // top
                        if row > 0 && unconnected_grid[row - 1][col].has_connections() {
                            connections.push(Position::new(row - 1, col));
                        }

                        // right
                        if col < unconnected_grid[row].len() - 1 && unconnected_grid[row][col + 1].has_connections() {
                            connections.push(Position::new(row, col + 1));
                        }

                        connections
                    },
                    PipeTile::BendNorthWest => {
                        let mut connections = vec![];
                        // top
                        if row > 0 && unconnected_grid[row - 1][col].has_connections() {
                            connections.push(Position::new(row - 1, col));
                        }

                        // left
                        if col > 0 && unconnected_grid[row][col - 1].has_connections() {
                            connections.push(Position::new(row, col - 1));
                        }

                        connections
                    },
                    PipeTile::BendSouthWest => {
                        let mut connections = vec![];

                        // left
                        if col > 0 && unconnected_grid[row][col - 1].has_connections() {
                            connections.push(Position::new(row, col - 1));
                        }

                        // down
                        if row < unconnected_grid.len() - 1 && unconnected_grid[row + 1][col].has_connections() {
                            connections.push(Position::new(row + 1, col));
                        }

                        connections
                    },
                    PipeTile::BendSouthEast => {
                        let mut connections = vec![];

                        // right
                        if col < unconnected_grid[row].len() - 1 && unconnected_grid[row][col + 1].has_connections() {
                            connections.push(Position::new(row, col + 1));
                        }

                        // down
                        if row < unconnected_grid.len() - 1 && unconnected_grid[row + 1][col].has_connections() {
                            connections.push(Position::new(row + 1, col));
                        }

                        connections
                    },
                    PipeTile::Ground => vec![], // special case, no connections
                    PipeTile::StartPosition => {
                        let mut connections = vec![];
                        // top, only a valid path if the node above moves DOWN
                        if row > 0 && unconnected_grid[row - 1][col].travels_down() {
                            connections.push(Position::new(row - 1, col));
                        }

                        // left
                        if col > 0 && unconnected_grid[row][col - 1].travels_right() {
                            connections.push(Position::new(row, col - 1));
                        }

                        // right
                        if col < unconnected_grid[row].len() - 1 && unconnected_grid[row][col + 1].travels_left() {
                            connections.push(Position::new(row, col + 1));
                        }

                        // down
                        if row < unconnected_grid.len() - 1 && unconnected_grid[row + 1][col].travels_up() {
                            connections.push(Position::new(row + 1, col));
                        }

                        connections
                    }
                };

                connection_grid[row].push(PipeNode::new(tile.clone(), tile_pos, connections));
            }
        }
        PipeGrid {
            grid: connection_grid
        }
    }

    pub fn find_start_position(&self) -> &PipeNode {
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                if self.grid[row][col].is_start() {
                    return &self.grid[row][col];
                }
            }
        }

        panic!();
    }

    pub fn find_loop(&self) -> Vec<Position> {
        let start = self.find_start_position();
        for node in &start.connections {
            let mut path: Vec<Position> = vec![start.position.clone(), node.clone()];
            loop {
                let node_under_consideration = self.get_node(&path.last().unwrap());
                if node_under_consideration.is_start() {
                    path.remove(path.len() - 1);
                    return path.clone();
                }

                let connections = &node_under_consideration.connections;
                let previous_node = path.get(path.len() - 2).unwrap();
                let next_node;

                if connections.len() == 2 {
                    if connections[0] == *previous_node {
                        next_node = &connections[1];
                    } else {
                        next_node = &connections[0];
                    }
                } else {
                    break;
                }

                path.push(next_node.clone());
            }
        }

        panic!("No start node was found");
    }

    pub fn find_farthest_distance_from_start(&self) -> usize {
        let path = self.find_loop();
        return (path.len() + 1) / 2;
    }

    pub fn find_area_enclosed_by_loop(&self) -> usize {
        let path = self.find_loop();

        // flood fill every node that isn't part of the path
        // if a node touches the edge, or touches an edge that touches an edge, we know its outside

        // loop has a direction to it.
        // Imagine you have your hand to the wall. You are either inside the wall or outside the wall.

        // step one
        // from the edge, find a node that touches the path
        // you now know you are outside of the loop. Cross the loop.

        // now, stay pointed in a direction, follow the loop
        // we know that every node that touches every node within the loops boundaries is on the INSIDE.
        // Every node that is INSIDE will touch either an inside node or part of the loop
        // queue this node

        // now we have all possible edge nodes
        // so pop them off and check boundaries

        // while queue is not empty
            // for each top, left, bottom ,and right
                // if node is marked, ignore
                // if node is within the path, ignore
                // other node, queue it and mark it

        // now we have marked all inside nodes
        // just return the count of them.

        // there is no possibly where above this is included within the line, so we know that nodes below are within the set

        let mut inside_marked_grid_map: Vec<Vec<GridStatus>> = vec![vec![GridStatus::Unknown; self.grid[0].len()]; self.grid.len()];
        let nodes_to_explore: Vec<Position> = vec![];

        for node in &path {
            inside_marked_grid_map[node.row][node.col] = GridStatus::Path;
        }


        // first we need to figure out which way is left which way is right...
        let first_node = &path[1];
        let last_node = path.last().unwrap();
        let start_tile_is_actually = PipeTile::derive_tile(last_node, first_node);
        for p in &path {
            let node = self.get_node(p);
            let tile = if node.is_start() { &start_tile_is_actually } else { &node.tile };

            let is_going_up = false;
            let is_going_right = false;

            match tile {
                PipeTile::Horizontal => {
                    // mark top as group 1
                    // bottom as group 2
                    if is_going_right {
                        if (p.row > 0) {

                        }
                    } else {

                    }
                },
                PipeTile::Vertical => {
                    // mark left as group 1
                    // mark right as group 2
                },
                PipeTile::BendSouthWest => {
                    // mark right as group 1
                    // mark bottom as group 1
                },
                PipeTile::BendNorthEast => {
                    // mark top as group 1
                    // mark left as group 1
                },
                PipeTile::BendNorthWest => {
                    // mark top as group 1
                    // mark right as group 1
                },
                PipeTile::BendSouthEast => {
                    // mark left as group 1
                    // mark bottom as group 1
                },
                _ => panic!()
            };
        }

        return path.len() / 2;
    }
}

#[cfg(test)]
mod tests {
    use crate::problems::day10::pipe_grid::PipeGrid;

    #[test]
    fn part1_simple_test_input() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";

        let grid = PipeGrid::parse(input);
        assert_eq!(4, grid.find_farthest_distance_from_start());
    }
    #[test]
    fn part1_complex_test_input() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

        let grid = PipeGrid::parse(input);
        assert_eq!(8, grid.find_farthest_distance_from_start());
    }

    #[test]
    fn part2_encloses_test_input() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        let grid = PipeGrid::parse(input);
        assert_eq!(4, grid.find_area_enclosed_by_loop());
    }

    #[test]
    fn part2_encloses_large_test_input() {
        let input = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

        let grid = PipeGrid::parse(input);
        assert_eq!(8, grid.find_area_enclosed_by_loop());
    }
}
