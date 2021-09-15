use crate::Maze;
use rand::prelude::*;

pub trait MazeGenerator {
    fn generate_maze<RandGen: Rng + RngCore>(
        &mut self,
        width: u64,
        height: u64,
        rng: &mut RandGen,
    ) -> Maze;
}
