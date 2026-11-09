use rusqlite::Connection;
use crate::models::kegiatan::{Kegiatan, TambahKegiatanInput, SubKegiatan, TambahSubKegiatanInput};

// === KEGIATAN ===

pub fn get_kegiatan(conn: &Connection, dokumen_id: i64) -> Result<Vec<Kegiatan>, String> {
    let mut stmt = conn
        .prepare("SELECT id, dokumen_id, kode, uraian, pagu, created_at FROM kegiatan WHERE dokumen_id = ?1 ORDER BY kode")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![dokumen_id], |row| {
            Ok(Kegiatan {
                id: row.get(0)?,
                dokumen_id: row.get(1)?,
                kode: row.get(2)?,
                uraian: row.get(3)?,
                pagu: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn tambah_kegiatan(conn: &Connection, input: TambahKegiatanInput) -> Result<i64, String> {
    let pagu = input.pagu.unwrap_or(0.0);
    conn.execute(
        "INSERT INTO kegiatan (dokumen_id, kode, uraian, pagu) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![input.dokumen_id, input.kode, input.uraian, pagu],
    ).map_err(|e| format!("Gagal tambah kegiatan: {}", e))?;
    Ok(conn.last_insert_rowid())
}

pub fn hapus_kegiatan(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM kegiatan WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

// === SUB KEGIATAN ===

pub fn get_sub_kegiatan(conn: &Connection, kegiatan_id: i64) -> Result<Vec<SubKegiatan>, String> {
    let mut stmt = conn
        .prepare("
            SELECT id, kegiatan_id, kode, uraian, pagu,
                   sumber_dana_id, lokasi, waktu_pelaksanaan,
                   keluaran, target_keluaran, created_at
            FROM sub_kegiatan
            WHERE kegiatan_id = ?1
            ORDER BY kode
        ")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![kegiatan_id], |row| {
            Ok(SubKegiatan {
                id: row.get(0)?,
                kegiatan_id: row.get(1)?,
                kode: row.get(2)?,
                uraian: row.get(3)?,
                pagu: row.get(4)?,
                sumber_dana_id: row.get(5)?,
                lokasi: row.get(6)?,
                waktu_pelaksanaan: row.get(7)?,
                keluaran: row.get(8)?,
                target_keluaran: row.get(9)?,
                created_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn tambah_sub_kegiatan(conn: &Connection, input: TambahSubKegiatanInput) -> Result<i64, String> {
    let pagu = input.pagu.unwrap_or(0.0);
    conn.execute(
        "INSERT INTO sub_kegiatan (kegiatan_id, kode, uraian, pagu, sumber_dana_id, lokasi, waktu_pelaksanaan, keluaran, target_keluaran)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            input.kegiatan_id,
            input.kode,
            input.uraian,
            pagu,
            input.sumber_dana_id,
            input.lokasi,
            input.waktu_pelaksanaan,
            input.keluaran,
            input.target_keluaran,
        ],
    ).map_err(|e| format!("Gagal tambah sub kegiatan: {}", e))?;
    Ok(conn.last_insert_rowid())
}

pub fn hapus_sub_kegiatan(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM sub_kegiatan WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
