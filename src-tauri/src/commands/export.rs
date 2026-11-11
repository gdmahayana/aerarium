use rusqlite::Connection;
use rust_xlsxwriter::*;

pub fn export_shs_rencana(conn: &Connection, tahun_anggaran_id: i64, output_path: &str) -> Result<usize, String> {
    let mut stmt = conn.prepare(
        "SELECT jenis, kode, uraian, spesifikasi, satuan, harga FROM shs WHERE tahun_anggaran_id = ?1 AND sumber = 'rencana' ORDER BY jenis, kode"
    ).map_err(|e| e.to_string())?;

    let rows: Vec<(String, String, String, Option<String>, String, f64)> = stmt
        .query_map(rusqlite::params![tahun_anggaran_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })
        .map_err(|e| e.to_string())?
        .filter_map(|r| r.ok())
        .collect();

    let count = rows.len();

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();

    // Format header
    let fmt_header = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x1e3a5f))
        .set_font_color(Color::White)
        .set_border(FormatBorder::Thin);

    let fmt_number = Format::new()
        .set_num_format("#,##0");

    let fmt_border = Format::new()
        .set_border(FormatBorder::Thin);

    // Header kolom
    let headers = ["No", "Jenis", "Kode", "Uraian", "Spesifikasi", "Satuan", "Harga Satuan"];
    for (i, h) in headers.iter().enumerate() {
        sheet.write_with_format(0, i as u16, *h, &fmt_header)
            .map_err(|e| e.to_string())?;
    }

    // Lebar kolom
    sheet.set_column_width(0, 5).map_err(|e| e.to_string())?;
    sheet.set_column_width(1, 8).map_err(|e| e.to_string())?;
    sheet.set_column_width(2, 30).map_err(|e| e.to_string())?;
    sheet.set_column_width(3, 40).map_err(|e| e.to_string())?;
    sheet.set_column_width(4, 30).map_err(|e| e.to_string())?;
    sheet.set_column_width(5, 12).map_err(|e| e.to_string())?;
    sheet.set_column_width(6, 18).map_err(|e| e.to_string())?;

    // Data
    for (i, (jenis, kode, uraian, spesifikasi, satuan, harga)) in rows.iter().enumerate() {
        let row = (i + 1) as u32;
        sheet.write_with_format(row, 0, (i + 1) as u32, &fmt_border).map_err(|e| e.to_string())?;
        sheet.write_with_format(row, 1, jenis.as_str(), &fmt_border).map_err(|e| e.to_string())?;
        sheet.write_with_format(row, 2, kode.as_str(), &fmt_border).map_err(|e| e.to_string())?;
        sheet.write_with_format(row, 3, uraian.as_str(), &fmt_border).map_err(|e| e.to_string())?;
        sheet.write_with_format(row, 4, spesifikasi.as_deref().unwrap_or(""), &fmt_border).map_err(|e| e.to_string())?;
        sheet.write_with_format(row, 5, satuan.as_str(), &fmt_border).map_err(|e| e.to_string())?;
        sheet.write_with_format(row, 6, *harga, &fmt_number).map_err(|e| e.to_string())?;
    }

    workbook.save(output_path).map_err(|e| e.to_string())?;
    Ok(count)
}
