use std::cmp::Ordering;
use crate::problems::day10::pipe::PipeTile;
use crate::problems::day10::pipe_node::PipeNode;
use crate::problems::day10::position::Position;

pub struct PipeGrid {
    grid: Vec<Vec<PipeNode>>
}

#[derive(Clone, PartialEq)]
enum GridStatus {
    Path,
    GroupA,
    GroupB,
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
                    break; // this path is a dead-end. Ignore it.
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

    pub fn get_node_east_of(&self, node_pos: &Position) -> Option<Position> {
        if node_pos.col == self.grid[0].len() - 1 {
            return None;
        }

        return Some(Position::new(node_pos.row, node_pos.col + 1));
    }

    pub fn get_node_west_of(&self, node_pos: &Position) -> Option<Position> {
        if node_pos.col == 0 {
            return None;
        }

        return Some(Position::new(node_pos.row, node_pos.col - 1));
    }

    pub fn get_node_north_of(&self, node_pos: &Position) -> Option<Position> {
        if node_pos.row == 0 {
            return None;
        }

        return Some(Position::new(node_pos.row - 1, node_pos.col));
    }
    pub fn get_node_south_of(&self, node_pos: &Position) -> Option<Position> {
        if node_pos.row == self.grid.len() - 1 {
            return None;
        }

        return Some(Position::new(node_pos.row + 1, node_pos.col));
    }

    pub fn find_area_enclosed_by_loop(&self) -> usize {
        let path = self.find_loop();

        let mut inside_marked_grid_map: Vec<Vec<GridStatus>> = vec![vec![GridStatus::Unknown; self.grid[0].len()]; self.grid.len()];
        for p in &path {
            inside_marked_grid_map[p.row][p.col] = GridStatus::Path;
        }

        let mut nodes_to_expand: Vec<Position> = vec![];
        for index in 0..path.len() {
            let current_node = &path[index];
            let next_node = if index < path.len() - 1 { &path[index + 1] } else { &path[0] };

            // Group A is Left going vertical
            // Group B is Right going vertical
            let vertical_direction = current_node.row.cmp(&next_node.row);
            let horizontal_direction = current_node.col.cmp(&next_node.col);

            let north = self.get_node_north_of(current_node).and_then(|node| if inside_marked_grid_map[node.row][node.col] != GridStatus::Path { Some(node) } else { None });
            let west = self.get_node_west_of(current_node).and_then(|node| if inside_marked_grid_map[node.row][node.col] != GridStatus::Path { Some(node) } else { None });
            let south = self.get_node_south_of(current_node).and_then(|node| if inside_marked_grid_map[node.row][node.col] != GridStatus::Path { Some(node) } else { None });
            let east = self.get_node_east_of(current_node).and_then(|node| if inside_marked_grid_map[node.row][node.col] != GridStatus::Path { Some(node) } else { None });
            match self.grid[current_node.row][current_node.col].tile {
                PipeTile::Horizontal => {
                    if let Some(node) = north {
                        inside_marked_grid_map[node.row][node.col] = if horizontal_direction == Ordering::Greater { GridStatus::GroupA } else { GridStatus::GroupB };
                        nodes_to_expand.push(node);
                    }
                    if let Some(node) = south {
                        inside_marked_grid_map[node.row][node.col] = if horizontal_direction == Ordering::Less { GridStatus::GroupA } else { GridStatus::GroupB };
                        nodes_to_expand.push(node);
                    }
                },
                PipeTile::Vertical => {
                    if let Some(node) = west {
                        inside_marked_grid_map[node.row][node.col] = if vertical_direction == Ordering::Less { GridStatus::GroupA } else { GridStatus::GroupB };
                        nodes_to_expand.push(node);
                    }
                    if let Some(node) = east {
                        inside_marked_grid_map[node.row][node.col] = if vertical_direction == Ordering::Greater { GridStatus::GroupA } else { GridStatus::GroupB };
                        nodes_to_expand.push(node);
                    }
                },
                PipeTile::BendNorthEast => {
                    let nodes_are = if current_node.row == next_node.row { GridStatus::GroupA } else { GridStatus::GroupB };
                    if let Some(node) = west {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                    if let Some(node) = north {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                },
                PipeTile::BendNorthWest => {
                    let nodes_are = if current_node.col == next_node.col { GridStatus::GroupA } else { GridStatus::GroupB };
                    if let Some(node) = east {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                    if let Some(node) = north {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                },
                PipeTile::BendSouthEast => {
                    let nodes_are = if current_node.col == next_node.col { GridStatus::GroupA } else { GridStatus::GroupB };
                    if let Some(node) = west {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                    if let Some(node) = south {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                },
                PipeTile::BendSouthWest => {
                    let nodes_are = if current_node.row == next_node.row { GridStatus::GroupA } else { GridStatus::GroupB };
                    if let Some(node) = east {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                    if let Some(node) = south {
                        inside_marked_grid_map[node.row][node.col] = nodes_are.clone();
                        nodes_to_expand.push(node);
                    }
                },
                PipeTile::StartPosition => {
                    // I really don't want to derive the start tile
                    //println!("Ignoring start position. I can't imagine a state where this would matter.");
                }
                _ => panic!("Impossible State")
            }
        }

        let mut is_group_a_edge = None;
        while let Some(node) = nodes_to_expand.pop() {
            let adjacent_nodes = vec![self.get_node_west_of(&node), self.get_node_south_of(&node), self.get_node_east_of(&node), self.get_node_north_of(&node)];
            for adjacent in adjacent_nodes {
                if let Some(adjacent_pos) = adjacent {
                    if inside_marked_grid_map[adjacent_pos.row][adjacent_pos.col] == GridStatus::Unknown {
                        if inside_marked_grid_map[node.row][node.col] == GridStatus::Path {
                            println!("Broken: {}, {}", node.row, node.col);
                        }
                        inside_marked_grid_map[adjacent_pos.row][adjacent_pos.col] = inside_marked_grid_map[node.row][node.col].clone();
                        nodes_to_expand.push(adjacent_pos);
                    }
                } else {
                    if inside_marked_grid_map[node.row][node.col] != GridStatus::Path {
                        is_group_a_edge = if inside_marked_grid_map[node.row][node.col] == GridStatus::GroupA { Some(true) } else { Some(false) };
                    }
                }
            }
        }

        // pretty print
        // for row in &inside_marked_grid_map {
        //     print!("\r\n");
        //     for node in row {
        //         let dbg = match node {
        //             GridStatus::GroupA => 'A',
        //             GridStatus::GroupB => 'B',
        //             GridStatus::Unknown => '*',
        //             GridStatus::Path => 'P',
        //         };
        //         print!("{}", dbg);
        //     }
        // }
        // print!("\r\n");

        let mut inside_nodes = 0;
        if let Some(is_group_a_edge) = is_group_a_edge {
            let group_that_is_inside = if is_group_a_edge { GridStatus::GroupB } else { GridStatus::GroupA };
            for row in &inside_marked_grid_map {
                for node in row {
                    if node == &group_that_is_inside {
                        inside_nodes += 1;
                    }
                }
            }
        } else {
            println!("Did not find an edge... I don't even know what to do here. Just count non-empty nodes?")
        }

        inside_nodes
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
    fn part2_squeeze() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........";


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
