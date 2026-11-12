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
    match input.jenis {
        JenisDokumen::DpaSkpd => {
            let asal_id = input.asal_id.ok_or(
                "DPA-SKPD harus memiliki asal dokumen RKA-SKPD di Aerarium".to_string()
            )?;
            let asal = get_by_id(conn, asal_id)?;
            if asal.jenis != JenisDokumen::RkaSkpd {
                return Err("Asal DPA-SKPD harus berupa RKA-SKPD".to_string());
            }
            if asal.tahun_anggaran_id != input.tahun_anggaran_id {
                return Err("Asal dokumen harus dari tahun anggaran yang sama".to_string());
            }
        }
        JenisDokumen::DppaSkpd => {
            match (input.asal_id, input.asal_keterangan.as_deref()) {
                (Some(asal_id), _) => {
                    let asal = get_by_id(conn, asal_id)?;
                    if asal.jenis != JenisDokumen::RkaSkpdPerubahan {
                        return Err("Asal DPPA-SKPD harus berupa RKA-SKPD Perubahan".to_string());
                    }
                    if asal.tahun_anggaran_id != input.tahun_anggaran_id {
                        return Err("Asal dokumen harus dari tahun anggaran yang sama".to_string());
                    }
                }
                (None, Some(ket)) if !ket.trim().is_empty() => {}
                _ => {
                    return Err("DPPA-SKPD harus memiliki asal: pilih RKA-SKPD Perubahan di Aerarium atau isi keterangan SIPD".to_string());
                }
            }
        }
        JenisDokumen::RkaSkpdPerubahan => {}
        JenisDokumen::RkaSkpd => {}
    }

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
"INSERT INTO dokumen (tahun_anggaran_id, jenis, versi, nomor_se, tanggal_se, asal_id, asal_keterangan, catatan)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        rusqlite::params![
            input.tahun_anggaran_id,
            input.jenis.as_str(),
            versi_berikutnya,
            input.nomor_se,
            input.tanggal_se,
            input.asal_id,
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

