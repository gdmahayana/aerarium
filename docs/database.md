# Database Aerarium

## Lokasi

Database disimpan di folder data user:
- **Linux:** `~/.local/share/aerarium/aerarium.db`
- **Windows:** `C:\Users\<user>\AppData\Roaming\aerarium\aerarium.db`
- **macOS:** `~/Library/Application Support/aerarium/aerarium.db`

## Schema

### tahun_anggaran
Menyimpan daftar tahun anggaran yang dikelola.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| tahun | INTEGER | Tahun anggaran (unik) |
| aktif | INTEGER | 1 = aktif, 0 = tidak aktif |
| created_at | TEXT | Waktu dibuat |

### ssh
Menyimpan database harga SSH/SHBJ per tahun anggaran.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| tahun_anggaran_id | INTEGER | FK ke tahun_anggaran |
| kode | TEXT | Kode item SSH |
| uraian | TEXT | Nama/deskripsi item |
| satuan | TEXT | Satuan (unit, paket, dll) |
| harga | REAL | Harga satuan |
| created_at | TEXT | Waktu dibuat |

### rekening
Menyimpan daftar kode rekening belanja.

| Kolom | Tipe | Keterangan |
|---|---|---|
| id | INTEGER | Primary key |
| kode | TEXT | Kode rekening (unik) |
| uraian | TEXT | Uraian rekening |
| level | INTEGER | Level hierarki rekening |

## Relasi
tahun_anggaran (1) ──── (N) ssh


## Catatan

- Foreign keys diaktifkan via `PRAGMA foreign_keys = ON`
- Database diinisialisasi otomatis saat app pertama kali dijalankan