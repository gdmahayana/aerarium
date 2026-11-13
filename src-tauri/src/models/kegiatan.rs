use serde::{Deserialize, Serialize};

// =====================
// Master Program
// =====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MasterProgram {
    pub id: i64,
    pub kode: String,
    pub uraian: String,
    pub aktif: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahMasterProgramInput {
    pub kode: String,
    pub uraian: String,
}

// =====================
// Master Kegiatan
// =====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MasterKegiatan {
    pub id: i64,
    pub master_program_id: i64,
    pub kode: String,
    pub uraian: String,
    pub aktif: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahMasterKegiatanInput {
    pub master_program_id: i64,
    pub kode: String,
    pub uraian: String,
}

// =====================
// Master Sub Kegiatan
// =====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MasterSubKegiatan {
    pub id: i64,
    pub master_kegiatan_id: i64,
    pub kode: String,
    pub uraian: String,
    pub aktif: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahMasterSubKegiatanInput {
    pub master_kegiatan_id: i64,
    pub kode: String,
    pub uraian: String,
}

// =====================
// Dokumen Sub Kegiatan
// =====================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DokumenSubKegiatan {
    pub id: i64,
    pub dokumen_id: i64,
    pub master_sub_kegiatan_id: i64,
    pub pagu: f64,
    pub sumber_dana_id: Option<i64>,
    pub lokasi: Option<String>,
    pub waktu_pelaksanaan: Option<String>,
    pub keluaran: Option<String>,
    pub target_keluaran: Option<String>,
    pub bebas_shs: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahDokumenSubKegiatanInput {
    pub dokumen_id: i64,
    pub master_sub_kegiatan_id: i64,
    pub pagu: Option<f64>,
    pub sumber_dana_id: Option<i64>,
    pub lokasi: Option<String>,
    pub waktu_pelaksanaan: Option<String>,
    pub keluaran: Option<String>,
    pub target_keluaran: Option<String>,
    pub bebas_shs: Option<bool>,
}
