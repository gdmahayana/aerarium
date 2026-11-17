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

    // Migrasi 6: tambah Perubahan DPA-SKPD dan Perubahan DPPA-SKPD ke CHECK constraint
    if versi_max < 6 {
        conn.execute_batch(r#"
            PRAGMA foreign_keys = OFF;

            CREATE TABLE IF NOT EXISTS dokumen_new (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                tahun_anggaran_id INTEGER NOT NULL,
                jenis TEXT NOT NULL CHECK(jenis IN (
                    'Kertas Kerja','Pagu Indikatif','RKA-SKPD','DPA-SKPD','RKA-SKPD Perubahan','DPPA-SKPD',
                    'Perubahan DPA-SKPD','Perubahan DPPA-SKPD'
                )),
                versi INTEGER NOT NULL DEFAULT 1,
                nomor_se TEXT,
                tanggal_se TEXT,
                status TEXT NOT NULL CHECK(status IN (
                    'draft','konsep','ditetapkan','dikunci','diarsipkan'
                )) DEFAULT 'draft',
                asal_id INTEGER,
                asal_keterangan TEXT,
                catatan TEXT,
                ditetapkan_at TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id),
                FOREIGN KEY (asal_id) REFERENCES dokumen(id)
            );

            INSERT INTO dokumen_new SELECT * FROM dokumen;
            DROP TABLE dokumen;
            ALTER TABLE dokumen_new RENAME TO dokumen;

            PRAGMA foreign_keys = ON;
        "#)?;
        conn.execute(
            "INSERT OR IGNORE INTO migrasi (versi, deskripsi) VALUES (6, 'Tambah Perubahan DPA-SKPD dan Perubahan DPPA-SKPD ke jenis dokumen')",
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