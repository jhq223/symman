use std::path::PathBuf;

use rusqlite::Connection;

use crate::{
    common::{LinkEntry, LinkKind, Result},
    db,
    error::SymmanError,
    platform,
};

pub fn new(
    conn: &Connection,
    link_path: PathBuf,
    target_path: PathBuf,
    name: Option<String>,
) -> Result<()> {
    if !target_path.exists() {
        return Err(SymmanError::TargetNotFound(target_path));
    }

    let kind = if target_path.is_dir() {
        LinkKind::Dir
    } else {
        LinkKind::File
    };

    let name = match name {
        Some(name) => name,
        None => link_path
            .file_name()
            .ok_or_else(|| {
                SymmanError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid link path",
                ))
            })?
            .to_string_lossy()
            .to_string(),
    };

    if db::find_by_name(conn, &name)?.is_some() {
        return Err(SymmanError::DuplicateName(name));
    }

    let entry = LinkEntry::new(name.clone(), link_path, target_path, kind);

    platform::create(&entry)?;

    if let Err(e) = db::insert_link(conn, &entry) {
        let _ = platform::remove(&entry);
        return Err(e);
    }

    println!("Successfully created symlink '{}'", name);
    Ok(())
}
