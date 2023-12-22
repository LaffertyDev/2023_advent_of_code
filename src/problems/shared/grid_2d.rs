use std::cmp::Ordering;
use crate::problems::shared::grid_point_2d::GridPoint2D;

#[allow(dead_code)]
struct Grid2D<TTile> {
    grid: Vec<Vec<TTile>>
}

pub trait GridTile {
    fn get_adjacents(&self, x_boundary: usize, y_boundary: usize) -> Vec<GridPoint2D>;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GridPointWithCost {
    pub point: GridPoint2D,
    pub cost: i64
}

impl GridPointWithCost {
    pub fn new(point: GridPoint2D, cost: i64) -> GridPointWithCost {
        GridPointWithCost {
            point,
            cost
        }
    }
}

impl PartialOrd for GridPointWithCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cost = self.cost;
        let other_cost = other.cost;
        Some(cost.cmp(&other_cost))
    }
}

impl Ord for GridPointWithCost {
    fn cmp(&self, other: &Self) -> Ordering {
        let cost = self.cost;
        let other_cost = other.cost;
        cost.cmp(&other_cost)
    }
}