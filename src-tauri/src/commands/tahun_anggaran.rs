use rusqlite::Connection;
use crate::models::tahun_anggaran::{TahunAnggaran, TambahTahunAnggaranInput};

pub fn get_all(conn: &Connection) -> Result<Vec<TahunAnggaran>, String> {
    let mut stmt = conn
        .prepare("SELECT id, tahun, aktif, created_at FROM tahun_anggaran ORDER BY tahun DESC")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_map([], |row| {
            Ok(TahunAnggaran {
                id: row.get(0)?,
                tahun: row.get(1)?,
                aktif: row.get::<_, i32>(2)? == 1,
                created_at: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(result)
}

pub fn tambah(conn: &Connection, input: TambahTahunAnggaranInput) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO tahun_anggaran (tahun) VALUES (?1)",
        rusqlite::params![input.tahun],
    )
    .map_err(|e| format!("Gagal tambah tahun anggaran: {}", e))?;

    Ok(conn.last_insert_rowid())
}

pub fn set_aktif(conn: &Connection, id: i64) -> Result<(), String> {
    // Reset semua ke tidak aktif
    conn.execute("UPDATE tahun_anggaran SET aktif = 0", [])
        .map_err(|e| e.to_string())?;

    // Set yang dipilih jadi aktif
    conn.execute(
        "UPDATE tahun_anggaran SET aktif = 1 WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

pub fn hapus(conn: &Connection, id: i64) -> Result<(), String> {
    // Cek apakah ada dokumen yang terkait
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM dokumen WHERE tahun_anggaran_id = ?1",
            rusqlite::params![id],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    if count > 0 {
        return Err("Tahun anggaran tidak bisa dihapus karena sudah memiliki dokumen".to_string());
    }

    conn.execute(
        "DELETE FROM tahun_anggaran WHERE id = ?1",
        rusqlite::params![id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
