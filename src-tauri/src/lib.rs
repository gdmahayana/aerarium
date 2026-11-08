mod db;
mod commands;
mod models;

use commands::import_ssh::import_shs_from_excel;
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db::init_db().expect("Gagal inisialisasi database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            import_shs,
            get_tahun_anggaran,
            tambah_tahun_anggaran,
            set_tahun_anggaran_aktif,
            hapus_tahun_anggaran,
            get_dokumen,
            buat_dokumen,
            tetapkan_dokumen,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application")
}
