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

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub inserted: usize,
    pub updated: usize,
    pub skipped: usize,
    pub kode_barang_inserted: usize,
    pub rekening_inserted: usize,
}

pub fn import_shs_from_excel(
    conn: &Connection,
    file_path: &str,
    tahun_anggaran_id: i64,
    jenis: &str,
) -> Result<ImportResult, String> {
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

    let mut result = ImportResult {
        inserted: 0,
        updated: 0,
        skipped: 0,
        kode_barang_inserted: 0,
        rekening_inserted: 0,
    };

    for row in range.rows().skip(1) {
        // Kolom: A=kode_kelompok B=uraian_kelompok C=id_sipd D=kode E=uraian F=spesifikasi G=satuan H=harga I=kode_rekening
        let kode_kelompok = row.get(0).map(|c| c.to_string()).filter(|s| !s.is_empty());
        let uraian_kelompok = row.get(1).map(|c| c.to_string()).filter(|s| !s.is_empty());
        let id_sipd = row.get(2).map(|c| {
            if let Some(f) = c.get_float() {
                (f as i64).to_string()
            } else {
                c.to_string()
            }
        }).filter(|s| !s.is_empty());
        let kode = match row.get(3) {
            Some(c) => c.to_string(),
            None => continue,
        };
        let uraian = match row.get(4) {
            Some(c) => c.to_string(),
            None => continue,
        };
        let spesifikasi = row.get(5).map(|c| c.to_string()).filter(|s| !s.is_empty());
        let satuan = row.get(6).map(|c| c.to_string()).unwrap_or_default();
        let harga = row.get(7).and_then(|c| c.get_float()).unwrap_or(0.0);
        let kode_rekening_raw = row.get(8).map(|c| c.to_string()).filter(|s| !s.is_empty());

        if kode.is_empty() || uraian.is_empty() {
            result.skipped += 1;
            continue;
        }

        // 1. Upsert kode_barang dari kode_kelompok
        if let (Some(kk), Some(uk)) = (&kode_kelompok, &uraian_kelompok) {
            let affected = conn.execute(
                "INSERT INTO kode_barang (kode, uraian) VALUES (?1, ?2)
                 ON CONFLICT(kode) DO UPDATE SET uraian = excluded.uraian",
                rusqlite::params![kk, uk],
            ).map_err(|e| format!("Gagal simpan kode_barang: {}", e))?;
            if affected > 0 {
                result.kode_barang_inserted += 1;
            }
        }

        // Rekening utama = yang pertama dalam list
        let kode_rekening_utama = kode_rekening_raw.as_ref().map(|s| {
            s.split(',').next().unwrap_or("").trim().to_string()
        }).filter(|s| !s.is_empty());

        // 2. Upsert shs — pakai id_sipd sebagai key unik kalau ada, fallback ke kode+tahun
        let shs_id: i64 = if let Some(ref sid) = id_sipd {
            // Cek apakah sudah ada
            let existing: Option<i64> = conn.query_row(
                "SELECT id FROM shs WHERE id_sipd = ?1 AND tahun_anggaran_id = ?2",
                rusqlite::params![sid, tahun_anggaran_id],
                |row| row.get(0),
            ).ok();

            if let Some(id) = existing {
                // Update
                conn.execute(
                    "UPDATE shs SET kode=?1, kode_kelompok=?2, uraian_kelompok=?3,
                      uraian=?4, spesifikasi=?5, satuan=?6, harga=?7, kode_rekening=?8
                      WHERE id=?9",
                    rusqlite::params![kode, kode_kelompok, uraian_kelompok, uraian,
                        spesifikasi, satuan, harga, kode_rekening_utama, id],
                ).map_err(|e| format!("Gagal update shs: {}", e))?;
                result.updated += 1;
                // Hapus rekening lama sebelum insert baru
                conn.execute("DELETE FROM shs_rekening WHERE shs_id = ?1",
                    rusqlite::params![id]).ok();
                id
            } else {
                // Insert baru
                conn.execute(
                    "INSERT INTO shs (tahun_anggaran_id, jenis, sumber, kode, kode_kelompok,
                      uraian_kelompok, id_sipd, uraian, spesifikasi, satuan, harga, kode_rekening)
                      VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
                    rusqlite::params![tahun_anggaran_id, jenis, "resmi", kode, kode_kelompok,
                        uraian_kelompok, sid, uraian, spesifikasi, satuan, harga, kode_rekening_utama],
                ).map_err(|e| format!("Gagal insert shs: {}", e))?;
                result.inserted += 1;
                conn.last_insert_rowid()
            }
        } else {
            // Tidak ada id_sipd — insert saja
            conn.execute(
                "INSERT INTO shs (tahun_anggaran_id, jenis, sumber, kode, kode_kelompok,
                  uraian_kelompok, id_sipd, uraian, spesifikasi, satuan, harga, kode_rekening)
                  VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
                rusqlite::params![tahun_anggaran_id, jenis, "resmi", kode, kode_kelompok,
                    uraian_kelompok, Option::<String>::None, uraian, spesifikasi, satuan, harga, kode_rekening_utama],
            ).map_err(|e| format!("Gagal insert shs: {}", e))?;
            result.inserted += 1;
            conn.last_insert_rowid()
        };

        // 3. Insert semua kode_rekening ke shs_rekening
        if let Some(ref rekening_str) = kode_rekening_raw {
            for kr in rekening_str.split(',') {
                let kr = kr.trim();
                if kr.is_empty() { continue; }
                conn.execute(
                    "INSERT INTO shs_rekening (shs_id, kode_rekening) VALUES (?1, ?2)",
                    rusqlite::params![shs_id, kr],
                ).map_err(|e| format!("Gagal insert shs_rekening: {}", e))?;
                result.rekening_inserted += 1;
            }
        }
    }

    // 4. Populate tabel rekening dari shs_rekening yang baru diinsert
    populate_rekening_dari_shs(conn)?;

    Ok(result)
}

