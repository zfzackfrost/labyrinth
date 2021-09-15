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

    #[argh(subcommand)]
    cmd: CliSubcommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum CliSubcommands {
    CmdGrowingTree(CliGrowingTreeCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "growingtree")]
/// Growing Tree algorithm
struct CliGrowingTreeCommand {
    #[argh(option)]
    /// index commands list for the growing tree algorithm. Each index command
    /// must be in the format: "<weight_1>:<command_1>,...,<weight_n>:<command_n>"
    /// `command` must be one of: "random", "newest", "oldest", "middle" and weight
    /// is floating-point number. The command list is semicolon separated. Example:
    /// "100.0:random;50.0:random,50.0:middle,70.0:newest"
    commands: String,
}

fn main() {
    let cli: Cli = argh::from_env();
    let mut rng = if let Some(seed) = cli.seed {
        ChaChaRng::seed_from_u64(seed)
    } else {
        ChaChaRng::from_entropy()
    };

    let maze = match cli.cmd {
        CliSubcommands::CmdGrowingTree(cmd) => {
            let index_commands = {
                cmd.commands
                    .split(';')
                    .map(|command_str| {
                        let weights: Vec<_> = command_str
                            .split(',')
                            .map(|pair_str| {
                                let (weight_str, mode_str) = pair_str.split_once(':').unwrap();
                                let mode = IndexMode::from_lower_str(mode_str).unwrap();
                                let weight = weight_str.parse::<f64>().unwrap();
                                (weight, mode)
                            })
                            .collect();
                        IndexCommand::new(weights)
                    })
                    .collect()
            };
            GrowingTree::new(index_commands).generate_maze(cli.width, cli.height, &mut rng)
        }
    };
    println!("{}", maze);
}