pub fn jadikan_rka(conn: &Connection, kertas_kerja_id: i64) -> Result<i64, String> {
    let asal = get_by_id(conn, kertas_kerja_id)?;

    if asal.jenis.as_str() != "Kertas Kerja" {
        return Err("Hanya Kertas Kerja yang bisa dijadikan RKA-SKPD".to_string());
    }

    // Buat dokumen RKA baru
    let versi_berikutnya: i64 = conn.query_row(
        "SELECT COALESCE(MAX(versi), 0) + 1 FROM dokumen
         WHERE tahun_anggaran_id = ?1 AND jenis = 'RKA-SKPD'",
        rusqlite::params![asal.tahun_anggaran_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO dokumen (tahun_anggaran_id, jenis, versi, asal_id, asal_keterangan, status)
         VALUES (?1, 'RKA-SKPD', ?2, ?3, 'Dari Kertas Kerja', 'draft')",
        rusqlite::params![asal.tahun_anggaran_id, versi_berikutnya, kertas_kerja_id],
    ).map_err(|e| e.to_string())?;
    let rka_id = conn.last_insert_rowid();

    // Salin semua sub kegiatan
    let dsk_list: Vec<(i64, i64, f64, Option<i64>)> = {
        let mut stmt = conn.prepare(
            "SELECT id, master_sub_kegiatan_id, pagu, sumber_dana_id
             FROM dokumen_sub_kegiatan WHERE dokumen_id = ?1"
        ).map_err(|e| e.to_string())?;
        stmt.query_map(rusqlite::params![kertas_kerja_id], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        }).map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect()
    };

    for (old_dsk_id, msk_id, pagu, sumber_dana_id) in &dsk_list {
        // Ambil detail sub kegiatan
        let (lokasi, waktu, keluaran, target): (Option<String>, Option<String>, Option<String>, Option<String>) = conn.query_row(
            "SELECT lokasi, waktu_pelaksanaan, keluaran, target_keluaran
             FROM dokumen_sub_kegiatan WHERE id = ?1",
            rusqlite::params![old_dsk_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "INSERT INTO dokumen_sub_kegiatan
                (dokumen_id, master_sub_kegiatan_id, pagu, sumber_dana_id, lokasi, waktu_pelaksanaan, keluaran, target_keluaran)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![rka_id, msk_id, pagu, sumber_dana_id, lokasi, waktu, keluaran, target],
        ).map_err(|e| e.to_string())?;
        let new_dsk_id = conn.last_insert_rowid();

        // Salin rincian rekening
        let rek_list: Vec<(i64, i64, i64)> = {
            let mut stmt = conn.prepare(
                "SELECT id, rekening_id, urutan FROM rincian_rekening WHERE dokumen_sub_kegiatan_id = ?1"
            ).map_err(|e| e.to_string())?;
            stmt.query_map(rusqlite::params![old_dsk_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?))
            }).map_err(|e| e.to_string())?
            .filter_map(|r| r.ok())
            .collect()
        };

        for (old_rek_id, rekening_id, rek_urutan) in &rek_list {
            conn.execute(
                "INSERT INTO rincian_rekening (dokumen_sub_kegiatan_id, rekening_id, urutan)
                 VALUES (?1, ?2, ?3)",
                rusqlite::params![new_dsk_id, rekening_id, rek_urutan],
            ).map_err(|e| e.to_string())?;
            let new_rek_id = conn.last_insert_rowid();

            // Salin pengelompokan
            let pg_list: Vec<(i64, String, String, Option<i64>, i64)> = {
                let mut stmt = conn.prepare(
                    "SELECT id, label, jenis, sumber_dana_id, urutan
                     FROM rincian_pengelompokan WHERE rincian_rekening_id = ?1"
                ).map_err(|e| e.to_string())?;
                stmt.query_map(rusqlite::params![old_rek_id], |row| {
                    Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?))
                }).map_err(|e| e.to_string())?
                .filter_map(|r| r.ok())
                .collect()
            };

            for (old_pg_id, pg_label, pg_jenis, pg_sd_id, pg_urutan) in &pg_list {
                conn.execute(
                    "INSERT INTO rincian_pengelompokan (rincian_rekening_id, label, jenis, sumber_dana_id, urutan)
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![new_rek_id, pg_label, pg_jenis, pg_sd_id, pg_urutan],
                ).map_err(|e| e.to_string())?;
                let new_pg_id = conn.last_insert_rowid();

                // Salin keterangan
                let ket_list: Vec<(i64, String, i64)> = {
                    let mut stmt = conn.prepare(
                        "SELECT id, label, urutan FROM rincian_keterangan WHERE rincian_pengelompokan_id = ?1"
                    ).map_err(|e| e.to_string())?;
                    stmt.query_map(rusqlite::params![old_pg_id], |row| {
                        Ok((row.get(0)?, row.get(1)?, row.get(2)?))
                    }).map_err(|e| e.to_string())?
                    .filter_map(|r| r.ok())
                    .collect()
                };

                for (old_ket_id, ket_label, ket_urutan) in &ket_list {
                    conn.execute(
                        "INSERT INTO rincian_keterangan (rincian_pengelompokan_id, label, urutan)
                         VALUES (?1, ?2, ?3)",
                        rusqlite::params![new_pg_id, ket_label, ket_urutan],
                    ).map_err(|e| e.to_string())?;
                    let new_ket_id = conn.last_insert_rowid();

                    // Salin item
                    let mut stmt = conn.prepare(
                        "SELECT shs_id, uraian, spesifikasi, volume, satuan,
                                koef2, satuan2, koef3, satuan3, koef4, satuan4,
                                harga, ppn, urutan
                         FROM rincian_item WHERE rincian_keterangan_id = ?1"
                    ).map_err(|e| e.to_string())?;
                    let items: Vec<_> = stmt.query_map(rusqlite::params![old_ket_id], |row| {
                        Ok((
                            row.get::<_,Option<i64>>(0)?, row.get::<_,String>(1)?,
                            row.get::<_,Option<String>>(2)?, row.get::<_,f64>(3)?,
                            row.get::<_,String>(4)?, row.get::<_,Option<f64>>(5)?,
                            row.get::<_,Option<String>>(6)?, row.get::<_,Option<f64>>(7)?,
                            row.get::<_,Option<String>>(8)?, row.get::<_,Option<f64>>(9)?,
                            row.get::<_,Option<String>>(10)?, row.get::<_,f64>(11)?,
                            row.get::<_,f64>(12)?, row.get::<_,i64>(13)?,
                        ))
                    }).map_err(|e| e.to_string())?
                    .filter_map(|r| r.ok())
                    .collect();

                    for (shs_id, uraian, spesifikasi, volume, satuan,
                         koef2, satuan2, koef3, satuan3, koef4, satuan4,
                         harga, ppn, urutan) in items {
                        conn.execute(
                            "INSERT INTO rincian_item
                                (rincian_keterangan_id, shs_id, uraian, spesifikasi,
                                 volume, satuan, koef2, satuan2, koef3, satuan3,
                                 koef4, satuan4, harga, ppn, urutan)
                             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15)",
                            rusqlite::params![
                                new_ket_id, shs_id, uraian, spesifikasi,
                                volume, satuan, koef2, satuan2, koef3, satuan3,
                                koef4, satuan4, harga, ppn, urutan
                            ],
                        ).map_err(|e| e.to_string())?;
                    }
                }
            }
        }
    }

    // Jejak rekam
    conn.execute(
        "INSERT INTO jejak_rekam (dokumen_id, entitas, entitas_id, aksi, pesan)
         VALUES (?1, 'dokumen', ?1, 'jadikan_rka', ?2)",
        rusqlite::params![rka_id, format!("Dibuat dari Kertas Kerja id={}", kertas_kerja_id)],
    ).map_err(|e| e.to_string())?;

    Ok(rka_id)
}

