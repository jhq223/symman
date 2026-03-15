mod cli;
mod commands;
mod db;
mod platform;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => todo!(),
        Commands::Check { name } => todo!(),
        Commands::New {
            link_path,
            target_path,
            name,
        } => todo!(),
        Commands::Remove { name, restore } => todo!(),
    }
}
