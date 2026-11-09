# Checkpoint Sesi

## Status Branch Aktif
Branch: feat/rincian-belanja
Belum di-merge ke dev — perlu diselesaikan di sesi berikutnya.

## Yang Sudah Selesai di Branch Ini
- Schema: sumber_dana, rincian_rekening, rincian_pengelompokan, rincian_keterangan, rincian_item
- Backend: models + commands untuk kegiatan, sub_kegiatan, rincian (semua level)
- UI pre-refactor: halaman dokumen detail, kegiatan, sub kegiatan, rincian belanja
- Command cari_atau_buat_rekening

## PRIORITAS SESI BERIKUTNYA: Refactor ke Master Data

### Alasan Refactor
Kegiatan dan sub kegiatan adalah master data — tidak berubah per dokumen.
Pagu baru ada saat input di dokumen untuk tahun anggaran tertentu.

### Skema Baru yang Harus Dibuat
Hapus tabel lama:
- kegiatan (lama)
- sub_kegiatan (lama)

Buat tabel master baru:
- master_program (id, tahun_anggaran_id, kode, uraian, aktif)
- master_kegiatan (id, program_id, kode, uraian, aktif)
- master_sub_kegiatan (id, kegiatan_id, kode, uraian, aktif)

Buat tabel penghubung:
- dokumen_sub_kegiatan (id, dokumen_id, master_sub_kegiatan_id, pagu, sumber_dana_id, lokasi, waktu_pelaksanaan, keluaran, target_keluaran)

Update FK:
- rincian_rekening.sub_kegiatan_id → rincian_rekening.dokumen_sub_kegiatan_id

### Alur Kerja User Setelah Refactor
1. Setup master — input program/kegiatan/sub kegiatan dari Renstra/Renja, set aktif/nonaktif
2. Buat dokumen — pilih sub kegiatan aktif, set pagu
3. Input rincian belanja per sub kegiatan

### Yang Perlu Diupdate Setelah Schema
- models/kegiatan.rs — refactor ke master_program, master_kegiatan, master_sub_kegiatan
- commands/kegiatan.rs — idem
- models/rincian.rs — tambah DokumenSubKegiatan
- commands/rincian.rs — tambah dokumen_sub_kegiatan commands
- lib.rs — update semua command
- UI — halaman master data + alur dokumen baru

## Cara Lanjut di Sesi Baru
1. Buka Project AERARIUM di Claude
2. Paste isi docs/checkpoint.md ini
3. Mulai dari refactor schema di db/schema.rs
