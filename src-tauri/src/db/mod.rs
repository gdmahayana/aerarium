use rusqlite::{Connection, Result};
use std::path::PathBuf;

mod schema;

pub fn get_db_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("aerarium");
    path.push("aerarium.db");
    path
}

pub fn init_db() -> Result<Connection> {
    let db_path = get_db_path();
    
    // Buat folder jika belum ada
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    
    let conn = Connection::open(&db_path)?;
    
    // Aktifkan foreign keys
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    
    // Buat tabel
    schema::create_tables(&conn)?;
    
    Ok(conn)
}