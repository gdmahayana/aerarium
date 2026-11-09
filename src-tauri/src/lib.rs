mod db;
mod commands;
mod models;

use commands::import_ssh::{import_shs_from_excel, get_shs as get_shs_cmd};
use commands::rincian as rin;
use commands::kegiatan as keg;
use models::kegiatan::{TambahKegiatanInput, TambahSubKegiatanInput};
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

// Kegiatan
#[tauri::command]
fn get_kegiatan(dokumen_id: i64) -> Result<Vec<models::kegiatan::Kegiatan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::get_kegiatan(&conn, dokumen_id)
}

#[tauri::command]
fn tambah_kegiatan(input: TambahKegiatanInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::tambah_kegiatan(&conn, input)
}

#[tauri::command]
fn hapus_kegiatan(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::hapus_kegiatan(&conn, id)
}

// Sub Kegiatan
#[tauri::command]
fn get_sub_kegiatan(kegiatan_id: i64) -> Result<Vec<models::kegiatan::SubKegiatan>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::get_sub_kegiatan(&conn, kegiatan_id)
}

#[tauri::command]
fn tambah_sub_kegiatan(input: TambahSubKegiatanInput) -> Result<i64, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::tambah_sub_kegiatan(&conn, input)
}

#[tauri::command]
fn hapus_sub_kegiatan(id: i64) -> Result<(), String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    keg::hapus_sub_kegiatan(&conn, id)
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
fn get_rincian_rekening(sub_kegiatan_id: i64) -> Result<Vec<models::rincian::RincianRekening>, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    rin::get_rincian_rekening(&conn, sub_kegiatan_id)
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
            get_kegiatan,
            tambah_kegiatan,
            hapus_kegiatan,
            get_sub_kegiatan,
            tambah_sub_kegiatan,
            hapus_sub_kegiatan,
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
