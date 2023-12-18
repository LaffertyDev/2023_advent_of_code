use std::collections::HashMap;
use std::collections::VecDeque;

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

impl PointedPoint {
    fn get_index(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    #[allow(dead_code)]
    fn get_previous(&self) -> (usize, usize) {
        let prev_node_row = ((self.row as i64) - self.vel_row) as usize;
        let prev_node_col = ((self.col as i64) - self.vel_col) as usize;
        (prev_node_row, prev_node_col)
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct ExplorationVertex {
    point: PointedPoint,
    same_direction_count: usize
}

impl FactoryCity {
    pub fn parse(contents: &str) -> FactoryCity {
        FactoryCity {
            grid: contents.lines().filter(|l| !l.is_empty()).map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect()).collect()
        }
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


    pub fn compute_lowest_heat_loss(&self) -> u64 {
        // start in top left
        // goal is bottom right
        let end_row = self.grid.len() - 1;
        let end_col = self.grid[0].len() - 1;

        let mut vertices: VecDeque<ExplorationVertex> = VecDeque::new();
        vertices.push_front(ExplorationVertex {
            point: PointedPoint {
                row: 0,
                col: 0,
                vel_row: 0,
                vel_col: 1,
            },
            same_direction_count: 1
        });

        let mut minimum_pathing_map: HashMap<(usize, usize), u64> = HashMap::new();
        while let Some(vertex) = vertices.pop_front() {
            if vertex.point.row == end_row && vertex.point.col == end_col {
                // we've reached the end
                // if I implemented it correctly, we're done now
                break;
            }

            let eligible_paths = [
                FactoryCity::rotate_velocity_clockwise((vertex.point.vel_row, vertex.point.vel_col)),
                FactoryCity::rotate_velocity_counter_clockwise((vertex.point.vel_row, vertex.point.vel_col)),
                (vertex.point.vel_row, vertex.point.vel_col)
            ];

            for (p_vel_row, p_vel_col) in eligible_paths {
                let direction_count = if p_vel_row == vertex.point.vel_row && p_vel_col == vertex.point.vel_col { vertex.same_direction_count + 1 } else { 1 };
                if (p_vel_row < 0 && vertex.point.row == 0) || (p_vel_row > 0 && vertex.point.row == end_row) {
                    continue;
                }

                if (p_vel_col < 0 && vertex.point.col == 0) || (p_vel_col > 0 && vertex.point.col == end_col) {
                    continue;
                }

                if direction_count > 3 {
                    continue;
                }

                let next_node_row = ((vertex.point.row as i64) + p_vel_row) as usize;
                let next_node_col = ((vertex.point.col as i64) + p_vel_col) as usize;

                // the cost to get to the next node is currently equal to the cost to myself
                let cost_to_me = minimum_pathing_map.get(&vertex.point.get_index()).unwrap_or(&0);
                let potential_cost = *cost_to_me + self.grid[next_node_row][next_node_col];

                let next_node = ExplorationVertex {
                    point: PointedPoint {
                        row: next_node_row,
                        col: next_node_col,
                        vel_row: p_vel_row,
                        vel_col: p_vel_col,
                    },
                    same_direction_count: direction_count
                };

                let has_explored = minimum_pathing_map.contains_key(&next_node.point.get_index());

                minimum_pathing_map.entry(next_node.point.get_index()).and_modify(|current_cost| {
                    // is this path a better path than current?
                    if potential_cost < *current_cost {
                        *current_cost = potential_cost
                    }
                }).or_insert(potential_cost);

                if !has_explored {
                    vertices.push_back(next_node.clone()); // todo rust smell clone should be unnecessary
                }

                vertices.make_contiguous().sort_unstable_by(|a, b| {
                    let a_cost =  minimum_pathing_map.get(&a.point.get_index()).unwrap();
                    let b_cost =  minimum_pathing_map.get(&b.point.get_index()).unwrap();
                    a_cost.cmp(b_cost)
                });
            }
        }

        return *minimum_pathing_map.get(&(end_row, end_col)).unwrap();
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
        assert_eq!(102, factory_city.compute_lowest_heat_loss());
    }


    #[test]
    fn simple() {
        let input = "
111
191
111
111
111
";
        let factory_city = FactoryCity::parse(input);
        assert_eq!(6, factory_city.compute_lowest_heat_loss());
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