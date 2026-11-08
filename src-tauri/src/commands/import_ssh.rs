use calamine::{open_workbook, DataType, Reader, Xlsx};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShsItem {
    pub jenis: String,
    pub kode: String,
    pub uraian: String,
    pub satuan: String,
    pub harga: f64,
}

pub fn import_shs_from_excel(
    conn: &Connection,
    file_path: &str,
    tahun_anggaran_id: i64,
    jenis: &str,
) -> Result<usize, String> {
    let mut workbook: Xlsx<_> =
        open_workbook(file_path).map_err(|e| format!("Gagal buka file: {}", e))?;

    let sheet_names = workbook.sheet_names().to_vec();
    let sheet_name = sheet_names
        .first()
        .ok_or("File Excel tidak memiliki sheet")?
        .clone();

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| format!("Gagal baca sheet: {}", e))?;

    let mut count = 0;

    for row in range.rows().skip(1) {
        let kode = row.get(0).map(|c| c.to_string()).unwrap_or_default();
        let uraian = row.get(1).map(|c| c.to_string()).unwrap_or_default();
        let satuan = row.get(2).map(|c| c.to_string()).unwrap_or_default();
        let harga = row
            .get(3)
            .and_then(|c| c.get_float())
            .unwrap_or(0.0);

        if kode.is_empty() && uraian.is_empty() {
            continue;
        }

        conn.execute(
            "INSERT INTO shs (tahun_anggaran_id, jenis, kode, uraian, satuan, harga)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![tahun_anggaran_id, jenis, kode, uraian, satuan, harga],
        )
        .map_err(|e| format!("Gagal simpan data: {}", e))?;

        count += 1;
    }

    Ok(count)
}


pub fn get_shs(conn: &Connection, tahun_anggaran_id: i64) -> Result<Vec<ShsItem>, String> {
    let mut stmt = conn
        .prepare("SELECT jenis, kode, uraian, satuan, harga FROM shs WHERE tahun_anggaran_id = ?1 ORDER BY jenis, kode")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_map(rusqlite::params![tahun_anggaran_id], |row| {
            Ok(ShsItem {
                jenis: row.get(0)?,
                kode: row.get(1)?,
                uraian: row.get(2)?,
                satuan: row.get(3)?,
                harga: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(result)
}
