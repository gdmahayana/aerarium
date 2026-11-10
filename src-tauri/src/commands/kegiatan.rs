use rusqlite::{Connection, Result, params};
use crate::models::kegiatan::{
    MasterProgram, TambahMasterProgramInput,
    MasterKegiatan, TambahMasterKegiatanInput,
    MasterSubKegiatan, TambahMasterSubKegiatanInput,
    DokumenSubKegiatan, TambahDokumenSubKegiatanInput,
};

// =====================
// Master Program
// =====================

pub fn get_master_program(conn: &Connection) -> Result<Vec<MasterProgram>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, kode, uraian, aktif, created_at FROM master_program ORDER BY kode"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(MasterProgram {
            id: row.get(0)?,
            kode: row.get(1)?,
            uraian: row.get(2)?,
            aktif: row.get::<_, i64>(3)? == 1,
            created_at: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn tambah_master_program(conn: &Connection, input: TambahMasterProgramInput) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO master_program (kode, uraian) VALUES (?1, ?2)",
        params![input.kode, input.uraian],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn toggle_master_program(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "UPDATE master_program SET aktif = CASE WHEN aktif = 1 THEN 0 ELSE 1 END WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn hapus_master_program(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM master_program WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// =====================
// Master Kegiatan
// =====================

pub fn get_master_kegiatan(conn: &Connection, master_program_id: i64) -> Result<Vec<MasterKegiatan>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, master_program_id, kode, uraian, aktif, created_at FROM master_kegiatan WHERE master_program_id = ?1 ORDER BY kode"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params![master_program_id], |row| {
        Ok(MasterKegiatan {
            id: row.get(0)?,
            master_program_id: row.get(1)?,
            kode: row.get(2)?,
            uraian: row.get(3)?,
            aktif: row.get::<_, i64>(4)? == 1,
            created_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn tambah_master_kegiatan(conn: &Connection, input: TambahMasterKegiatanInput) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO master_kegiatan (master_program_id, kode, uraian) VALUES (?1, ?2, ?3)",
        params![input.master_program_id, input.kode, input.uraian],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn toggle_master_kegiatan(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "UPDATE master_kegiatan SET aktif = CASE WHEN aktif = 1 THEN 0 ELSE 1 END WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn hapus_master_kegiatan(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM master_kegiatan WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// =====================
// Master Sub Kegiatan
// =====================

pub fn get_master_sub_kegiatan(conn: &Connection, master_kegiatan_id: i64) -> Result<Vec<MasterSubKegiatan>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, master_kegiatan_id, kode, uraian, aktif, created_at FROM master_sub_kegiatan WHERE master_kegiatan_id = ?1 ORDER BY kode"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params![master_kegiatan_id], |row| {
        Ok(MasterSubKegiatan {
            id: row.get(0)?,
            master_kegiatan_id: row.get(1)?,
            kode: row.get(2)?,
            uraian: row.get(3)?,
            aktif: row.get::<_, i64>(4)? == 1,
            created_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn get_master_sub_kegiatan_aktif(conn: &Connection) -> Result<Vec<MasterSubKegiatan>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, master_kegiatan_id, kode, uraian, aktif, created_at FROM master_sub_kegiatan WHERE aktif = 1 ORDER BY kode"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map([], |row| {
        Ok(MasterSubKegiatan {
            id: row.get(0)?,
            master_kegiatan_id: row.get(1)?,
            kode: row.get(2)?,
            uraian: row.get(3)?,
            aktif: row.get::<_, i64>(4)? == 1,
            created_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn tambah_master_sub_kegiatan(conn: &Connection, input: TambahMasterSubKegiatanInput) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO master_sub_kegiatan (master_kegiatan_id, kode, uraian) VALUES (?1, ?2, ?3)",
        params![input.master_kegiatan_id, input.kode, input.uraian],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn toggle_master_sub_kegiatan(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "UPDATE master_sub_kegiatan SET aktif = CASE WHEN aktif = 1 THEN 0 ELSE 1 END WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn hapus_master_sub_kegiatan(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM master_sub_kegiatan WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

// =====================
// Dokumen Sub Kegiatan
// =====================

pub fn get_dokumen_sub_kegiatan(conn: &Connection, dokumen_id: i64) -> Result<Vec<DokumenSubKegiatan>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, dokumen_id, master_sub_kegiatan_id, pagu, sumber_dana_id, lokasi, waktu_pelaksanaan, keluaran, target_keluaran, created_at
         FROM dokumen_sub_kegiatan WHERE dokumen_id = ?1 ORDER BY id"
    ).map_err(|e| e.to_string())?;
    let rows = stmt.query_map(params![dokumen_id], |row| {
        Ok(DokumenSubKegiatan {
            id: row.get(0)?,
            dokumen_id: row.get(1)?,
            master_sub_kegiatan_id: row.get(2)?,
            pagu: row.get(3)?,
            sumber_dana_id: row.get(4)?,
            lokasi: row.get(5)?,
            waktu_pelaksanaan: row.get(6)?,
            keluaran: row.get(7)?,
            target_keluaran: row.get(8)?,
            created_at: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;
    rows.collect::<Result<Vec<_>, _>>().map_err(|e| e.to_string())
}

pub fn tambah_dokumen_sub_kegiatan(conn: &Connection, input: TambahDokumenSubKegiatanInput) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO dokumen_sub_kegiatan
            (dokumen_id, master_sub_kegiatan_id, pagu, sumber_dana_id, lokasi, waktu_pelaksanaan, keluaran, target_keluaran)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            input.dokumen_id,
            input.master_sub_kegiatan_id,
            input.pagu.unwrap_or(0.0),
            input.sumber_dana_id,
            input.lokasi,
            input.waktu_pelaksanaan,
            input.keluaran,
            input.target_keluaran,
        ],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn update_dokumen_sub_kegiatan(conn: &Connection, id: i64, pagu: f64, sumber_dana_id: Option<i64>, lokasi: Option<String>, waktu_pelaksanaan: Option<String>, keluaran: Option<String>, target_keluaran: Option<String>) -> Result<(), String> {
    conn.execute(
        "UPDATE dokumen_sub_kegiatan SET pagu=?1, sumber_dana_id=?2, lokasi=?3, waktu_pelaksanaan=?4, keluaran=?5, target_keluaran=?6 WHERE id=?7",
        params![pagu, sumber_dana_id, lokasi, waktu_pelaksanaan, keluaran, target_keluaran, id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn hapus_dokumen_sub_kegiatan(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute(
        "DELETE FROM dokumen_sub_kegiatan WHERE id = ?1",
        params![id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}
