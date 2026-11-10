use rusqlite::Connection;
use crate::models::rincian::{
    SumberDana, TambahSumberDanaInput,
    RincianRekening, TambahRincianRekeningInput,
    RincianPengelompokan, TambahRincianPengelompokanInput,
    RincianKeterangan, TambahRincianKeteranganInput,
    RincianItem, TambahRincianItemInput,
};

// === SUMBER DANA ===

pub fn get_sumber_dana(conn: &Connection, tahun_anggaran_id: i64) -> Result<Vec<SumberDana>, String> {
    let mut stmt = conn
        .prepare("SELECT id, tahun_anggaran_id, kode, nama, created_at FROM sumber_dana WHERE tahun_anggaran_id = ?1 ORDER BY kode")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![tahun_anggaran_id], |row| {
            Ok(SumberDana {
                id: row.get(0)?,
                tahun_anggaran_id: row.get(1)?,
                kode: row.get(2)?,
                nama: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn tambah_sumber_dana(conn: &Connection, input: TambahSumberDanaInput) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO sumber_dana (tahun_anggaran_id, kode, nama) VALUES (?1, ?2, ?3)",
        rusqlite::params![input.tahun_anggaran_id, input.kode, input.nama],
    ).map_err(|e| format!("Gagal tambah sumber dana: {}", e))?;
    Ok(conn.last_insert_rowid())
}

// === RINCIAN REKENING ===

pub fn get_rincian_rekening(conn: &Connection, dokumen_sub_kegiatan_id: i64) -> Result<Vec<RincianRekening>, String> {
    let mut stmt = conn
        .prepare("
            SELECT rr.id, rr.dokumen_sub_kegiatan_id, rr.rekening_id,
                   r.kode, r.uraian, rr.urutan, rr.created_at
            FROM rincian_rekening rr
            LEFT JOIN rekening r ON r.id = rr.rekening_id
            WHERE rr.dokumen_sub_kegiatan_id = ?1
            ORDER BY rr.urutan
        ")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![dokumen_sub_kegiatan_id], |row| {
            Ok(RincianRekening {
                id: row.get(0)?,
                dokumen_sub_kegiatan_id: row.get(1)?,
                rekening_id: row.get(2)?,
                rekening_kode: row.get(3)?,
                rekening_uraian: row.get(4)?,
                urutan: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn tambah_rincian_rekening(conn: &Connection, input: TambahRincianRekeningInput) -> Result<i64, String> {
    let urutan = input.urutan.unwrap_or(0);
    conn.execute(
        "INSERT INTO rincian_rekening (dokumen_sub_kegiatan_id, rekening_id, urutan) VALUES (?1, ?2, ?3)",
        rusqlite::params![input.dokumen_sub_kegiatan_id, input.rekening_id, urutan],
    ).map_err(|e| format!("Gagal tambah rincian rekening: {}", e))?;
    Ok(conn.last_insert_rowid())
}

// === RINCIAN PENGELOMPOKAN ===

pub fn get_rincian_pengelompokan(conn: &Connection, rincian_rekening_id: i64) -> Result<Vec<RincianPengelompokan>, String> {
    let mut stmt = conn
        .prepare("
            SELECT rp.id, rp.rincian_rekening_id, rp.label, rp.jenis,
                   rp.sumber_dana_id, sd.nama, rp.urutan, rp.created_at
            FROM rincian_pengelompokan rp
            LEFT JOIN sumber_dana sd ON sd.id = rp.sumber_dana_id
            WHERE rp.rincian_rekening_id = ?1
            ORDER BY rp.urutan
        ")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![rincian_rekening_id], |row| {
            Ok(RincianPengelompokan {
                id: row.get(0)?,
                rincian_rekening_id: row.get(1)?,
                label: row.get(2)?,
                jenis: row.get(3)?,
                sumber_dana_id: row.get(4)?,
                sumber_dana_nama: row.get(5)?,
                urutan: row.get(6)?,
                created_at: row.get(7)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn tambah_rincian_pengelompokan(conn: &Connection, input: TambahRincianPengelompokanInput) -> Result<i64, String> {
    let jenis = input.jenis.unwrap_or_else(|| "pengelompokan".to_string());
    let urutan = input.urutan.unwrap_or(0);
    conn.execute(
        "INSERT INTO rincian_pengelompokan (rincian_rekening_id, label, jenis, sumber_dana_id, urutan) VALUES (?1, ?2, ?3, ?4, ?5)",
        rusqlite::params![input.rincian_rekening_id, input.label, jenis, input.sumber_dana_id, urutan],
    ).map_err(|e| format!("Gagal tambah pengelompokan: {}", e))?;
    Ok(conn.last_insert_rowid())
}

// === RINCIAN KETERANGAN ===

pub fn get_rincian_keterangan(conn: &Connection, rincian_pengelompokan_id: i64) -> Result<Vec<RincianKeterangan>, String> {
    let mut stmt = conn
        .prepare("SELECT id, rincian_pengelompokan_id, label, urutan, created_at FROM rincian_keterangan WHERE rincian_pengelompokan_id = ?1 ORDER BY urutan")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![rincian_pengelompokan_id], |row| {
            Ok(RincianKeterangan {
                id: row.get(0)?,
                rincian_pengelompokan_id: row.get(1)?,
                label: row.get(2)?,
                urutan: row.get(3)?,
                created_at: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn tambah_rincian_keterangan(conn: &Connection, input: TambahRincianKeteranganInput) -> Result<i64, String> {
    let urutan = input.urutan.unwrap_or(0);
    conn.execute(
        "INSERT INTO rincian_keterangan (rincian_pengelompokan_id, label, urutan) VALUES (?1, ?2, ?3)",
        rusqlite::params![input.rincian_pengelompokan_id, input.label, urutan],
    ).map_err(|e| format!("Gagal tambah keterangan: {}", e))?;
    Ok(conn.last_insert_rowid())
}

// === RINCIAN ITEM ===

pub fn get_rincian_item(conn: &Connection, rincian_keterangan_id: i64) -> Result<Vec<RincianItem>, String> {
    let mut stmt = conn
        .prepare("SELECT id, rincian_keterangan_id, shs_id, uraian, spesifikasi, volume, satuan, harga, ppn, realisasi, dikunci, urutan, created_at FROM rincian_item WHERE rincian_keterangan_id = ?1 ORDER BY urutan")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![rincian_keterangan_id], |row| {
            let dikunci: i32 = row.get(10)?;
            Ok(RincianItem {
                id: row.get(0)?,
                rincian_keterangan_id: row.get(1)?,
                shs_id: row.get(2)?,
                uraian: row.get(3)?,
                spesifikasi: row.get(4)?,
                volume: row.get(5)?,
                satuan: row.get(6)?,
                harga: row.get(7)?,
                ppn: row.get(8)?,
                realisasi: row.get(9)?,
                dikunci: dikunci != 0,
                urutan: row.get(11)?,
                created_at: row.get(12)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn tambah_rincian_item(conn: &Connection, input: TambahRincianItemInput) -> Result<i64, String> {
    let ppn = input.ppn.unwrap_or(0.0);
    let urutan = input.urutan.unwrap_or(0);
    conn.execute(
        "INSERT INTO rincian_item (rincian_keterangan_id, shs_id, uraian, spesifikasi, volume, satuan, harga, ppn, urutan) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            input.rincian_keterangan_id,
            input.shs_id,
            input.uraian,
            input.spesifikasi,
            input.volume,
            input.satuan,
            input.harga,
            ppn,
            urutan,
        ],
    ).map_err(|e| format!("Gagal tambah item: {}", e))?;
    Ok(conn.last_insert_rowid())
}

pub fn hapus_rincian_item(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM rincian_item WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
