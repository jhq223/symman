use std::path::PathBuf;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SymmanError {
    #[error("Link '{0}' not found in registry")]
    NotFound(String),

    #[error("Link name '{0}' already exists")]
    DuplicateName(String),

    #[error("Target path does not exist: {0}")]
    TargetNotFound(PathBuf),

    #[error("Invalid link kind in database: '{0}'")]
    InvalidLinkKind(String),

    #[error(
        "Insufficient privilege to create symlinks.Run as Administrator or enable Developer Mode."
    )]
    InsufficientPrivilege,

    #[error("Path exists but is not a symlink: {0}")]
    NotASymlink(PathBuf),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Db(#[from] rusqlite::Error),
}
