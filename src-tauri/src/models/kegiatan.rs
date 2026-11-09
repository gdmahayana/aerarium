use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kegiatan {
    pub id: i64,
    pub dokumen_id: i64,
    pub kode: String,
    pub uraian: String,
    pub pagu: f64,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahKegiatanInput {
    pub dokumen_id: i64,
    pub kode: String,
    pub uraian: String,
    pub pagu: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubKegiatan {
    pub id: i64,
    pub kegiatan_id: i64,
    pub kode: String,
    pub uraian: String,
    pub pagu: f64,
    pub sumber_dana_id: Option<i64>,
    pub lokasi: Option<String>,
    pub waktu_pelaksanaan: Option<String>,
    pub keluaran: Option<String>,
    pub target_keluaran: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahSubKegiatanInput {
    pub kegiatan_id: i64,
    pub kode: String,
    pub uraian: String,
    pub pagu: Option<f64>,
    pub sumber_dana_id: Option<i64>,
    pub lokasi: Option<String>,
    pub waktu_pelaksanaan: Option<String>,
    pub keluaran: Option<String>,
    pub target_keluaran: Option<String>,
}
