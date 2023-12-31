use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct FactoryCity {
    grid: Vec<Vec<u64>>
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct PointedPoint {
    row: usize,
    col: usize,
    vel_row: i64,
    vel_col: i64
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct ExplorationVertex {
    point: PointedPoint,
    same_direction_count: usize
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct ExplorationVortexWithCost {
    vertex: ExplorationVertex,
    cost: u64,
    potential_total_cost: u64
}

impl PartialOrd for ExplorationVortexWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cost = self.cost as i64 * -1;
        let other_cost = other.cost as i64 * -1;
        Some(cost.cmp(&other_cost))
    }
}

impl Ord for ExplorationVortexWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        let cost = self.cost as i64 * -1;
        let other_cost = other.cost as i64 * -1;
        cost.cmp(&other_cost)
    }
}

impl FactoryCity {
    pub fn parse(contents: &str) -> FactoryCity {
        FactoryCity {
            grid: contents.lines().filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()).collect()
        }
    }

    fn get_smallest_cost_to_point<'a>(map: &'a HashMap<ExplorationVertex, u64>, row: usize, col: usize, min_path: usize) -> Option<u64> {
        let mut smallest_cost = None;
        for (p, c) in map.iter() {
            if p.point.row == row && p.point.col == col && (smallest_cost.is_none() || *c < smallest_cost.unwrap() && p.same_direction_count >= min_path) {
                smallest_cost = Some(*c);
            }
        }

        return smallest_cost;
    }

    fn rotate_velocity_clockwise(vel: (i64, i64)) -> (i64, i64) {
        let (row_vel, col_vel) = vel;

        // top, go right    {-1,  0} -> { 0,  1}
        // right, go down   { 0,  1} -> { 1,  0}
        // down, go left    { 1,  0} -> { 0, -1}
        // left, go top     { 0, -1} -> {-1,  0}

        (col_vel, row_vel * -1)
    }

    fn rotate_velocity_counter_clockwise(vel: (i64, i64)) -> (i64, i64) {
        let (row_vel, col_vel) = vel;

        // top, go left    {-1,  0} -> { 0, -1}
        // left, go down   { 0, -1} -> { 1,  0}
        // down, go right  { 1,  0} -> { 0,  1}
        // right, go top   { 0,  1} -> {-1,  0}

        (col_vel * -1, row_vel)
    }


    pub fn compute_lowest_heat_loss(&self, minimum_stopping_distance: usize, maximum_straight: usize) -> u64 {
        // start in top left
        // goal is bottom right
        let end_row = self.grid.len() - 1;
        let end_col = self.grid[0].len() - 1;

        let mut vertices: BinaryHeap<ExplorationVortexWithCost> = BinaryHeap::new();
        vertices.push(ExplorationVortexWithCost {
            vertex: ExplorationVertex {
                point: PointedPoint {
                    row: 0,
                    col: 0,
                    vel_row: 0,
                    vel_col: 1,
                },
                same_direction_count: 1
            },
            cost: 0,
            potential_total_cost: 0
        });

        let mut minimum_pathing_map: HashMap<ExplorationVertex, u64> = HashMap::new();
        let mut exploration_set = HashSet::new();
        while let Some(vertex) = vertices.pop() {
            if vertex.vertex.point.row == end_row && vertex.vertex.point.col == end_col {
                // we've reached the end
                // if I implemented it correctly, we're done now
                break;
            }

            let eligible_directions = [
                (FactoryCity::rotate_velocity_clockwise((vertex.vertex.point.vel_row, vertex.vertex.point.vel_col)), 1),
                (FactoryCity::rotate_velocity_counter_clockwise((vertex.vertex.point.vel_row, vertex.vertex.point.vel_col)), 1),
                ((vertex.vertex.point.vel_row, vertex.vertex.point.vel_col), vertex.vertex.same_direction_count + 1)
            ];

            for ((p_vel_row, p_vel_col), duration) in eligible_directions {
                // determine minimum distance forward to get to a valid destination node
                // then check to see if its valid
                let minimum_nodes_forward = if duration >= minimum_stopping_distance { 1 } else { minimum_stopping_distance + 1 - duration };
                let forward_row = (p_vel_row * minimum_nodes_forward as i64).abs() as usize;
                let forward_column = (p_vel_col * minimum_nodes_forward as i64).abs() as usize;
                if (p_vel_row < 0 && vertex.vertex.point.row < forward_row) || (p_vel_row > 0 && vertex.vertex.point.row + forward_row > end_row) {
                    continue; // not enough room to go in this direction
                }

                if (p_vel_col < 0 && vertex.vertex.point.col < forward_column) || (p_vel_col > 0 && vertex.vertex.point.col + forward_column > end_col) {
                    continue; // not enough room to go in this direction
                }

                if vertex.vertex.same_direction_count < minimum_stopping_distance && p_vel_row != vertex.vertex.point.vel_row && p_vel_col != vertex.vertex.point.vel_col {
                    continue; // cannot turn yet, skip it
                }

                if p_vel_row == vertex.vertex.point.vel_row && p_vel_col == vertex.vertex.point.vel_col && duration > maximum_straight {
                    // discard this node if the minimum forward distance reaches an edge
                    continue; // cannot continue straight
                }

                let next_node_row = ((vertex.vertex.point.row as i64) + p_vel_row) as usize;
                let next_node_col = ((vertex.vertex.point.col as i64) + p_vel_col) as usize;

                let potential_cost = vertex.cost + self.grid[next_node_row][next_node_col];

                let next_node = ExplorationVortexWithCost {
                    vertex: ExplorationVertex {
                        point: PointedPoint {
                            row: next_node_row,
                            col: next_node_col,
                            vel_row: p_vel_row,
                            vel_col: p_vel_col,
                        },
                        same_direction_count: duration
                    },
                    cost: potential_cost,
                    potential_total_cost: potential_cost + (end_row - next_node_row) as u64 + (end_col - next_node_col) as u64
                };

                minimum_pathing_map.entry(next_node.vertex.clone()).and_modify(|current_cost| {
                    if potential_cost < *current_cost {
                        *current_cost = potential_cost
                    }
                }).or_insert(potential_cost);

                if !exploration_set.contains(&next_node.vertex) {
                    exploration_set.insert(next_node.vertex.clone());
                    vertices.push(next_node);
                }
            }
        }

        return FactoryCity::get_smallest_cost_to_point(&minimum_pathing_map, end_row, end_col, minimum_stopping_distance).unwrap();
    }
}


