use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(r#"
        PRAGMA foreign_keys = ON;

        CREATE TABLE IF NOT EXISTS migrasi (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            versi INTEGER NOT NULL UNIQUE,
            deskripsi TEXT NOT NULL,
            diterapkan_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS tahun_anggaran (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun INTEGER NOT NULL UNIQUE,
            aktif INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS dokumen (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun_anggaran_id INTEGER NOT NULL,
            jenis TEXT NOT NULL CHECK(jenis IN (
                'Kertas Kerja','RKA-SKPD','DPA-SKPD','RKA-SKPD Perubahan','DPPA-SKPD'
            )),
            versi INTEGER NOT NULL DEFAULT 1,
            nomor_se TEXT,
            tanggal_se TEXT,
            status TEXT NOT NULL CHECK(status IN (
                'draft','konsep','ditetapkan','dikunci','diarsipkan'
            )) DEFAULT 'draft',
            asal_id INTEGER,
            asal_keterangan TEXT,
            catatan TEXT,
            ditetapkan_at TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id),
            FOREIGN KEY (asal_id) REFERENCES dokumen(id)
        );

        CREATE TABLE IF NOT EXISTS salinan_final (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL UNIQUE,
            label TEXT NOT NULL,
            catatan TEXT,
            data_json TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id)
        );

        CREATE TABLE IF NOT EXISTS shs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun_anggaran_id INTEGER NOT NULL,
            jenis TEXT NOT NULL DEFAULT 'SSH' CHECK(jenis IN ('SSH','SBU','HSPK','ASB')),
            sumber TEXT NOT NULL DEFAULT 'rencana' CHECK(sumber IN ('rencana','resmi')),
            kode TEXT NOT NULL,
            kode_kelompok TEXT,
            uraian_kelompok TEXT,
            id_sipd TEXT,
            uraian TEXT NOT NULL,
            spesifikasi TEXT,
            satuan TEXT NOT NULL,
            harga REAL NOT NULL DEFAULT 0,
            kode_rekening TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id)
        );

        CREATE TABLE IF NOT EXISTS rekening (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kode TEXT NOT NULL UNIQUE,
            uraian TEXT NOT NULL,
            level INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sumber_dana (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun_anggaran_id INTEGER NOT NULL,
            kode TEXT NOT NULL,
            nama TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id)
        );

        CREATE TABLE IF NOT EXISTS jejak_rekam (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL,
            entitas TEXT NOT NULL,
            entitas_id INTEGER NOT NULL,
            aksi TEXT NOT NULL,
            nilai_lama TEXT,
            nilai_baru TEXT,
            pesan TEXT,
            tingkat TEXT NOT NULL DEFAULT 'info' CHECK(tingkat IN ('info','warning','critical')),
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id)
        );

        CREATE TABLE IF NOT EXISTS rencana_sementara (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL,
            label TEXT,
            data_json TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id)
        );

        CREATE TABLE IF NOT EXISTS master_program (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kode TEXT NOT NULL,
            uraian TEXT NOT NULL,
            aktif INTEGER NOT NULL DEFAULT 1,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS master_kegiatan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            master_program_id INTEGER NOT NULL,
            kode TEXT NOT NULL,
            uraian TEXT NOT NULL,
            aktif INTEGER NOT NULL DEFAULT 1,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (master_program_id) REFERENCES master_program(id)
        );

        CREATE TABLE IF NOT EXISTS master_sub_kegiatan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            master_kegiatan_id INTEGER NOT NULL,
            kode TEXT NOT NULL,
            uraian TEXT NOT NULL,
            aktif INTEGER NOT NULL DEFAULT 1,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (master_kegiatan_id) REFERENCES master_kegiatan(id)
        );

        CREATE TABLE IF NOT EXISTS dokumen_sub_kegiatan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL,
            master_sub_kegiatan_id INTEGER NOT NULL,
            pagu REAL NOT NULL DEFAULT 0,
            sumber_dana_id INTEGER,
            lokasi TEXT,
            waktu_pelaksanaan TEXT,
            keluaran TEXT,
            target_keluaran TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id),
            FOREIGN KEY (master_sub_kegiatan_id) REFERENCES master_sub_kegiatan(id),
            FOREIGN KEY (sumber_dana_id) REFERENCES sumber_dana(id)
        );

        CREATE TABLE IF NOT EXISTS rincian_rekening (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_sub_kegiatan_id INTEGER NOT NULL,
            rekening_id INTEGER NOT NULL,
            urutan INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_sub_kegiatan_id) REFERENCES dokumen_sub_kegiatan(id),
            FOREIGN KEY (rekening_id) REFERENCES rekening(id)
        );

        CREATE TABLE IF NOT EXISTS rincian_pengelompokan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rincian_rekening_id INTEGER NOT NULL,
            label TEXT NOT NULL,
            jenis TEXT NOT NULL DEFAULT 'pengelompokan' CHECK(jenis IN ('pengelompokan','pemaketan')),
            sumber_dana_id INTEGER,
            urutan INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (rincian_rekening_id) REFERENCES rincian_rekening(id),
            FOREIGN KEY (sumber_dana_id) REFERENCES sumber_dana(id)
        );

        CREATE TABLE IF NOT EXISTS rincian_keterangan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rincian_pengelompokan_id INTEGER NOT NULL,
            label TEXT NOT NULL,
            urutan INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (rincian_pengelompokan_id) REFERENCES rincian_pengelompokan(id)
        );

        CREATE TABLE IF NOT EXISTS rincian_item (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rincian_keterangan_id INTEGER NOT NULL,
            shs_id INTEGER,
            uraian TEXT NOT NULL,
            spesifikasi TEXT,
            volume REAL NOT NULL DEFAULT 1,
            satuan TEXT NOT NULL,
            harga REAL NOT NULL DEFAULT 0,
            ppn REAL NOT NULL DEFAULT 0,
            realisasi REAL NOT NULL DEFAULT 0,
            dikunci INTEGER NOT NULL DEFAULT 0,
            urutan INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (rincian_keterangan_id) REFERENCES rincian_keterangan(id),
            FOREIGN KEY (shs_id) REFERENCES shs(id)
        );

        INSERT OR IGNORE INTO migrasi (versi, deskripsi) VALUES (1, 'Initial schema');
        INSERT OR IGNORE INTO migrasi (versi, deskripsi) VALUES (2, 'Rename ssh to shs, tambah jenis SHS, tambah asal_keterangan dokumen');
        INSERT OR IGNORE INTO migrasi (versi, deskripsi) VALUES (3, 'Tambah sumber_dana, rincian_rekening, rincian_pengelompokan, rincian_keterangan, rincian_item');
        INSERT OR IGNORE INTO migrasi (versi, deskripsi) VALUES (4, 'Refactor ke master data: master_program, master_kegiatan, master_sub_kegiatan, dokumen_sub_kegiatan');
    "#)?;

    Ok(())
}
