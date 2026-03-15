use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "symman", version, about = "Symlink manager")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all managed symlinks
    List,
    /// Check symlink health
    Check {
        /// Check specific link by name, omit to check all
        name: Option<String>,
    },
    /// Create a new managed symlink
    New {
        /// Where the symlink will be created (e.g. C:\Users\foo\AppData)
        link_path: PathBuf,
        /// Where it points to (e.g. D:\Symlinks\AppData)
        target_path: PathBuf,
        /// Human-readable alias
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Remove a managed symlink
    Remove {
        /// Alias name of the link to remove
        name: String,
        /// Copy target contents back before removing
        #[arg(short, long)]
        restore: bool,
    },
}
