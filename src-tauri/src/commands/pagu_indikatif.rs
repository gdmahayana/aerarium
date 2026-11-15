
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaguIndikatif {
    pub id: i64,
    pub dokumen_id: i64,
    pub kode_rekening: Option<String>,
    pub uraian: String,
    pub pagu: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaguIndikatifRekap {
    pub id: i64,
    pub dokumen_id: i64,
    pub kode_rekening: Option<String>,
    pub uraian: String,
    pub pagu: f64,
    pub total_rincian: f64,
    pub selisih: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahPaguIndikatifInput {
    pub dokumen_id: i64,
    pub kode_rekening: Option<String>,
    pub uraian: String,
    pub pagu: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePaguIndikatifInput {
    pub kode_rekening: Option<String>,
    pub uraian: String,
    pub pagu: f64,
}

pub fn tambah_pagu_indikatif(conn: &Connection, input: TambahPaguIndikatifInput) -> Result<PaguIndikatif, String> {
    conn.execute(
        "INSERT INTO pagu_indikatif (dokumen_id, kode_rekening, uraian, pagu) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![input.dokumen_id, input.kode_rekening, input.uraian, input.pagu],
    ).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    get_pagu_indikatif_by_id(conn, id)
}

pub fn get_pagu_indikatif_by_id(conn: &Connection, id: i64) -> Result<PaguIndikatif, String> {
    conn.query_row(
        "SELECT id, dokumen_id, kode_rekening, uraian, pagu, created_at FROM pagu_indikatif WHERE id = ?1",
        rusqlite::params![id],
        |row| Ok(PaguIndikatif {
            id: row.get(0)?,
            dokumen_id: row.get(1)?,
            kode_rekening: row.get(2)?,
            uraian: row.get(3)?,
            pagu: row.get(4)?,
            created_at: row.get(5)?,
        }),
    ).map_err(|e| e.to_string())
}

pub fn get_pagu_indikatif(conn: &Connection, dokumen_id: i64) -> Result<Vec<PaguIndikatif>, String> {
    let mut stmt = conn.prepare(
        "SELECT id, dokumen_id, kode_rekening, uraian, pagu, created_at FROM pagu_indikatif WHERE dokumen_id = ?1 ORDER BY id ASC",
    ).map_err(|e| e.to_string())?;
    let result = stmt.query_map(rusqlite::params![dokumen_id], |row| {
        Ok(PaguIndikatif {
            id: row.get(0)?,
            dokumen_id: row.get(1)?,
            kode_rekening: row.get(2)?,
            uraian: row.get(3)?,
            pagu: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(result)
}

pub fn get_pagu_indikatif_rekap(conn: &Connection, dokumen_id: i64) -> Result<Vec<PaguIndikatifRekap>, String> {
    let mut stmt = conn.prepare("
        SELECT
            p.id, p.dokumen_id, p.kode_rekening, p.uraian, p.pagu,
            COALESCE(SUM(
                ri.volume
                * COALESCE(ri.koef2, 1.0)
                * COALESCE(ri.koef3, 1.0)
                * COALESCE(ri.koef4, 1.0)
                * ri.harga
                * (1.0 + CASE WHEN ri.ppn > 0 THEN ri.ppn / 100.0 ELSE 0.0 END)
            ), 0.0) AS total_rincian
        FROM pagu_indikatif p
        LEFT JOIN rincian_item ri ON ri.pagu_indikatif_id = p.id
        WHERE p.dokumen_id = ?1
        GROUP BY p.id
        ORDER BY p.id ASC
    ").map_err(|e| e.to_string())?;

    let result = stmt.query_map(rusqlite::params![dokumen_id], |row| {
        let pagu: f64 = row.get(4)?;
        let total_rincian: f64 = row.get(5)?;
        Ok(PaguIndikatifRekap {
            id: row.get(0)?,
            dokumen_id: row.get(1)?,
            kode_rekening: row.get(2)?,
            uraian: row.get(3)?,
            pagu,
            total_rincian,
            selisih: pagu - total_rincian,
        })
    }).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(result)
}

pub fn update_pagu_indikatif(conn: &Connection, id: i64, input: UpdatePaguIndikatifInput) -> Result<PaguIndikatif, String> {
    conn.execute(
        "UPDATE pagu_indikatif SET kode_rekening = ?1, uraian = ?2, pagu = ?3 WHERE id = ?4",
        rusqlite::params![input.kode_rekening, input.uraian, input.pagu, id],
    ).map_err(|e| e.to_string())?;
    get_pagu_indikatif_by_id(conn, id)
}

pub fn hapus_pagu_indikatif(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM pagu_indikatif WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}
