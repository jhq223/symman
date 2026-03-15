use rusqlite::{Connection, Result};

const INIT_SQL: &str = "
    CREATE TABLE IF NOT EXISTS links (
        id          INTEGER PRIMARY KEY AUTOINCREMENT,
        name        TEXT NOT NULL UNIQUE,
        link_path   TEXT NOT NULL,
        target_path TEXT NOT NULL,
        kind        TEXT NOT NULL,   -- 'file' | 'dir'
        created_at  TEXT NOT NULL
    );
    CREATE INDEX IF NOT EXISTS idx_name ON links(name);";

pub fn run_migrations(conn: &Connection) -> Result<()> {
    conn.execute_batch(INIT_SQL)
}
