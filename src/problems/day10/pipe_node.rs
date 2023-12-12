use crate::problems::day10::pipe::PipeTile;
use crate::problems::day10::position::Position;


pub struct PipeNode {
    pub tile: PipeTile,
    pub position: Position,
    pub connections: Vec<Position>,
}

impl PipeNode {
    pub fn new(tile: PipeTile, position: Position, connections: Vec<Position>) -> PipeNode {
        PipeNode {
            tile,
            position,
            connections
        }
    }

    pub fn is_start(&self) -> bool {
        self.tile == PipeTile::StartPosition
    }
}