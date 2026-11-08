use rusqlite::Connection;
use crate::models::dokumen::{Dokumen, BuatDokumenInput, JenisDokumen, StatusDokumen};

pub fn get_all(conn: &Connection, tahun_anggaran_id: i64) -> Result<Vec<Dokumen>, String> {
    let mut stmt = conn
        .prepare("
            SELECT id, tahun_anggaran_id, jenis, versi, nomor_se, tanggal_se,
                   status, asal_id, asal_keterangan, catatan, ditetapkan_at, created_at
            FROM dokumen
            WHERE tahun_anggaran_id = ?1
            ORDER BY jenis, versi DESC
        ")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_map(rusqlite::params![tahun_anggaran_id], |row| {
            let jenis_str: String = row.get(2)?;
            let status_str: String = row.get(6)?;
            Ok(Dokumen {
                id: row.get(0)?,
                tahun_anggaran_id: row.get(1)?,
                jenis: JenisDokumen::from_str(&jenis_str).unwrap_or(JenisDokumen::RkaSkpd),
                versi: row.get(3)?,
                nomor_se: row.get(4)?,
                tanggal_se: row.get(5)?,
                status: StatusDokumen::from_str(&status_str).unwrap_or(StatusDokumen::Draft),
                asal_id: row.get(7)?,
                asal_keterangan: row.get(8)?,
                catatan: row.get(9)?,
                ditetapkan_at: row.get(10)?,
                created_at: row.get(11)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    Ok(result)
}

pub fn buat(conn: &Connection, input: BuatDokumenInput) -> Result<i64, String> {
    let versi_berikutnya: i64 = conn
        .query_row(
            "SELECT COALESCE(MAX(versi), 0) + 1 FROM dokumen
             WHERE tahun_anggaran_id = ?1 AND jenis = ?2",
            rusqlite::params![input.tahun_anggaran_id, input.jenis.as_str()],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;

    conn.execute(
        "UPDATE dokumen SET status = 'dikunci'
         WHERE tahun_anggaran_id = ?1 AND jenis = ?2 AND status != 'dikunci'",
rusqlite::params![input.tahun_anggaran_id, input.jenis.as_str()],
)
.map_err(|e| e.to_string())?;

conn.execute(
"INSERT INTO dokumen (tahun_anggaran_id, jenis, versi, nomor_se, tanggal_se, asal_keterangan, catatan)
VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
rusqlite::params![
input.tahun_anggaran_id,
input.jenis.as_str(),
versi_berikutnya,
input.nomor_se,
input.tanggal_se,
input.asal_keterangan,
input.catatan,
],
)
.map_err(|e| format!("Gagal buat dokumen: {}", e))?;

Ok(conn.last_insert_rowid())
}

pub fn tetapkan(conn: &Connection, id: i64, label: &str) -> Result<(), String> {
let dokumen = get_by_id(conn, id)?;

if dokumen.status != StatusDokumen::Konsep {
return Err("Hanya dokumen berstatus konsep yang bisa ditetapkan".to_string());
}

conn.execute(
"UPDATE dokumen SET status = 'ditetapkan', ditetapkan_at = CURRENT_TIMESTAMP WHERE id = ?1",
rusqlite::params![id],
)
.map_err(|e| e.to_string())?;

let data_json = ambil_data_json(conn, id)?;
conn.execute(
"INSERT INTO salinan_final (dokumen_id, label, data_json) VALUES (?1, ?2, ?3)",
rusqlite::params![id, label, data_json],
)
.map_err(|e| e.to_string())?;

conn.execute(
"INSERT INTO jejak_rekam (dokumen_id, entitas, entitas_id, aksi, pesan)
VALUES (?1, 'dokumen', ?1, 'tetapkan', ?2)",
rusqlite::params![id, format!("Dokumen ditetapkan: {}", label)],
)
.map_err(|e| e.to_string())?;

Ok(())
}

pub fn get_by_id(conn: &Connection, id: i64) -> Result<Dokumen, String> {
conn.query_row(
"SELECT id, tahun_anggaran_id, jenis, versi, nomor_se, tanggal_se,
status, asal_id, asal_keterangan, catatan, ditetapkan_at, created_at
FROM dokumen WHERE id = ?1",
rusqlite::params![id],
|row| {
let jenis_str: String = row.get(2)?;
let status_str: String = row.get(6)?;
Ok(Dokumen {
id: row.get(0)?,
tahun_anggaran_id: row.get(1)?,
jenis: JenisDokumen::from_str(&jenis_str).unwrap_or(JenisDokumen::RkaSkpd),
versi: row.get(3)?,
nomor_se: row.get(4)?,
tanggal_se: row.get(5)?,
status: StatusDokumen::from_str(&status_str).unwrap_or(StatusDokumen::Draft),
asal_id: row.get(7)?,
asal_keterangan: row.get(8)?,
catatan: row.get(9)?,
ditetapkan_at: row.get(10)?,
created_at: row.get(11)?,
})
},
)
.map_err(|e| e.to_string())
}

fn ambil_data_json(conn: &Connection, dokumen_id: i64) -> Result<String, String> {
let dokumen = get_by_id(conn, dokumen_id)?;
let data = serde_json::json!({
"dokumen": dokumen,
});
Ok(data.to_string())
}
