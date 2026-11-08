# Checkpoint Sesi

## Status Branch Aktif
Branch: dev
Branch feat/perbaikan-dokumen-shs sudah di-merge ke dev.

## Yang Sudah Selesai
- Rename tabel ssh → shs, tambah kolom jenis (SSH/SBU/HSPK/ASB)
- Tambah field asal_keterangan dan asal_id di tabel dokumen dan model
- Validasi logika asal dokumen (DPA dari RKA, DPPA dari RKA Perubahan)
- Update UI form buat dokumen dengan pilihan asal Aerarium/SIPD
- Update UI halaman SSH → SHS dengan filter per jenis
- Tambah command get_shs di backend
- Fix syntax error SQL di schema migrasi
- Merge feat/perbaikan-dokumen-shs → dev

## Fitur Berikutnya (Prioritas)

### 1. Penyusunan Rincian Belanja
- Struktur: Kegiatan → Sub Kegiatan → Rincian
- Rincian merujuk ke SHS (ssh_id) dan Rekening (rekening_id)
- Field rincian: uraian, volume, satuan, harga, realisasi, dikunci

### 2. Jejak Rekam Otomatis
- Catat setiap perubahan rincian ke tabel jejak_rekam
- Append-only — tidak boleh dihapus

### 3. Export Excel

## Cara Lanjut di Sesi Baru
1. Buka Project AERARIUM di Claude
2. Paste isi file docs/checkpoint.md ini
3. Lanjut dari fitur berikutnya