#[cfg(test)]
mod tests {
    use crate::problems::day17::factory_city::FactoryCity;

    #[test]
    fn part1() {
        let input = "

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533

";
        let factory_city = FactoryCity::parse(input);
        assert_eq!(102, factory_city.compute_lowest_heat_loss(1, 3));
    }

    #[test]
    fn part2() {
        let input = "

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533

";
        let factory_city = FactoryCity::parse(input);
        assert_eq!(94, factory_city.compute_lowest_heat_loss(4, 10));
    }

    #[test]
    fn part2_tests() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991
";
        let factory_city = FactoryCity::parse(input);
        assert_eq!(71, factory_city.compute_lowest_heat_loss(4, 10));
    }

    #[test]
    fn rotates_rotates() {
        assert_eq!((0, 1), FactoryCity::rotate_velocity_clockwise((-1, 0)));
        assert_eq!((1, 0), FactoryCity::rotate_velocity_clockwise((0, 1)));
        assert_eq!((0, -1), FactoryCity::rotate_velocity_clockwise((1, 0)));
        assert_eq!((-1, 0), FactoryCity::rotate_velocity_clockwise((0, -1)));

        assert_eq!((0, -1), FactoryCity::rotate_velocity_counter_clockwise((-1, 0)));
        assert_eq!((1, 0), FactoryCity::rotate_velocity_counter_clockwise((0, -1)));
        assert_eq!((0, 1), FactoryCity::rotate_velocity_counter_clockwise((1, 0)));
        assert_eq!((-1, 0), FactoryCity::rotate_velocity_counter_clockwise((0, 1)));
    }
}