fn populate_rekening_dari_shs(conn: &rusqlite::Connection) -> Result<(), String> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT kode_rekening FROM shs_rekening WHERE kode_rekening IS NOT NULL AND kode_rekening != ''"
    ).map_err(|e| e.to_string())?;
    let kode_list: Vec<String> = stmt.query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    for kode in kode_list {
        let segmen: Vec<&str> = kode.split('.').collect();
        for i in 1..=segmen.len() {
            let kode_parent = segmen[..i].join(".");
            let level = i as i32;
            // Uraian level 1-4 standar Permendagri 90/2019
            let uraian = match (level, kode_parent.as_str()) {
                (1, _) => "Belanja".to_string(),
                (2, "5.1") => "Belanja Operasi".to_string(),
                (2, "5.2") => "Belanja Modal".to_string(),
                (2, "5.3") => "Belanja Tidak Terduga".to_string(),
                (3, "5.1.01") => "Belanja Pegawai".to_string(),
                (3, "5.1.02") => "Belanja Barang dan Jasa".to_string(),
                (3, "5.1.03") => "Belanja Bunga".to_string(),
                (3, "5.1.04") => "Belanja Subsidi".to_string(),
                (3, "5.1.05") => "Belanja Hibah".to_string(),
                (3, "5.1.06") => "Belanja Bantuan Sosial".to_string(),
                (3, "5.2.01") => "Belanja Modal Tanah".to_string(),
                (3, "5.2.02") => "Belanja Modal Peralatan dan Mesin".to_string(),
                (3, "5.2.03") => "Belanja Modal Gedung dan Bangunan".to_string(),
                (3, "5.2.04") => "Belanja Modal Jalan, Jaringan, dan Irigasi".to_string(),
                (3, "5.2.05") => "Belanja Modal Aset Tetap Lainnya".to_string(),
                (3, "5.3.01") => "Belanja Tidak Terduga".to_string(),
                (2, k) => format!("Kelompok {}", k),
                (3, k) => format!("Jenis {}", k),
                (4, k) => format!("Objek {}", k),
                (5, k) => format!("Rincian Objek {}", k),
                (_, k) => k.to_string(),
            };
            conn.execute(
                "INSERT OR IGNORE INTO rekening (kode, uraian, level) VALUES (?1, ?2, ?3)",
                rusqlite::params![kode_parent, uraian, level],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

pub fn tambah_shs_manual(conn: &Connection, tahun_anggaran_id: i64, jenis: &str, sumber: &str, kode: &str, uraian: &str, spesifikasi: Option<&str>, satuan: &str, harga: f64, kode_rekening: Option<&str>) -> Result<i64, String> {
    // Rekening utama = yang pertama
    let kode_rek_utama = kode_rekening.map(|s| s.split(',').next().unwrap_or("").trim().to_string()).filter(|s| !s.is_empty());
    conn.execute(
        "INSERT INTO shs (tahun_anggaran_id, jenis, sumber, kode, uraian, spesifikasi, satuan, harga, kode_rekening) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![tahun_anggaran_id, jenis, sumber, kode, uraian, spesifikasi, satuan, harga, kode_rek_utama],
    ).map_err(|e| e.to_string())?;
    let shs_id = conn.last_insert_rowid();
    // Insert semua kode rekening ke shs_rekening
    if let Some(rek_str) = kode_rekening {
        for kr in rek_str.split(',') {
            let kr = kr.trim();
            if kr.is_empty() { continue; }
            conn.execute(
                "INSERT INTO shs_rekening (shs_id, kode_rekening) VALUES (?1, ?2)",
                rusqlite::params![shs_id, kr],
            ).map_err(|e| e.to_string())?;
        }
    }
    Ok(shs_id)
}

pub fn hapus_shs(conn: &Connection, id: i64) -> Result<(), String> {
    conn.execute("DELETE FROM shs WHERE id = ?1", rusqlite::params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_shs(conn: &Connection, tahun_anggaran_id: i64) -> Result<Vec<ShsItem>, String> {
    let mut stmt = conn
        .prepare("SELECT s.id, s.jenis, s.sumber, s.kode, s.kode_kelompok, s.uraian_kelompok, s.id_sipd, s.uraian, s.spesifikasi, s.satuan, s.harga, COALESCE((SELECT GROUP_CONCAT(sr.kode_rekening, ', ') FROM shs_rekening sr WHERE sr.shs_id = s.id), s.kode_rekening) FROM shs s WHERE s.tahun_anggaran_id = ?1 ORDER BY s.jenis, s.kode")
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

pub fn get_kode_barang(conn: &Connection) -> Result<Vec<(i64, String, String)>, String> {
    let mut stmt = conn
        .prepare("SELECT id, kode, uraian FROM kode_barang ORDER BY kode")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}

pub fn search_shs(conn: &Connection, tahun_anggaran_id: i64, query: &str, sumber: &str, limit: i64) -> Result<Vec<ShsItem>, String> {
    let q = format!("%{}%", query.to_lowercase());
    let mut stmt = conn.prepare(
        "SELECT id, jenis, sumber, kode, kode_kelompok, uraian_kelompok, id_sipd, uraian, spesifikasi, satuan, harga, kode_rekening
         FROM shs
         WHERE tahun_anggaran_id = ?1
           AND sumber = ?2
           AND (LOWER(uraian) LIKE ?3 OR LOWER(kode) LIKE ?3 OR LOWER(spesifikasi) LIKE ?3)
         ORDER BY
           CASE WHEN LOWER(uraian) LIKE ?4 THEN 0 ELSE 1 END,
           uraian
         LIMIT ?5"
    ).map_err(|e| e.to_string())?;

    let starts_with = format!("{}%", query.to_lowercase());
    let result = stmt.query_map(
        rusqlite::params![tahun_anggaran_id, sumber, q, starts_with, limit],
        |row| Ok(ShsItem {
            id: row.get(0)?, jenis: row.get(1)?, sumber: row.get(2)?,
            kode: row.get(3)?, kode_kelompok: row.get(4)?, uraian_kelompok: row.get(5)?,
            id_sipd: row.get(6)?, uraian: row.get(7)?, spesifikasi: row.get(8)?,
            satuan: row.get(9)?, harga: row.get(10)?, kode_rekening: row.get(11)?,
        })
    ).map_err(|e| e.to_string())?
    .filter_map(|r| r.ok())
    .collect();
    Ok(result)
}

pub fn get_shs_rekening(conn: &Connection, shs_id: i64) -> Result<Vec<String>, String> {
    let mut stmt = conn
        .prepare("SELECT kode_rekening FROM shs_rekening WHERE shs_id = ?1 ORDER BY id")
        .map_err(|e| e.to_string())?;
    let result = stmt
        .query_map(rusqlite::params![shs_id], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();
    Ok(result)
}
