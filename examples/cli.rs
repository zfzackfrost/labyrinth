use argh::FromArgs;

use rand::prelude::*;
use rand_chacha::ChaChaRng;

use labyrinth::*;

#[derive(FromArgs)]
/// Generate random mazes
struct Cli {
    /// seed value. uses entropy by default.
    #[argh(option)]
    seed: Option<u64>,

    /// width of the generated maze
    #[argh(option, default = "15")]
    width: u64,

    /// height of the generated maze
    #[argh(option, default = "15")]
    height: u64,
}

fn main() {
    let cli: Cli = argh::from_env();
    let mut rng = if let Some(seed) = cli.seed {
        ChaChaRng::seed_from_u64(seed)
    } else {
        ChaChaRng::from_entropy()
    };

    let maze = GrowingTree::default().generate_maze(cli.width, cli.height, &mut rng);
    println!("{}", maze);
}