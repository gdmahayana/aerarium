use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dokumen {
    pub id: i64,
    pub tahun_anggaran_id: i64,
    pub jenis: JenisDokumen,
    pub versi: i32,
    pub nomor_se: Option<String>,
    pub tanggal_se: Option<String>,
    pub status: StatusDokumen,
    pub asal_id: Option<i64>,
    pub asal_keterangan: Option<String>,
    pub catatan: Option<String>,
    pub ditetapkan_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum JenisDokumen {
    #[serde(rename = "RKA-SKPD")]
    RkaSkpd,
    #[serde(rename = "DPA-SKPD")]
    DpaSkpd,
    #[serde(rename = "RKA-SKPD Perubahan")]
    RkaSkpdPerubahan,
    #[serde(rename = "DPPA-SKPD")]
    DppaSkpd,
}

impl JenisDokumen {
    pub fn as_str(&self) -> &str {
        match self {
            JenisDokumen::RkaSkpd => "RKA-SKPD",
            JenisDokumen::DpaSkpd => "DPA-SKPD",
            JenisDokumen::RkaSkpdPerubahan => "RKA-SKPD Perubahan",
            JenisDokumen::DppaSkpd => "DPPA-SKPD",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "RKA-SKPD" => Some(JenisDokumen::RkaSkpd),
            "DPA-SKPD" => Some(JenisDokumen::DpaSkpd),
            "RKA-SKPD Perubahan" => Some(JenisDokumen::RkaSkpdPerubahan),
            "DPPA-SKPD" => Some(JenisDokumen::DppaSkpd),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum StatusDokumen {
    Draft,
    Konsep,
    Ditetapkan,
    Dikunci,
    Diarsipkan,
}

impl StatusDokumen {
    pub fn as_str(&self) -> &str {
        match self {
            StatusDokumen::Draft => "draft",
            StatusDokumen::Konsep => "konsep",
            StatusDokumen::Ditetapkan => "ditetapkan",
            StatusDokumen::Dikunci => "dikunci",
            StatusDokumen::Diarsipkan => "diarsipkan",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "draft" => Some(StatusDokumen::Draft),
            "konsep" => Some(StatusDokumen::Konsep),
            "ditetapkan" => Some(StatusDokumen::Ditetapkan),
            "dikunci" => Some(StatusDokumen::Dikunci),
            "diarsipkan" => Some(StatusDokumen::Diarsipkan),
            _ => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BuatDokumenInput {
    pub tahun_anggaran_id: i64,
    pub jenis: JenisDokumen,
    pub nomor_se: Option<String>,
    pub tanggal_se: Option<String>,
    pub asal_id: Option<i64>,
    pub asal_keterangan: Option<String>,
    pub catatan: Option<String>,
}
