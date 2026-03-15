mod migrations;
mod queries;

use std::fs;

use directories::ProjectDirs;
use rusqlite::Connection;

use crate::db;
use crate::{common::Result, error::SymmanError};

pub fn init_db() -> Result<Connection> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "symman", "symman") {
        let data_dir = proj_dirs.data_dir();

        if !data_dir.exists() {
            fs::create_dir_all(data_dir)?;
        }

        let db_path = data_dir.join("symman.db");
        let conn = Connection::open(db_path)?;
        db::run_migrations(&conn)?;
        Ok(conn)
    } else {
        Err(SymmanError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Could not determine home directory",
        )))
    }
}

pub use migrations::run_migrations;
pub use queries::{find_by_name, insert_link, list_all, remove_link};
