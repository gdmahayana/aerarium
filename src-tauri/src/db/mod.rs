use rusqlite::{Connection, Result};
use std::path::PathBuf;

mod schema;

pub fn get_db_path() -> PathBuf {
    let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("aerarium");
    path.push("aerarium.db");
    path
}

pub fn run_migrations(conn: &Connection) -> Result<()> {
    // Cek versi migrasi tertinggi yang sudah diterapkan
    let versi_max: i64 = conn.query_row(
        "SELECT COALESCE(MAX(versi), 0) FROM migrasi",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    // Migrasi 5: tambah pagu_indikatif_id di rincian_item
    if versi_max < 5 {
        // Tambah kolom jika belum ada
        let _ = conn.execute_batch(
            "ALTER TABLE rincian_item ADD COLUMN pagu_indikatif_id INTEGER REFERENCES pagu_indikatif(id) ON DELETE SET NULL;"
        );
        conn.execute(
            "INSERT OR IGNORE INTO migrasi (versi, deskripsi) VALUES (5, 'Tambah pagu_indikatif dan pagu_indikatif_id di rincian_item')",
            [],
        )?;
    }

    Ok(())
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

    // Jalankan migrasi inkremental
    run_migrations(&conn)?;
    
    Ok(conn)
}