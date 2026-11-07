mod db;
mod commands;

use commands::import_ssh::import_ssh_from_excel;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn import_ssh(file_path: String, tahun_anggaran_id: i64) -> Result<usize, String> {
    let conn = db::init_db().map_err(|e| e.to_string())?;
    import_ssh_from_excel(&conn, &file_path, tahun_anggaran_id)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    db::init_db().expect("Gagal inisialisasi database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, import_ssh])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}