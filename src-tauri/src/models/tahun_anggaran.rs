use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TahunAnggaran {
    pub id: i64,
    pub tahun: i32,
    pub aktif: bool,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TambahTahunAnggaranInput {
    pub tahun: i32,
}
