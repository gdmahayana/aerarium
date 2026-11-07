# Fitur Aerarium

> Plan with Confidence. Change with Control.

## Status

- ✅ Selesai
- 🚧 Dalam pengerjaan
- 📋 Direncanakan
- 🔮 Roadmap jauh

## Modul Inti

### 1. Manajemen Tahun Anggaran dan Dokumen
Status: 🚧

- Tambah dan kelola tahun anggaran
- Buat dokumen RKA-SKPD dipicu oleh SE resmi
- State machine dokumen: draft → konsep → ditetapkan → dikunci → diarsipkan
- Salin seluruh rincian saat versi baru dibuat
- Kunci versi lama saat versi baru dibuat
- Hanya versi terakhir yang bisa ditetapkan menjadi DPA-SKPD

### 2. Import SSH/SHBJ
Status: 🚧

- Import dari file Excel SIPD
- Validasi format dan struktur file
- Update database harga per tahun anggaran
- Cari dan filter data SSH

### 3. Penyusunan Rincian Belanja
Status: 📋

- Tambah kegiatan dan sub kegiatan
- Pilih item dari database SSH
- Input volume, satuan, harga
- Mapping ke kode rekening
- Kalkulasi otomatis total per rekening dan sub kegiatan
- Kalkulasi otomatis total per kegiatan dan dokumen

### 4. Jejak Rekam dan Riwayat
Status: 📋

- Setiap perubahan tercatat otomatis
- Pesan rekaman saat menyimpan perubahan
- Lihat riwayat lengkap per dokumen
- Filter riwayat per entitas, aksi, waktu
- Append-only — riwayat tidak bisa dihapus

### 5. Rencana Sementara (Simulasi)
Status: 📋

- Buat simulasi potong/tambah/geser anggaran
- Tidak mempengaruhi dokumen resmi
- Simpan beberapa skenario simulasi
- Bandingkan skenario satu sama lain
- Terapkan simulasi ke dokumen resmi jika sudah yakin

### 6. Realization Conflict Engine
Status: 📋

Deteksi benturan antara rencana perubahan dan realisasi:

| Level | Makna | Contoh |
|---|---|---|
| Critical | Melanggar integritas wajib | Pagu di bawah realisasi |
| Warning | Perlu pemeriksaan | Perubahan volume pada komponen terealisasi |
| Information | Konsekuensi perlu diketahui | Penambahan rekening baru |

Setiap benturan menampilkan:
- Objek yang terdampak
- Kondisi sebelum dan sesudah
- Nilai selisih
- Level keparahan
- Rekomendasi tindak lanjut

### 7. Budget Impact Analyzer
Status: 📋

Gambaran dampak perubahan sebelum direkam ke dokumen resmi:
- Selisih pagu per kegiatan dan sub kegiatan
- Sisa alokasi setelah memperhitungkan realisasi
- Rekening yang mengalami perubahan
- Potensi benturan dengan realisasi
- Ringkasan perubahan keseluruhan

### 8. Perbandingan Dokumen (Diff)
Status: 📋

Bandingkan dua versi dokumen secara terstruktur:
- RKA-SKPD v1 vs v2 — perubahan antar SE
- RKA-SKPD vs DPA-SKPD — kesesuaian setelah penetapan
- DPA-SKPD vs RKA-SKPD Perubahan — perubahan di siklus perubahan
- RKA-SKPD Perubahan vs DPPA-SKPD — kesesuaian setelah penetapan

Hasil perbandingan menampilkan:
- Data yang ditambahkan
- Data yang dihapus
- Data yang diubah (nilai lama vs baru)
- Perubahan nominal dan persentase
- Perubahan struktur rekening

### 9. Export Dokumen
Status: 📋

- Export Excel — rincian belanja terstruktur per rekening
- Export PDF — dokumen referensi input SIPD
- Format output memudahkan input manual ke SIPD

## Fitur Lanjutan

### Import Reconciliation Engine
Status: 🔮 v0.3.x

- Bandingkan dokumen kerja Aerarium dengan ekspor SIPD
- Tampilkan perbedaan dan ketidaksesuaian
- Hasil rekonsiliasi untuk diperiksa perencana

### Backup dan Restore
Status: 🔮 v0.3.x

- Backup otomatis dengan jadwal
- Backup sebelum migrasi database
- Restore dari backup
- Pemeriksaan integritas database
- Export arsip lengkap

### Validasi Kesesuaian Barang dan Rekening
Status: 🔮 v0.3.x

- Cek kesesuaian item SSH dengan kode rekening
- Peringatan jika mapping tidak sesuai
- Saran rekening yang tepat

### Web App Kontributor
Status: 🔮 v1.x

- Kontributor lihat progress dokumen
- Tinggalkan komentar dan masukan
- Perencana baca komentar via desktop app
- Tidak ada edit langsung oleh kontributor

### Self-Hosting Server
Status: 🔮 v1.x

- Server OPD opsional
- Kirim dokumen ke server
- Tarik dokumen dari server
- Sinkronisasi antar perangkat

## Roadmap

### v0.1.0 — Fondasi
- Setup database + migrasi schema
- Manajemen tahun anggaran
- Import SSH/SHBJ dari Excel SIPD
- Penyusunan rincian belanja dasar
- Export Excel

### v0.2.0 — Versioning dan Simulasi
- Jejak rekam dan riwayat lengkap
- Rencana sementara (simulasi)
- Perbandingan antar dokumen
- Realization Conflict Engine
- Budget Impact Analyzer
- Export PDF

### v0.3.0 — Penguatan
- Import reconciliation engine
- Backup dan restore
- Validasi kesesuaian barang dan rekening
- Pengujian integritas database

### v1.0.0 — Self-Hosting dan Kolaborasi
- Server OPD opsional
- Web app kontributor
- Kirim dan tarik dokumen