mod db;
mod commands;
mod models;

use commands::import_ssh::{import_shs_from_excel, get_shs as get_shs_cmd};
use commands::rincian as rin;
use commands::kegiatan as keg;
use models::kegiatan::{
    TambahMasterProgramInput,
    TambahMasterKegiatanInput,
    TambahMasterSubKegiatanInput,
    TambahDokumenSubKegiatanInput,
};
use models::rincian::{
    TambahSumberDanaInput,
    TambahRincianRekeningInput,
    TambahRincianPengelompokanInput,
    TambahRincianKeteranganInput,
    TambahRincianItemInput,
};
use commands::tahun_anggaran as ta;
use commands::dokumen as dok;
use models::tahun_anggaran::TambahTahunAnggaranInput;
use models::dokumen::BuatDokumenInput;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You have been greeted from Rust!", name)
}

#[tauri::command]
fn import_shs(file_path: String, tahun_anggaran_id: i64, jenis: String) -> Result<usize, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    import_shs_from_excel(&conn, &file_path, tahun_anggaran_id, &jenis)
}

#[tauri::command]
fn get_shs(tahun_anggaran_id: i64) -> Result<Vec<commands::import_ssh::ShsItem>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    get_shs_cmd(&conn, tahun_anggaran_id)
}

// Tahun Anggaran
#[tauri::command]
fn get_tahun_anggaran() -> Result<Vec<models::tahun_anggaran::TahunAnggaran>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    ta::get_all(&conn)
}

#[tauri::command]
fn tambah_tahun_anggaran(input: TambahTahunAnggaranInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    ta::tambah(&conn, input)
}

#[tauri::command]
fn set_tahun_anggaran_aktif(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    ta::set_aktif(&conn, id)
}

#[tauri::command]
fn hapus_tahun_anggaran(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    ta::hapus(&conn, id)
}

// Dokumen
#[tauri::command]
fn get_dokumen(tahun_anggaran_id: i64) -> Result<Vec<models::dokumen::Dokumen>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    dok::get_all(&conn, tahun_anggaran_id)
}

#[tauri::command]
fn buat_dokumen(input: BuatDokumenInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    dok::buat(&conn, input)
}

#[tauri::command]
fn tetapkan_dokumen(id: i64, label: String) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    dok::tetapkan(&conn, id, &label)
}

// Rekening
#[tauri::command]
fn cari_atau_buat_rekening(kode: String, uraian: String) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    let existing: Result<i64, _> = conn.query_row(
        "SELECT id FROM rekening WHERE kode = ?1",
        rusqlite::params![kode],
        |row| row.get(0),
    );
    match existing {
        Ok(id) => Ok(id),
        Err(_) => {
            let level = kode.matches(".").count() as i32;
            conn.execute(
                "INSERT INTO rekening (kode, uraian, level) VALUES (?1, ?2, ?3)",
                rusqlite::params![kode, uraian, level],
            ).map_err(|e| format!("Gagal buat rekening: {}", e))?;
            Ok(conn.last_insert_rowid())
        }
    }
}

// Master Program
#[tauri::command]
fn get_master_program() -> Result<Vec<models::kegiatan::MasterProgram>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::get_master_program(&conn)
}

#[tauri::command]
fn tambah_master_program(input: TambahMasterProgramInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::tambah_master_program(&conn, input)
}

#[tauri::command]
fn toggle_master_program(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::toggle_master_program(&conn, id)
}

#[tauri::command]
fn hapus_master_program(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::hapus_master_program(&conn, id)
}

// Master Kegiatan
#[tauri::command]
fn get_master_kegiatan(master_program_id: i64) -> Result<Vec<models::kegiatan::MasterKegiatan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::get_master_kegiatan(&conn, master_program_id)
}

#[tauri::command]
fn tambah_master_kegiatan(input: TambahMasterKegiatanInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::tambah_master_kegiatan(&conn, input)
}

#[tauri::command]
fn toggle_master_kegiatan(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::toggle_master_kegiatan(&conn, id)
}

#[tauri::command]
fn hapus_master_kegiatan(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::hapus_master_kegiatan(&conn, id)
}

// Master Sub Kegiatan
#[tauri::command]
fn get_master_sub_kegiatan(master_kegiatan_id: i64) -> Result<Vec<models::kegiatan::MasterSubKegiatan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::get_master_sub_kegiatan(&conn, master_kegiatan_id)
}

#[tauri::command]
fn get_master_sub_kegiatan_aktif() -> Result<Vec<models::kegiatan::MasterSubKegiatan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::get_master_sub_kegiatan_aktif(&conn)
}

