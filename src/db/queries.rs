use std::path::PathBuf;

use rusqlite::{Connection, params};

use crate::{
    common::{LinkEntry, LinkKind, Result},
    error::SymmanError,
};

pub fn insert_link(conn: &Connection, entry: &LinkEntry) -> Result<()> {
    conn.execute(
        "insert into links (name, link_path, target_path, kind, created_at)
        values (?1, ?2, ?3, ?4, ?5)",
        params![
            entry.name,
            entry.link_path.to_str(),
            entry.target_path.to_str(),
            String::from(entry.kind),
            entry.created_at.to_rfc3339()
        ],
    )?;
    Ok(())
}

pub fn list_all(conn: &Connection) -> Result<Vec<LinkEntry>> {
    let mut stmt = conn.prepare(
        "select name, link_path, target_path, kind, created_at 
        from links order by created_at",
    )?;

    let rows = stmt.query_map([], |row| {
        let link_path: String = row.get(1)?;
        let target_path: String = row.get(2)?;
        let kind: String = row.get(3)?;

        let link_path = PathBuf::from(link_path);
        let target_path = PathBuf::from(target_path);
        let kind = LinkKind::try_from(kind)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;

        Ok(LinkEntry {
            name: row.get(0)?,
            link_path,
            target_path,
            kind,
            created_at: row.get(4)?,
        })
    })?;

    let result = rows.collect::<rusqlite::Result<Vec<LinkEntry>>>()?;
    Ok(result)
}

pub fn find_by_name(conn: &Connection, name: &str) -> Result<Option<LinkEntry>> {
    let mut stmt = conn.prepare(
        "select name, link_path, target_path, kind, created_at from links where name = ?1",
    )?;

    let result = stmt.query_row(params![name], |row| {
        let link_path: String = row.get(1)?;
        let target_path: String = row.get(2)?;
        let kind: String = row.get(3)?;

        let link_path = PathBuf::from(link_path);
        let target_path = PathBuf::from(target_path);
        let kind = LinkKind::try_from(kind)
            .map_err(|e| rusqlite::Error::InvalidParameterName(e.to_string()))?;
        Ok(LinkEntry {
            name: row.get(0)?,
            link_path,
            target_path,
            kind,
            created_at: row.get(4)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(e.into()),
    }
}

pub fn remove_link(conn: &Connection, name: &str) -> Result<()> {
    let affected = conn.execute("delete from links where name = ?1", params![name])?;
    if affected == 0 {
        return Err(SymmanError::NotFound(name.to_string()));
    }
    Ok(())
}
