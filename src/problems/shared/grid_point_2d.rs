use crate::problems::shared::grid_2d::GridTile;

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct GridPoint2D {
    pub x: usize,
    pub y: usize
}

impl GridPoint2D {
    pub fn new(row: usize, col: usize) -> GridPoint2D {
        GridPoint2D {
            x: row,
            y: col
        }
    }
}

impl GridTile for GridPoint2D {
    fn get_adjacents(&self, x_boundary: usize, y_boundary: usize) -> Vec<GridPoint2D> {
        let mut adjacents = vec![];
        if self.x > 0 {
            adjacents.push(GridPoint2D::new(self.x - 1, self.y));
        }

        if self.x < x_boundary - 1 {
            adjacents.push(GridPoint2D::new(self.x + 1, self.y));
        }

        if self.y > 0 {
            adjacents.push(GridPoint2D::new(self.x, self.y - 1));
        }

        if self.y < y_boundary - 1 {
            adjacents.push(GridPoint2D::new(self.x, self.y + 1));
        }
        adjacents
    }
}