mod cli;
mod commands;
mod common;
mod db;
mod error;
mod platform;

use clap::Parser;
use colored::Colorize as _;

use crate::cli::{Cli, Commands};

fn main() {
    let cli = Cli::parse();
    let conn = db::init_db().unwrap_or_else(|e| {
        eprintln!("Failed to initialize database: {}", e);
        std::process::exit(1);
    });

    let result = match cli.command {
        Commands::List => commands::list(&conn),
        Commands::Check { name } => commands::check(&conn, name),
        Commands::New {
            link_path,
            target_path,
            name,
        } => commands::new(&conn, link_path, target_path, name),
        Commands::Remove { name, restore } => commands::remove(&conn, &name, restore),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".red().bold(), e);
        std::process::exit(1);
    }
}
