use rusqlite::{Connection, Result};

pub fn create_tables(conn: &Connection) -> Result<()> {
    conn.execute_batch(r#"
        PRAGMA foreign_keys = ON;

        -- Migrasi schema
        CREATE TABLE IF NOT EXISTS migrasi (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            versi INTEGER NOT NULL UNIQUE,
            deskripsi TEXT NOT NULL,
            diterapkan_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        -- Tahun Anggaran
        CREATE TABLE IF NOT EXISTS tahun_anggaran (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun INTEGER NOT NULL UNIQUE,
            aktif INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP
        );

        -- Dokumen Anggaran
        -- jenis: RKA-SKPD / DPA-SKPD / RKA-SKPD Perubahan / DPPA-SKPD
        -- status: draft / konsep / ditetapkan / dikunci / diarsipkan
        -- versi: urutan SE dalam satu jenis dokumen
        -- asal_id: dokumen induk (DPA-SKPD dari RKA-SKPD mana)
        CREATE TABLE IF NOT EXISTS dokumen (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun_anggaran_id INTEGER NOT NULL,
            jenis TEXT NOT NULL CHECK(jenis IN (
                'RKA-SKPD',
                'DPA-SKPD',
                'RKA-SKPD Perubahan',
                'DPPA-SKPD'
            )),
            versi INTEGER NOT NULL DEFAULT 1,
            nomor_se TEXT,
            tanggal_se TEXT,
            status TEXT NOT NULL DEFAULT 'draft' CHECK(status IN (
                'draft',
                'konsep',
                'ditetapkan',
                'dikunci',
                'diarsipkan'
            )),
            asal_id INTEGER,
            asal_keterangan TEXT,
            catatan TEXT,
            ditetapkan_at TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id),
            FOREIGN KEY (asal_id) REFERENCES dokumen(id)
        );

        -- Salinan Final (Snapshot)
        -- Dibuat otomatis saat dokumen ditetapkan
        -- Tidak bisa diubah setelah dibuat
        CREATE TABLE IF NOT EXISTS salinan_final (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL UNIQUE,
            label TEXT NOT NULL,
            catatan TEXT,
            data_json TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id)
        );

        -- Kegiatan
        CREATE TABLE IF NOT EXISTS kegiatan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL,
            kode TEXT NOT NULL,
            uraian TEXT NOT NULL,
            pagu REAL NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id)
        );

        -- Sub Kegiatan
        CREATE TABLE IF NOT EXISTS sub_kegiatan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kegiatan_id INTEGER NOT NULL,
            kode TEXT NOT NULL,
            uraian TEXT NOT NULL,
            pagu REAL NOT NULL DEFAULT 0,
            sumber_dana_id INTEGER,
            lokasi TEXT,
            waktu_pelaksanaan TEXT,
            keluaran TEXT,
            target_keluaran TEXT,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (kegiatan_id) REFERENCES kegiatan(id),
            FOREIGN KEY (sumber_dana_id) REFERENCES sumber_dana(id)
        );

        -- Sumber Dana
        -- Referensi sumber pendanaan per pengelompokan belanja
        -- Siap untuk fitur alokasi sumber dana ke belanja
        CREATE TABLE IF NOT EXISTS sumber_dana (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun_anggaran_id INTEGER NOT NULL,
            kode TEXT NOT NULL,
            nama TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id)
        );

        -- Rincian Rekening
        -- Kode rekening yang dipakai dalam satu sub kegiatan
        CREATE TABLE IF NOT EXISTS rincian_rekening (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            sub_kegiatan_id INTEGER NOT NULL,
            rekening_id INTEGER NOT NULL,
            urutan INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (sub_kegiatan_id) REFERENCES sub_kegiatan(id),
            FOREIGN KEY (rekening_id) REFERENCES rekening(id)
        );

        -- Rincian Pengelompokan [ # ]
        -- Pengelompokan belanja atau pemaketan kerja dalam satu rekening
        -- jenis: pengelompokan (default) / pemaketan
        CREATE TABLE IF NOT EXISTS rincian_pengelompokan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rincian_rekening_id INTEGER NOT NULL,
            label TEXT NOT NULL,
            jenis TEXT NOT NULL DEFAULT 'pengelompokan' CHECK(jenis IN ('pengelompokan', 'pemaketan')),
            sumber_dana_id INTEGER,
            urutan INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (rincian_rekening_id) REFERENCES rincian_rekening(id),
            FOREIGN KEY (sumber_dana_id) REFERENCES sumber_dana(id)
        );

        -- Rincian Keterangan [ - ]
        -- Sub-grup dalam satu pengelompokan
        CREATE TABLE IF NOT EXISTS rincian_keterangan (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            rincian_pengelompokan_id INTEGER NOT NULL,
            label TEXT NOT NULL,
            urutan INTEGER NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (rincian_pengelompokan_id) REFERENCES rincian_pengelompokan(id)
        );

        -- Rincian Item
        -- Item belanja paling detail
        -- dikunci: 1 jika sudah terealisasi
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

        -- Jejak Rekam (Event Log)
        -- Append-only — tidak boleh dihapus
        -- tingkat: info / warning / critical (untuk benturan realisasi)
        CREATE TABLE IF NOT EXISTS jejak_rekam (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL,
            entitas TEXT NOT NULL CHECK(entitas IN (
                'rincian',
                'kegiatan',
                'sub_kegiatan',
                'dokumen'
            )),
            entitas_id INTEGER NOT NULL,
            aksi TEXT NOT NULL CHECK(aksi IN (
                'tambah',
                'ubah',
                'hapus',
                'kunci',
                'tetapkan',
                'arsipkan'
            )),
            nilai_lama TEXT,
            nilai_baru TEXT,
            pesan TEXT,
            tingkat TEXT NOT NULL DEFAULT 'info' CHECK(tingkat IN (
                'info',
                'warning',
                'critical'
            )),
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id)
        );

        -- Rencana Sementara (Stash)
        -- Simulasi potong/tambah/geser yang belum resmi direkam
        -- Terisolasi dari dokumen aktif
        CREATE TABLE IF NOT EXISTS rencana_sementara (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dokumen_id INTEGER NOT NULL,
            label TEXT,
            data_json TEXT NOT NULL,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (dokumen_id) REFERENCES dokumen(id)
        );

        -- SHS (Standar Harga Satuan)
        -- jenis: SSH / SBU / HSPK / ASB
        CREATE TABLE IF NOT EXISTS shs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            tahun_anggaran_id INTEGER NOT NULL,
            jenis TEXT NOT NULL DEFAULT 'SSH' CHECK(jenis IN ('SSH','SBU','HSPK','ASB')),
            kode TEXT NOT NULL,
            uraian TEXT NOT NULL,
            satuan TEXT NOT NULL,
            harga REAL NOT NULL DEFAULT 0,
            created_at TEXT DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (tahun_anggaran_id) REFERENCES tahun_anggaran(id)
        );

        -- Kode Rekening
        CREATE TABLE IF NOT EXISTS rekening (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            kode TEXT NOT NULL UNIQUE,
            uraian TEXT NOT NULL,
            level INTEGER NOT NULL
        );

        -- Catat versi schema pertama
        INSERT OR IGNORE INTO migrasi (versi, deskripsi)
        VALUES (3, 'Tambah sumber_dana, rincian_rekening, rincian_pengelompokan, rincian_keterangan, rincian_item');
        INSERT OR IGNORE INTO migrasi (versi, deskripsi)
        VALUES (2, 'Rename ssh to shs, tambah jenis SHS, tambah asal_keterangan dokumen');
        INSERT OR IGNORE INTO migrasi (versi, deskripsi)
        VALUES (1, 'Initial schema — tahun_anggaran, dokumen, kegiatan, sub_kegiatan, rincian, jejak_rekam, rencana_sementara, ssh, rekening');
    "#)?;

    Ok(())
}
