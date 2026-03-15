use rusqlite::Connection;
use tabled::Table;
use tabled::Tabled;

use crate::common::Result;
use crate::db;
use crate::platform;

#[derive(Tabled)]
pub struct LinkTable {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Link Path")]
    link_path: String,
    #[tabled(rename = "Target Path")]
    target_path: String,
    #[tabled(rename = "Status")]
    status: String,
    #[tabled(rename = "Created At")]
    created_at: String,
}

pub fn check(conn: &Connection, name: Option<String>) -> Result<()> {
    if let Some(name) = name {
        let entry = db::find_by_name(conn, &name)?;
        if let Some(entry) = entry {
            let status = platform::check(&entry).to_string();
            println!(
                "{}",
                Table::new([LinkTable {
                    name: entry.name,
                    link_path: entry.link_path.to_string_lossy().to_string(),
                    target_path: entry.target_path.to_string_lossy().to_string(),
                    status,
                    created_at: entry.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                }])
            );
        } else {
            return Err(crate::error::SymmanError::NotFound(name));
        }
    } else {
        let rows = db::list_all(conn)?;
        let rows = rows
            .iter()
            .map(|entry| {
                let status = platform::check(entry).to_string();
                LinkTable {
                    name: entry.name.clone(),
                    link_path: entry.link_path.to_string_lossy().to_string(),
                    target_path: entry.target_path.to_string_lossy().to_string(),
                    status,
                    created_at: entry.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                }
            })
            .collect::<Vec<LinkTable>>();

        println!("{}", Table::new(rows));
    }

    Ok(())
}
