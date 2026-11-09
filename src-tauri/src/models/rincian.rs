use serde::{Deserialize, Serialize};

// Sumber Dana
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SumberDana {
    pub id: i64,
    pub tahun_anggaran_id: i64,
    pub kode: String,
    pub nama: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahSumberDanaInput {
    pub tahun_anggaran_id: i64,
    pub kode: String,
    pub nama: String,
}

// Rincian Rekening
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RincianRekening {
    pub id: i64,
    pub sub_kegiatan_id: i64,
    pub rekening_id: i64,
    pub rekening_kode: Option<String>,
    pub rekening_uraian: Option<String>,
    pub urutan: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahRincianRekeningInput {
    pub sub_kegiatan_id: i64,
    pub rekening_id: i64,
    pub urutan: Option<i32>,
}

// Rincian Pengelompokan
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RincianPengelompokan {
    pub id: i64,
    pub rincian_rekening_id: i64,
    pub label: String,
    pub jenis: String,
    pub sumber_dana_id: Option<i64>,
    pub sumber_dana_nama: Option<String>,
    pub urutan: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahRincianPengelompokanInput {
    pub rincian_rekening_id: i64,
    pub label: String,
    pub jenis: Option<String>,
    pub sumber_dana_id: Option<i64>,
    pub urutan: Option<i32>,
}

// Rincian Keterangan
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RincianKeterangan {
    pub id: i64,
    pub rincian_pengelompokan_id: i64,
    pub label: String,
    pub urutan: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahRincianKeteranganInput {
    pub rincian_pengelompokan_id: i64,
    pub label: String,
    pub urutan: Option<i32>,
}

// Rincian Item
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RincianItem {
    pub id: i64,
    pub rincian_keterangan_id: i64,
    pub shs_id: Option<i64>,
    pub uraian: String,
    pub spesifikasi: Option<String>,
    pub volume: f64,
    pub satuan: String,
    pub harga: f64,
    pub ppn: f64,
    pub realisasi: f64,
    pub dikunci: bool,
    pub urutan: i32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahRincianItemInput {
    pub rincian_keterangan_id: i64,
    pub shs_id: Option<i64>,
    pub uraian: String,
    pub spesifikasi: Option<String>,
    pub volume: f64,
    pub satuan: String,
    pub harga: f64,
    pub ppn: Option<f64>,
    pub urutan: Option<i32>,
}
