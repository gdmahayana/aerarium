# Checkpoint Sesi

## Status Branch Aktif
Branch: feat/perbaikan-dokumen-shs
Belum di-merge ke dev — perlu diselesaikan di sesi berikutnya.

## Yang Sudah Selesai di Branch Ini
- Rename tabel ssh → shs, tambah kolom jenis (SSH/SBU/HSPK/ASB)
- Tambah field asal_keterangan di tabel dokumen dan model
- Update import command: import_shs_from_excel
- Build berhasil — hanya warnings

## Yang Perlu Diselesaikan di Sesi Berikutnya

### 1. Logika dokumen (PRIORITAS)
- DPA-SKPD harus lahir dari RKA-SKPD (via asal_id)
- DPPA-SKPD harus lahir dari RKA-SKPD Perubahan
- Dokumen perubahan bisa dibuat mandiri (asal di SIPD) via asal_keterangan
- Update UI form buat dokumen — pilihan asal di Aerarium atau di SIPD

### 2. Update UI halaman SSH → SHS
- Rename label SSH → SHS
- Tambah filter per jenis (SSH/SBU/HSPK/ASB)
- Update command invoke: import_ssh → import_shs dengan parameter jenis

### 3. Merge ke dev setelah selesai

### 4. Fitur berikutnya setelah merge
- Penyusunan rincian belanja (kegiatan, sub kegiatan, rincian)
- Jejak rekam otomatis
- Export Excel

## Cara Lanjut di Sesi Baru
1. Buka Project AERARIUM di Claude
2. Paste isi file docs/checkpoint.md ini
3. Paste isi docs/architecture.md dan docs/database.md jika perlu konteks lebih
4. Lanjut dari langkah yang belum selesai
