use crate::base::MazeGenerator;
use crate::direction::*;
use crate::maze::Maze;

use rand::distributions::Uniform;
use rand::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum IndexMode {
    Random,
    Newest,
    Middle,
    Oldest,
}

impl IndexMode {
    fn evaluate<R: Rng + RngCore>(self, ceil: usize, rng: &mut R) -> usize {
        match self {
            IndexMode::Random => rng.gen_range(0..ceil),
            IndexMode::Newest => ceil - 1,
            IndexMode::Middle => ceil / 2,
            IndexMode::Oldest => 0,
        }
    }
    pub fn from_lower_str(s: &str) -> Option<Self> {
        match s {
            "random" => Some(Self::Random),
            "newest" => Some(Self::Newest),
            "oldest" => Some(Self::Oldest),
            "middle" => Some(Self::Middle),
            _ => None
        }
    }
}

#[derive(Clone)]
pub struct IndexCommand {
    weights: Vec<(f64, IndexMode)>,
    distribution: Uniform<f64>,
}

impl IndexCommand {
    pub fn new(weights: Vec<(f64, IndexMode)>) -> Self {
        let mut total_weight = 0.0;
        for (weight, _mode) in &weights {
            total_weight += *weight;
        }
        Self {
            weights,
            distribution: Uniform::from(0.0..total_weight),
        }
    }

    fn evaluate<R: Rng + RngCore>(&self, ceil: usize, rng: &mut R) -> usize {
        let mut v = self.distribution.sample(rng);
        for (weight, mode) in &self.weights {
            if v < *weight {
                return mode.evaluate(ceil, rng);
            } else {
                v -= *weight;
            }
        }
        panic!("Failed to evaluate index!");
    }
}

pub struct GrowingTree {
    index_commands: Vec<IndexCommand>,
    current_index_cmd: usize,
}

impl Default for GrowingTree {
    fn default() -> Self {
        Self {
            index_commands: {
                use IndexMode::*;
                vec![
                    IndexCommand::new(vec![(40.0, Newest), (12.0, Random), (2.0, Oldest)]),
                    IndexCommand::new(vec![(10.0, Random), (20.0, Middle), (45.0, Newest)]),
                ]
            },
            current_index_cmd: 0,
        }
    }
}

impl GrowingTree {
    pub fn new(index_commands: Vec<IndexCommand>) -> Self {
        debug_assert!(!index_commands.is_empty(), "No index commands provided!");
        Self {
            index_commands,
            ..Default::default()
        }
    }

    fn next_index<R: Rng + RngCore>(&mut self, ceil: usize, rng: &mut R) -> usize {
        let command = &self.index_commands[self.current_index_cmd];
        self.current_index_cmd = (self.current_index_cmd + 1) % self.index_commands.len();
        command.evaluate(ceil, rng)
    }
}

impl MazeGenerator for GrowingTree {
    fn generate_maze<RandGen: Rng + RngCore>(
        &mut self,
        width: u64,
        height: u64,
        rng: &mut RandGen,
    ) -> Maze {
        let mut maze = Maze::new(width, height);
        let mut cells = Vec::new();

        {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);
            cells.push((x, y));
        }

        while !cells.is_empty() {
            let index = self.next_index(cells.len(), rng);
            let (x, y) = cells[index];
            let mut index = Some(index);
            let dirs = {
                let mut v = vec![N, S, E, W];
                v.shuffle(rng);
                v
            };
            for dir in dirs {
                let nx = x as i64 + get_dx(dir);
                let ny = y as i64 + get_dy(dir);
                if nx >= 0
                    && ny >= 0
                    && nx < width as i64
                    && ny < height as i64
                    && maze.grid[ny as usize][nx as usize] == 0
                {
                    maze.grid[y as usize][x as usize] |= dir;
                    maze.grid[ny as usize][nx as usize] |= get_opposite(dir);
                    cells.push((nx as u64, ny as u64));
                    index = None;
                    break;
                }
            }
            if let Some(index) = index {
                cells.remove(index);
            }
        }

        maze
    }
}
