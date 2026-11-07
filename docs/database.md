# Database Aerarium

## Konsep Inti

Aerarium mengadopsi konsep version control (Git) dan accounting systems
untuk pengelolaan dokumen anggaran OPD. Pendekatan hybrid digunakan:
tabel utama menyimpan kondisi terkini, jejak_rekam menyimpan peristiwa
penting secara append-only, snapshot menyimpan kondisi dokumen saat
ditetapkan.

## Padanan Istilah

| Konsep Git | Istilah Aerarium | Keterangan |
|---|---|---|
| commit | rekam | Simpan perubahan dengan pesan |
| branch | tahap | RKA-SKPD, DPA-SKPD, DPPA-SKPD |
| merge | tetapkan | Dokumen disahkan/final |
| tag/release | dokumen final | Salinan saat dokumen dikunci |
| diff | perbandingan | Selisih antar versi dokumen |
| log | riwayat | jejak_rekam — audit trail |
| snapshot | salinan final | State dokumen saat ditetapkan |
| conflict | benturan | Benturan rencana vs realisasi |
| lock | kunci | Bagian yang sudah ada realisasi |
| stash | rencana sementara | Simulasi belum resmi |
| parent | asal | DPA-SKPD berasal dari RKA-SKPD |

## Prinsip Desain

1. **Jejak Rekam (Event Sourcing Hybrid)** — tabel utama menyimpan
   kondisi terkini, jejak_rekam append-only untuk audit trail.

2. **Catatan Permanen (Immutable Records)** — bagian yang sudah
   terealisasi tidak bisa diubah. Bagian belum terealisasi tetap
   bisa disesuaikan melalui mekanisme perubahan yang sah.

3. **Salinan Final (Snapshot)** — saat dokumen ditetapkan, seluruh
   state disimpan permanen.

4. **Salinan Penuh (Full Copy)** — saat RKA-SKPD baru dibuat,
   seluruh rincian disalin dari versi sebelumnya.

5. **Migrasi Schema** — database harus bisa diupgrade tanpa
   kehilangan data. Versi schema dicatat di tabel migrasi.

## State Machine Dokumen

draft → konsep → ditetapkan → dikunci → diarsipkan


## Siklus Dokumen

SE v1 → RKA-SKPD v1 (konsep) ──┐
SE v2 → RKA-SKPD v2 (konsep) ──┤ versi lama otomatis dikunci
SE v3 → RKA-SKPD v3 (konsep) ──┘
↓ ditetapkan
DPA-SKPD (asal: RKA-SKPD v3)
↓ dikunci
SE Perubahan v1 → RKA-SKPD Perubahan v1 ──┐
SE Perubahan v2 → RKA-SKPD Perubahan v2 ──┘
↓ ditetapkan
DPPA-SKPD (asal: RKA-SKPD Perubahan v2)


### Aturan Siklus
- Setiap RKA-SKPD baru dipicu oleh SE resmi
- Saat vN dibuat, v(N-1) otomatis dikunci
- Hanya versi terakhir yang bisa ditetapkan
- Realisasi tidak mengunci seluruh kegiatan, hanya bagian terealisasi

## Lokasi Database

- **Linux:** ~/.local/share/aerarium/aerarium.db
- **Windows:** C:/Users/<user>/AppData/Roaming/aerarium/aerarium.db
- **macOS:** ~/Library/Application Support/aerarium/aerarium.db

## Schema

### migrasi
Mencatat versi schema database untuk keperluan upgrade.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| versi | INTEGER | Nomor versi schema |
| deskripsi | TEXT | Deskripsi perubahan |
| diterapkan_at | TEXT | Waktu migrasi |

### tahun_anggaran

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| tahun | INTEGER | Tahun anggaran (unik) |
| aktif | INTEGER | 1 = aktif |
| created_at | TEXT | Waktu dibuat |

### dokumen
Setiap tahap anggaran adalah satu dokumen. RKA-SKPD dan
RKA-SKPD Perubahan bisa banyak versi, masing-masing dipicu SE resmi.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| tahun_anggaran_id | INTEGER | FK tahun_anggaran |
| jenis | TEXT | RKA-SKPD / DPA-SKPD / RKA-SKPD Perubahan / DPPA-SKPD |
| versi | INTEGER | Urutan SE (1, 2, 3...) |
| nomor_se | TEXT | Nomor SE pemicu |
| tanggal_se | TEXT | Tanggal SE |
| status | TEXT | draft / konsep / ditetapkan / dikunci / diarsipkan |
| asal_id | INTEGER | FK dokumen asal (nullable) |
| catatan | TEXT | Keterangan |
| ditetapkan_at | TEXT | Waktu ditetapkan |
| created_at | TEXT | Waktu dibuat |

