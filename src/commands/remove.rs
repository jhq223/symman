use std::fs;
use std::path::Path;

use rusqlite::Connection;

use crate::{common::Result, db, error::SymmanError, platform};

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }

    Ok(())
}

pub fn remove(conn: &Connection, name: &str, restore: bool) -> Result<()> {
    let entry =
        db::find_by_name(conn, name)?.ok_or_else(|| SymmanError::NotFound(name.to_string()))?;

    if restore && !entry.target_path.exists() {
        return Err(SymmanError::TargetNotFound(entry.target_path.clone()));
    }

    match platform::remove(&entry) {
        Ok(_) => {}
        Err(SymmanError::Io(e)) if e.kind() == std::io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }

    if restore {
        if entry.target_path.is_dir() {
            copy_dir_all(&entry.target_path, &entry.link_path)?;
        } else {
            fs::copy(&entry.target_path, &entry.link_path)?;
        }
        println!("Restored real file to {:?}", entry.link_path);
    }

    db::remove_link(conn, name)?;

    println!("Removed symlink entry '{}'", name);
    Ok(())
}
