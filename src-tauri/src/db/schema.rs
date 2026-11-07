use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        -- Tabel tahun anggaran
        CREATE TABLE IF NOT EXISTS tahun_anggaran (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun INTEGER NOT NULL UNIQUE,
            aktif INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        -- Tabel SSH/SHBJ
        CREATE TABLE IF NOT EXISTS ssh (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun_anggaran_id INTEGER NOT NULL,
            kode TEXT NOT NULL,
            uraian TEXT NOT NULL,
            satuan TEXT NOT NULL,
            harga REAL NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id)
        );

        -- Tabel kode rekening
        CREATE TABLE IF NOT EXISTS rekening (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kode TEXT NOT NULL UNIQUE,
            uraian TEXT NOT NULL,
            level INTEGER NOT NULL
        );
    ")?;

    Ok(())
}