### salinan_final
Salinan lengkap state dokumen saat ditetapkan — tidak bisa diubah.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| dokumen_id | INTEGER | FK dokumen (unik) |
| label | TEXT | Misal: DPA-SKPD 2025 Final |
| catatan | TEXT | Keterangan tambahan |
| data_json | TEXT | Seluruh state dokumen (JSON) |
| created_at | TEXT | Waktu dibuat |

### kegiatan

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| dokumen_id | INTEGER | FK dokumen |
| kode | TEXT | Kode kegiatan |
| uraian | TEXT | Nama kegiatan |
| pagu | REAL | Pagu anggaran |
| created_at | TEXT | Waktu dibuat |

### sub_kegiatan

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| kegiatan_id | INTEGER | FK kegiatan |
| kode | TEXT | Kode sub kegiatan |
| uraian | TEXT | Nama sub kegiatan |
| pagu | REAL | Pagu sub kegiatan |
| created_at | TEXT | Waktu dibuat |

### rincian

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| sub_kegiatan_id | INTEGER | FK sub kegiatan |
| ssh_id | INTEGER | FK SSH (nullable) |
| rekening_id | INTEGER | FK rekening |
| uraian | TEXT | Nama item belanja |
| volume | REAL | Volume/jumlah |
| satuan | TEXT | Satuan |
| harga | REAL | Harga satuan |
| realisasi | REAL | Jumlah yang sudah direalisasi |
| dikunci | INTEGER | 1 = bagian terealisasi, tidak bisa diubah |
| created_at | TEXT | Waktu dibuat |

### jejak_rekam
Setiap perubahan penting tercatat — append-only, tidak bisa dihapus.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| dokumen_id | INTEGER | FK dokumen |
| entitas | TEXT | rincian / kegiatan / sub_kegiatan / dokumen |
| entitas_id | INTEGER | ID entitas yang berubah |
| aksi | TEXT | tambah / ubah / hapus / kunci / tetapkan |
| nilai_lama | TEXT | State sebelum (JSON) |
| nilai_baru | TEXT | State sesudah (JSON) |
| pesan | TEXT | Pesan rekaman |
| tingkat | TEXT | info / warning / critical (untuk benturan) |
| created_at | TEXT | Waktu rekaman |

### rencana_sementara
Simulasi potong/tambah/geser yang belum resmi direkam ke dokumen.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| dokumen_id | INTEGER | FK dokumen aktif |
| label | TEXT | Nama simulasi |
| data_json | TEXT | Seluruh simulasi (JSON) |
| created_at | TEXT | Waktu dibuat |

### ssh
Database harga SSH/SHBJ per tahun anggaran.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| tahun_anggaran_id | INTEGER | FK tahun_anggaran |
| kode | TEXT | Kode item |
| uraian | TEXT | Nama item |
| satuan | TEXT | Satuan |
| harga | REAL | Harga satuan |
| created_at | TEXT | Waktu dibuat |

### rekening

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| kode | TEXT | Kode rekening (unik) |
| uraian | TEXT | Uraian rekening |
| level | INTEGER | Level hierarki |

## Fitur Perbandingan (Diff)
Tidak memerlukan tabel tambahan — dihitung on-the-fly karena
setiap dokumen menyimpan salinan penuh rinciannya.

Skenario perbandingan:
- RKA-SKPD v1 vs v2 — perubahan antar SE
- RKA-SKPD v3 vs DPA-SKPD — kesesuaian DPA dengan RKA final
- DPA-SKPD vs RKA-SKPD Perubahan — perubahan di siklus perubahan
- RKA-SKPD Perubahan vs DPPA-SKPD — kesesuaian DPPA dengan RKA final

## Relasi

tahun_anggaran (1) ──── (N) dokumen
tahun_anggaran (1) ──── (N) ssh
dokumen (1) ──── (N) kegiatan
dokumen (1) ──── (1) salinan_final
dokumen (1) ──── (N) jejak_rekam
dokumen (1) ──── (N) rencana_sementara
dokumen (0/1) ── asal ── (N) dokumen
kegiatan (1) ──── (N) sub_kegiatan
sub_kegiatan (1) ──── (N) rincian
rincian (0/1) ──── ssh
rincian (1) ──── rekening