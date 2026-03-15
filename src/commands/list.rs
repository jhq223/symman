use rusqlite::Connection;
use tabled::{Table, Tabled};

use crate::common::Result;
use crate::db;

#[derive(Tabled)]
pub struct LinkTable {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Link Path")]
    link_path: String,
    #[tabled(rename = "Target Path")]
    target_path: String,
    #[tabled(rename = "Created At")]
    created_at: String,
}

pub fn list(conn: &Connection) -> Result<()> {
    let result = db::list_all(conn)?;
    let rows = result
        .iter()
        .map(|entry| LinkTable {
            name: entry.name.clone(),
            link_path: entry.link_path.to_string_lossy().to_string(),
            target_path: entry.target_path.to_string_lossy().to_string(),
            created_at: entry.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
        })
        .collect::<Vec<LinkTable>>();
    println!("{}", Table::new(rows));
    Ok(())
}
