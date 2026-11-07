# Fitur Aerarium

## Status

Legend:
- ✅ Selesai
- 🚧 Dalam pengerjaan
- 📋 Direncanakan

## Fitur Inti

### 1. Manajemen Tahun Anggaran
Status: 📋

- Tambah tahun anggaran baru
- Set tahun anggaran aktif
- Arsip tahun anggaran lama

### 2. Import SSH/SHBJ
Status: 📋

- Import dari file Excel SIPD
- Validasi format file
- Update database harga per tahun
- Lihat dan cari data SSH

### 3. Susun Rincian Belanja
Status: 📋

- Buat kegiatan/sub-kegiatan
- Pilih item dari database SSH
- Input volume dan harga
- Mapping otomatis ke kode rekening
- Hitung total per rekening

### 4. Simulasi Perubahan Pagu
Status: 📋

- Input pagu per kegiatan
- Simulasi pemotongan/penambahan pagu
- Adjustment rincian otomatis
- Riwayat perubahan pagu

### 5. Export Dokumen
Status: 📋

- Export ke Excel — rincian belanja terstruktur per rekening
- Export ke PDF — dokumen referensi input SIPD

## Fitur Lanjutan

### Validasi Kesesuaian Barang ↔ Rekening
Status: 📋

- Cek kesesuaian item SSH dengan kode rekening
- Peringatan jika mapping tidak sesuai
- Saran rekening yang tepat

## Roadmap

### v0.1.0 — MVP
- Setup database
- Import SSH/SHBJ dari Excel
- Susun rincian belanja sederhana
- Export Excel

### v0.2.0
- Simulasi perubahan pagu
- Export PDF
- Manajemen tahun anggaran

### v0.3.0
- Validasi kesesuaian barang ↔ rekening
- Riwayat perubahan