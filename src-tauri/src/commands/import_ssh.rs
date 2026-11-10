use calamine::{open_workbook, DataType, Reader, Xlsx};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShsItem {
    pub id: i64,
    pub jenis: String,
    pub sumber: String,
    pub kode: String,
    pub kode_kelompok: Option<String>,
    pub uraian_kelompok: Option<String>,
    pub id_sipd: Option<String>,
    pub uraian: String,
    pub spesifikasi: Option<String>,
    pub satuan: String,
    pub harga: f64,
    pub kode_rekening: Option<String>,
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
        // Struktur SIPD:
        // A=kode_kelompok, B=uraian_kelompok, C=id_sipd, D=kode, E=uraian, F=spesifikasi, G=satuan, H=harga, I=kode_rekening
        let kode_kelompok = row.get(0).map(|c| c.to_string()).filter(|s| !s.is_empty());
        let uraian_kelompok = row.get(1).map(|c| c.to_string()).filter(|s| !s.is_empty());
        let id_sipd = row.get(2).map(|c| c.to_string()).filter(|s| !s.is_empty());
        let kode = row.get(3).map(|c| c.to_string()).unwrap_or_default();
        let uraian = row.get(4).map(|c| c.to_string()).unwrap_or_default();
        let spesifikasi = row.get(5).map(|c| c.to_string()).filter(|s| !s.is_empty());
        let satuan = row.get(6).map(|c| c.to_string()).unwrap_or_default();
        let harga = row.get(7).and_then(|c| c.get_float()).unwrap_or(0.0);
        let kode_rekening = row.get(8).map(|c| c.to_string()).filter(|s| !s.is_empty());

        if kode.is_empty() && uraian.is_empty() {
            continue;
        }

        conn.execute(
            "INSERT INTO shs (tahun_anggaran_id, jenis, sumber, kode, kode_kelompok, uraian_kelompok, id_sipd, uraian, spesifikasi, satuan, harga, kode_rekening)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
            rusqlite::params![tahun_anggaran_id, jenis, "resmi", kode, kode_kelompok, uraian_kelompok, id_sipd, uraian, spesifikasi, satuan, harga, kode_rekening],
        )
        .map_err(|e| format!("Gagal simpan data: {}", e))?;

        count += 1;
    }

    Ok(count)
}


pub fn tambah_shs_manual(conn: &Connection, tahun_anggaran_id: i64, jenis: &str, sumber: &str, kode: &str, uraian: &str, spesifikasi: Option<&str>, satuan: &str, harga: f64) -> Result<i64, String> {
    conn.execute(
        "INSERT INTO shs (tahun_anggaran_id, jenis, sumber, kode, uraian, spesifikasi, satuan, harga) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![tahun_anggaran_id, jenis, sumber, kode, uraian, spesifikasi, satuan, harga],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid())
}

pub fn hapus_shs(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM shs WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_shs(conn: &Connection, tahun_anggaran_id: i64) -> Result<Vec<ShsItem>, String> {
    let mut stmt = conn
        .prepare("SELECT id, jenis, sumber, kode, kode_kelompok, uraian_kelompok, id_sipd, uraian, spesifikasi, satuan, harga, kode_rekening FROM shs WHERE tahun_anggaran_id = ?1 ORDER BY jenis, kode")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_map(rusqlite::params![tahun_anggaran_id], |row| {
            Ok(ShsItem {
                id: row.get(0)?,
                jenis: row.get(1)?,
                sumber: row.get(2)?,
                kode: row.get(3)?,
                kode_kelompok: row.get(4)?,
                uraian_kelompok: row.get(5)?,
                id_sipd: row.get(6)?,
                uraian: row.get(7)?,
                spesifikasi: row.get(8)?,
                satuan: row.get(9)?,
                harga: row.get(10)?,
                kode_rekening: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(result)
}
