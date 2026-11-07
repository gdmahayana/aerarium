use calamine::{open_workbook, DataType, Reader, Xlsx};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SshItem {
    pub kode: String,
    pub uraian: String,
    pub satuan: String,
    pub harga: f64,
}

pub fn import_ssh_from_excel(
    conn: &Connection,
    file_path: &str,
    tahun_anggaran_id: i64,
) -> Result<usize, String> {
    let mut workbook: Xlsx<_> =
        open_workbook(file_path).map_err(|e| format!("Gagal buka file: {}", e))?;

    // Ambil sheet pertama
    let sheet_names = workbook.sheet_names().to_vec();
    let sheet_name = sheet_names
        .first()
        .ok_or("File Excel tidak memiliki sheet")?
        .clone();

    let range = workbook
        .worksheet_range(&sheet_name)
        .map_err(|e| format!("Gagal baca sheet: {}", e))?;

    let mut count = 0;

    // Skip baris header (baris pertama)
    for row in range.rows().skip(1) {
        let kode = row.get(0).map(|c| c.to_string()).unwrap_or_default();
        let uraian = row.get(1).map(|c| c.to_string()).unwrap_or_default();
        let satuan = row.get(2).map(|c| c.to_string()).unwrap_or_default();
        let harga = row
            .get(3)
            .and_then(|c| c.get_float())
            .unwrap_or(0.0);

        // Skip baris kosong
        if kode.is_empty() && uraian.is_empty() {
            continue;
        }

        conn.execute(
            "INSERT INTO ssh (tahun_anggaran_id, kode, uraian, satuan, harga)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![tahun_anggaran_id, kode, uraian, satuan, harga],
        )
        .map_err(|e| format!("Gagal simpan data: {}", e))?;

        count += 1;
    }

    Ok(count)
}