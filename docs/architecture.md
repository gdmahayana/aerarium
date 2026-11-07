# Aerarium

> Plan with Confidence. Change with Control.

## Tentang

Aerarium adalah aplikasi desktop open source untuk membantu perencana
dan pengelola anggaran OPD di Indonesia mengelola siklus hidup dokumen
anggaran.

Aerarium bukan pengganti SIPD. Aerarium adalah ruang kerja pendukung
yang membantu perencana mengelola kompleksitas pekerjaan di sekitar
proses formal penganggaran.

## Filosofi

**Support, Not Replace**
Mendukung proses formal penganggaran, bukan menggantikannya.
Tetap berguna meski SIPD mengalami kendala.

**Offline-First, Desktop-First**
Tidak membutuhkan koneksi internet untuk fungsi inti.
Data utama tersimpan lokal, pengguna punya kendali penuh.

**Data Integrity**
Data yang sudah memiliki konsekuensi tidak boleh diubah sembarangan.
Setiap perubahan mempertahankan hubungan dengan kondisi sebelumnya.

**Traceability**
Aerarium harus mampu menjelaskan perjalanan sebuah dokumen:
dari mana asalnya, apa yang berubah, kapan, dan mengapa.

**Open Source by Design**
Kode terbuka untuk diperiksa, dipelajari, dikembangkan, dan
disesuaikan oleh komunitas sesuai ketentuan lisensi.

## Stack

| Layer | Teknologi | Keterangan |
|---|---|---|
| Desktop Framework | Tauri v2 | Cross-platform, ringan |
| Frontend | Svelte + TypeScript | UI reaktif, performa tinggi |
| Backend | Rust | Logika bisnis, keamanan |
| Database | SQLite (rusqlite bundled) | Lokal, tidak perlu install |
| Build Tool | Vite + pnpm | Modern, cepat |

## Adaptasi Konsep Git ke Penganggaran

| Konsep Git | Istilah Aerarium | Implementasi |
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

**1. Jejak Rekam (Event Sourcing Hybrid)**
Tabel utama menyimpan kondisi terkini. Tabel jejak_rekam menyimpan
setiap perubahan penting secara append-only. Snapshot menyimpan
kondisi dokumen pada tahap tertentu.

**2. Catatan Permanen (Immutable Records)**
Bagian yang sudah ada realisasi tidak bisa diubah. Namun bagian
yang belum terealisasi tetap bisa disesuaikan melalui mekanisme
perubahan yang sah.

**3. Salinan Final (Snapshot)**
Saat dokumen ditetapkan, seluruh state disimpan sebagai salinan
permanen yang tidak bisa diubah.

**4. Salinan Penuh (Full Copy)**
Saat RKA-SKPD baru dibuat, seluruh rincian disalin dari versi
sebelumnya. Setiap dokumen berdiri sendiri sebagai audit trail.

**5. Realization Conflict Engine**
Tiga level deteksi benturan:
- Critical: pagu di bawah realisasi yang harus ditanggung
- Warning: perubahan pada komponen yang sudah terealisasi
- Information: konsekuensi perubahan yang perlu diketahui

**6. Budget Impact Analyzer**
Gambaran dampak perubahan sebelum direkam ke dokumen resmi:
selisih pagu, sisa alokasi, perubahan rekening, potensi benturan.

**7. Rencana Sementara**
Simulasi potong/tambah/geser yang belum resmi direkam ke dokumen.
Perencana bisa eksplorasi bebas sebelum memutuskan.

**8. Migrasi Schema**
Database harus bisa diupgrade tanpa kehilangan data.

## State Machine Dokumen

draft → konsep → ditetapkan → dikunci → diarsipkan


Status internal Aerarium tidak sama dengan status penetapan resmi
pemerintah.

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
- Setiap RKA-SKPD/RKA-SKPD Perubahan baru dipicu oleh SE resmi
- Saat vN dibuat, v(N-1) otomatis dikunci
- Hanya versi terakhir yang bisa ditetapkan
- Realisasi tidak mengunci seluruh kegiatan, hanya bagian terealisasi

## Arsitektur Berlapis

Presentation Layer → UI, dashboard, editor, simulasi
Application Layer → commands, use case, orkestrasi
Domain Layer → aturan penganggaran, versioning, validasi
Persistence Layer → SQLite, repository, migrasi
Audit Layer → jejak_rekam, append-only
Document Layer → import, ekspor, parsing


Logika bisnis penting dijalankan di backend Rust, bukan frontend.

## Peran Pengguna

| Peran | Akses | Keterangan |
|---|---|---|
| Perencana | Penuh | Satu-satunya yang edit dan putuskan |
| Kontributor | Lihat + komentar | Via web app (fitur v1.x) |

## Struktur Folder

aerarium/
├── src/ # Frontend Svelte
│ ├── routes/ # Halaman-halaman app
│ ├── lib/
│ │ ├── components/ # Komponen UI reusable
│ │ ├── stores/ # State management Svelte
│ │ └── utils/ # Helper functions
│ └── app.html
├── src-tauri/ # Backend Rust
│ ├── src/
│ │ ├── commands/ # Tauri commands
│ │ │ ├── mod.rs
│ │ │ └── import_ssh.rs
│ │ ├── db/ # Database
│ │ │ ├── mod.rs
│ │ │ └── schema.rs
│ │ ├── models/ # Struktur data
│ │ ├── lib.rs
│ │ └── main.rs
│ └── Cargo.toml
├── docs/
│ ├── architecture.md
│ ├── database.md
│ ├── features.md
│ └── workflow.md
└── static/


## Identitas

| | |
|---|---|
| **Nama** | Aerarium |
| **Tagline** | Plan with Confidence. Change with Control. |
| **Kategori** | Open Source Budget Management Workspace |
| **Identifier** | com.gdmahayana.aerarium |
| **Lisensi** | GNU Affero General Public License v3.0 |
| **Repository** | https://github.com/gdmahayana/aerarium |
| **Target** | OPD/SKPD pemerintah daerah Indonesia |

## Roadmap

### v0.1.0 — Fondasi
- Setup database + migrasi schema
- Manajemen tahun anggaran dan dokumen
- Import SSH/SHBJ dari Excel SIPD
- Penyusunan rincian belanja

### v0.2.0 — Versioning dan Simulasi
- Jejak rekam dan riwayat
- Rencana sementara (simulasi)
- Perbandingan antar dokumen
- Realization Conflict Engine
- Budget Impact Analyzer
- Export Excel + PDF

### v0.3.0 — Penguatan
- Backup dan restore
- Import reconciliation engine
- Pengujian integritas database

### v1.0.0 — Self-Hosting
- Server OPD opsional
- Web app kontributor (lihat + komentar)
- Fitur kirim/tarik dokumen