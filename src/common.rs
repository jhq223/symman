use std::{fmt::Display, path::PathBuf};

use chrono::{DateTime, Local};
use colored::Colorize;

use crate::error::SymmanError;

#[derive(Debug, Clone, Copy)]
pub enum LinkKind {
    File,
    Dir,
}

impl From<LinkKind> for String {
    fn from(value: LinkKind) -> Self {
        match value {
            LinkKind::File => "file".to_string(),
            LinkKind::Dir => "dir".to_string(),
        }
    }
}

impl TryFrom<String> for LinkKind {
    type Error = SymmanError;

    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "file" => Ok(LinkKind::File),
            "dir" => Ok(LinkKind::Dir),
            other => Err(SymmanError::InvalidLinkKind(other.to_string())),
        }
    }
}

#[derive(Debug)]
pub struct LinkEntry {
    pub name: String,
    pub link_path: PathBuf,
    pub target_path: PathBuf,
    pub kind: LinkKind,
    pub created_at: DateTime<Local>,
}

impl LinkEntry {
    pub fn new(name: String, link_path: PathBuf, target_path: PathBuf, kind: LinkKind) -> Self {
        Self {
            name,
            link_path,
            target_path,
            kind,
            created_at: Local::now(),
        }
    }
}

#[derive(Debug)]
pub enum LinkStatus {
    Healthy,
    BrokenLink,
    LinkMissing,
    NotASymlink,
}

impl Display for LinkStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            LinkStatus::Healthy => "✓ OK".green().to_string(),
            LinkStatus::BrokenLink => "✗ Broken".red().to_string(),
            LinkStatus::LinkMissing => "? Missing".yellow().to_string(),
            LinkStatus::NotASymlink => "! Not symlink".red().to_string(),
        };
        write!(f, "{}", status)
    }
}

pub type Result<T> = std::result::Result<T, SymmanError>;
