use crate::problems::day18::dig_plan::DigPlan;

#[derive(Copy, Clone, PartialEq)]
enum GridSpace {
    Empty,
    Wall,
    Outside
}

impl GridSpace {
    pub fn pretty_print(&self) {
        match self {
            GridSpace::Empty => print!("."),
            GridSpace::Wall => print!("#"),
            GridSpace::Outside => print!("O")
        }
    }
}

pub struct DigGrid {
    grid: Vec<Vec<GridSpace>>
}

impl DigGrid {
    #[allow(dead_code)]
    pub fn pretty_print(&self) {
        // debug printer
        for row in 0..self.grid.len() {
            for col in 0..self.grid[row].len() {
                self.grid[row][col].pretty_print();
            }

            print!("\r\n");
        }
    }

    #[allow(dead_code)]
    pub fn count_inside(&mut self) -> u64 {
        // first, construct a map of the edges
        // then compute the number of tiles that are inside

        // now map inside/outside
        // basically, any nodes that touch a node on the outside are outside
        let mut outside_nodes = vec![];
        for col in 0..self.grid[0].len() {
            if self.grid[0][col] == GridSpace::Empty {
                outside_nodes.push((0, col));
            }

            if self.grid[self.grid.len() - 1][col] == GridSpace::Empty {
                outside_nodes.push((self.grid.len() - 1, col));
            }
        }

        for row in 0..self.grid.len() {
            if self.grid[row][0] == GridSpace::Empty {
                outside_nodes.push((row, 0));
            }

            if self.grid[row][self.grid[row].len() - 1] == GridSpace::Empty {
                outside_nodes.push((row, self.grid[row].len() - 1));
            }
        }

        while let Some(outside_node) = outside_nodes.pop() {
            let (row, col) = outside_node;
            self.grid[row][col] = GridSpace::Outside;
            if row > 0 {
                // evaluate top
                if self.grid[row - 1][col] == GridSpace::Empty {
                    outside_nodes.push((row - 1, col));
                }
            }

            if row < self.grid.len() - 1 {
                // evaluate below
                if self.grid[row + 1][col] == GridSpace::Empty {
                    outside_nodes.push((row + 1, col));
                }
            }

            if col > 0 {
                // evaluate left
                if self.grid[row][col - 1] == GridSpace::Empty {
                    outside_nodes.push((row, col - 1));
                }
            }

            if col < self.grid[row].len() - 1 {
                // evaluate right
                if self.grid[row][col + 1] == GridSpace::Empty {
                    outside_nodes.push((row, col + 1));
                }
            }
        }

        let mut dug_nodes = 0;
        for row in self.grid.iter() {
            for node in row {
                if node == &GridSpace::Empty || node == &GridSpace::Wall {
                    dug_nodes += 1;
                }
            }
        }

        self.pretty_print();

        dug_nodes
    }

    #[allow(dead_code)]
    pub fn build_grid_from_plan(plan: &DigPlan) -> DigGrid {
        let max_left = plan.get_max_left();
        let max_down = plan.get_max_down();
        let max_right = plan.get_max_right();
        let max_top = plan.get_max_up();

        let mut grid = vec![vec![GridSpace::Empty; (max_left + max_right + 2) as usize]; (max_down + max_top + 2) as usize];

        // build the grid
        let mut cursor_row = (max_top + 1) as usize;
        let mut cursor_col = (max_left + 1) as usize;
        for instruction in plan.instructions.iter() {
            let (vel_row, vel_col) = instruction.direction.as_vector();
            for _ in 0..instruction.length {
                if cursor_row == 0 && vel_row < 0 {
                    panic!();
                }

                if cursor_col == 0 && vel_col < 0 {
                    panic!();
                }
                cursor_row = (cursor_row as i64 + vel_row) as usize;
                cursor_col = (cursor_col as i64 + vel_col) as usize;
                grid[cursor_row][cursor_col] = GridSpace::Wall;
            }
        }

        DigGrid {
            grid
        }
    }
}