pub fn get_dashboard_summary(conn: &Connection, tahun_anggaran_id: i64) -> Result<(f64, f64, i64), String> {
    let total_pagu: f64 = conn.query_row(
        "SELECT COALESCE(SUM(dsk.pagu), 0)
         FROM dokumen_sub_kegiatan dsk
         JOIN dokumen d ON d.id = dsk.dokumen_id
         WHERE d.tahun_anggaran_id = ?1 AND d.status NOT IN ('dikunci','diarsipkan')",
        rusqlite::params![tahun_anggaran_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let total_rincian: f64 = conn.query_row(
        "SELECT COALESCE(SUM(
            ri.volume
            * COALESCE(ri.koef2, 1.0)
            * COALESCE(ri.koef3, 1.0)
            * COALESCE(ri.koef4, 1.0)
            * ri.harga
            * (1.0 + ri.ppn / 100.0)
         ), 0)
         FROM rincian_item ri
         JOIN rincian_keterangan rk ON rk.id = ri.rincian_keterangan_id
         JOIN rincian_pengelompokan rp ON rp.id = rk.rincian_pengelompokan_id
         JOIN rincian_rekening rrek ON rrek.id = rp.rincian_rekening_id
         JOIN dokumen_sub_kegiatan dsk ON dsk.id = rrek.dokumen_sub_kegiatan_id
         JOIN dokumen d ON d.id = dsk.dokumen_id
         WHERE d.tahun_anggaran_id = ?1 AND d.status NOT IN ('dikunci','diarsipkan')",
        rusqlite::params![tahun_anggaran_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    let sub_warning: i64 = conn.query_row(
        "SELECT COUNT(DISTINCT dsk.id)
         FROM dokumen_sub_kegiatan dsk
         JOIN dokumen d ON d.id = dsk.dokumen_id
         JOIN rincian_rekening rrek ON rrek.dokumen_sub_kegiatan_id = dsk.id
         JOIN rincian_pengelompokan rp ON rp.rincian_rekening_id = rrek.id
         JOIN rincian_keterangan rk ON rk.rincian_pengelompokan_id = rp.id
         JOIN rincian_item ri ON ri.rincian_keterangan_id = rk.id
         LEFT JOIN shs s ON s.id = ri.shs_id
         WHERE d.tahun_anggaran_id = ?1
           AND d.status NOT IN ('dikunci','diarsipkan')
           AND (ri.shs_id IS NULL OR s.sumber != 'resmi')",
        rusqlite::params![tahun_anggaran_id],
        |row| row.get(0),
    ).map_err(|e| e.to_string())?;

    Ok((total_pagu, total_rincian, sub_warning))
}