#[tauri::command]
fn tambah_master_sub_kegiatan(input: TambahMasterSubKegiatanInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::tambah_master_sub_kegiatan(&conn, input)
}

#[tauri::command]
fn toggle_master_sub_kegiatan(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::toggle_master_sub_kegiatan(&conn, id)
}

#[tauri::command]
fn hapus_master_sub_kegiatan(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::hapus_master_sub_kegiatan(&conn, id)
}

// Dokumen Sub Kegiatan
#[tauri::command]
fn get_dokumen_sub_kegiatan(dokumen_id: i64) -> Result<Vec<models::kegiatan::DokumenSubKegiatan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::get_dokumen_sub_kegiatan(&conn, dokumen_id)
}

#[tauri::command]
fn tambah_dokumen_sub_kegiatan(input: TambahDokumenSubKegiatanInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::tambah_dokumen_sub_kegiatan(&conn, input)
}

#[tauri::command]
fn hapus_dokumen_sub_kegiatan(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::hapus_dokumen_sub_kegiatan(&conn, id)
}

// Sumber Dana
#[tauri::command]
fn get_sumber_dana(tahun_anggaran_id: i64) -> Result<Vec<models::rincian::SumberDana>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::get_sumber_dana(&conn, tahun_anggaran_id)
}

#[tauri::command]
fn tambah_sumber_dana(input: TambahSumberDanaInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::tambah_sumber_dana(&conn, input)
}

// Rincian Rekening
#[tauri::command]
fn get_rincian_rekening(dokumen_sub_kegiatan_id: i64) -> Result<Vec<models::rincian::RincianRekening>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::get_rincian_rekening(&conn, dokumen_sub_kegiatan_id)
}

#[tauri::command]
fn tambah_rincian_rekening(input: TambahRincianRekeningInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::tambah_rincian_rekening(&conn, input)
}

// Rincian Pengelompokan
#[tauri::command]
fn get_rincian_pengelompokan(rincian_rekening_id: i64) -> Result<Vec<models::rincian::RincianPengelompokan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::get_rincian_pengelompokan(&conn, rincian_rekening_id)
}

#[tauri::command]
fn tambah_rincian_pengelompokan(input: TambahRincianPengelompokanInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::tambah_rincian_pengelompokan(&conn, input)
}

// Rincian Keterangan
#[tauri::command]
fn get_rincian_keterangan(rincian_pengelompokan_id: i64) -> Result<Vec<models::rincian::RincianKeterangan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::get_rincian_keterangan(&conn, rincian_pengelompokan_id)
}

#[tauri::command]
fn tambah_rincian_keterangan(input: TambahRincianKeteranganInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::tambah_rincian_keterangan(&conn, input)
}

// Rincian Item
#[tauri::command]
fn get_rincian_item(rincian_keterangan_id: i64) -> Result<Vec<models::rincian::RincianItem>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::get_rincian_item(&conn, rincian_keterangan_id)
}

#[tauri::command]
fn tambah_rincian_item(input: TambahRincianItemInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::tambah_rincian_item(&conn, input)
}

#[tauri::command]
fn hapus_rincian_item(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::hapus_rincian_item(&conn, id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db::init_db().expect("Gagal inisialisasi database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            import_shs,
            get_shs,
            get_tahun_anggaran,
            tambah_tahun_anggaran,
            set_tahun_anggaran_aktif,
            hapus_tahun_anggaran,
            get_dokumen,
            buat_dokumen,
            tetapkan_dokumen,
            cari_atau_buat_rekening,
            get_master_program,
            tambah_master_program,
            toggle_master_program,
            hapus_master_program,
            get_master_kegiatan,
            tambah_master_kegiatan,
            toggle_master_kegiatan,
            hapus_master_kegiatan,
            get_master_sub_kegiatan,
            get_master_sub_kegiatan_aktif,
            tambah_master_sub_kegiatan,
            toggle_master_sub_kegiatan,
            hapus_master_sub_kegiatan,
            get_dokumen_sub_kegiatan,
            tambah_dokumen_sub_kegiatan,
            hapus_dokumen_sub_kegiatan,
            get_sumber_dana,
            tambah_sumber_dana,
            get_rincian_rekening,
            tambah_rincian_rekening,
            get_rincian_pengelompokan,
            tambah_rincian_pengelompokan,
            get_rincian_keterangan,
            tambah_rincian_keterangan,
            get_rincian_item,
            tambah_rincian_item,
            hapus_rincian_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
