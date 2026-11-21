# Aerarium

> Plan with Confidence. Change with Control.

Aplikasi desktop open source untuk perencanaan dan pengelolaan anggaran OPD/SKPD di lingkungan pemerintah daerah Indonesia.

Aerarium bukan pengganti SIPD. Aerarium adalah **ruang kerja pendukung** — membantu perencana mengelola kompleksitas pekerjaan di sekitar proses formal penganggaran: menyusun rincian belanja, mensimulasikan perubahan pagu, dan menghasilkan dokumen referensi yang siap diinput ke SIPD.

---

## Fitur Utama (v0.4.x)

### Database SHS
- Import SSH/SBU/HSPK/ASB dari file Excel SIPD (format 9 kolom)
- Dua sumber: **SHS Rencana** (estimasi OPD) dan **SHS Resmi** (harga ditetapkan BPKAD)
- Multi-rekening per item — tersimpan di tabel relasi `shs_rekening`
- Export ke Excel — format identik dengan import SIPD, siap untuk dikirim balik
- Search server-side dengan debounce — performa optimal untuk 5000+ item
- Input manual dengan validasi

### Manajemen Dokumen
- Siklus dokumen lengkap: **KK → RKA → DPA → DPPA**
- Status dokumen dengan state machine: draft → konsep → ditetapkan → dikunci
- Alur dokumen visual — lihat relasi antar versi
- Pagu indikatif per sub kegiatan

### Rincian Belanja
- Susun rincian per rekening, pengelompokan, keterangan, dan item
- Validasi SHS — item belanja terkait ke SHS Resmi
- Kalkulasi otomatis: volume × koef × harga × (1 + PPN)
- Pemetaan SHS Rencana → SHS Resmi saat harga resmi terbit
- Realisasi per item

### Export
- **Kertas Kerja** — Excel multi-sheet dengan rekapitulasi per sub kegiatan
- **SHS Rencana** — Excel untuk dikirim ke BPKAD sebagai usulan survei harga
- **SHS Resmi** — Excel format SIPD, siap reimport

### Master Data
- Master Program / Kegiatan / Sub Kegiatan
- Rekening (kode akun belanja, basis BAS Permendagri 90/2019)
- Kode Kelompok Barang (kode BMD `1.x` dan kode Beban `8.x` dari SIPD)
- Sumber dana per tahun anggaran

---

## Stack Teknologi

| Layer | Teknologi | Keterangan |
|---|---|---|
| Desktop Framework | Tauri v2 | Cross-platform, ringan, ~10MB installer |
| Frontend | Svelte 5 + TypeScript | UI reaktif, Svelte runes |
| Backend | Rust | Logika bisnis, keamanan memory |
| Database | SQLite (rusqlite bundled) | Lokal, tidak perlu install server |
| Excel | rust_xlsxwriter + calamine | Import dan export .xlsx |
| Build Tool | Vite + pnpm | Modern, cepat |

---

## Memulai

### Prasyarat

- [Rust](https://rustup.rs/) (via rustup, stable)
- [Node.js](https://nodejs.org/) >= 18
- [pnpm](https://pnpm.io/) >= 8
- Dependensi sistem Tauri — lihat [Prerequisites Tauri](https://tauri.app/start/prerequisites/)

### Development

```bash
git clone https://github.com/gdmahayana/aerarium.git
cd aerarium
pnpm install
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

Output installer ada di `src-tauri/target/release/bundle/`.

---

## Struktur Proyek

aerarium/
├── src/ # Frontend Svelte
│ ├── routes/ # Halaman aplikasi
│ │ ├── +layout.svelte # Layout utama + sidebar
│ │ ├── +page.svelte # Dashboard
│ │ ├── dokumen/ # Manajemen dokumen anggaran
│ │ ├── ssh/ # Database SHS
│ │ ├── kode-barang/ # Kode kelompok barang BMD
│ │ ├── master-data/ # Program/Kegiatan/Sub Kegiatan
│ │ └── tahun-anggaran/ # Manajemen tahun anggaran
│ └── lib/
│ ├── components/ # Toast, ConfirmDialog, dll
│ └── stores/ # Global state (toast, dll)
├── src-tauri/ # Backend Rust
│ └── src/
│ ├── commands/ # Tauri commands (bisnis logic)
│ │ ├── export.rs # Export Excel
│ │ ├── import_ssh.rs # Import SHS dari SIPD
│ │ ├── dokumen.rs # Manajemen dokumen
│ │ ├── kegiatan.rs # Program/Kegiatan/Sub Kegiatan
│ │ ├── rincian.rs # Rincian belanja
│ │ └── pagu_indikatif.rs
│ ├── db/
│ │ └── schema.rs # Migrasi database (v1-v9+)
│ ├── models/ # Struct data
│ └── lib.rs # Entry point + command registry
├── docs/ # Dokumentasi teknis
│ ├── architecture.md # Filosofi dan desain sistem
│ ├── database.md # Skema database
│ ├── features.md # Detail fitur
│ └── workflow.md # Alur kerja pengguna
└── static/


---

## Dokumentasi

| Dokumen | Isi |
|---|---|
| [Architecture](docs/architecture.md) | Filosofi, prinsip desain, state machine dokumen |
| [Database](docs/database.md) | Skema tabel, relasi, strategi migrasi |
| [Features](docs/features.md) | Detail implementasi setiap fitur |
| [Workflow](docs/workflow.md) | Alur kerja perencana OPD |

---

## Berkontribusi

Aerarium adalah proyek open source di bawah AGPL-3.0. Kontribusi disambut baik.

### Cara Berkontribusi

1. Fork repository
2. Buat branch fitur: `git checkout -b feat/nama-fitur`
3. Commit perubahan: `git commit -m "feat: deskripsi singkat"`
4. Push dan buat Pull Request ke branch `dev`

### Konvensi Commit

Gunakan format [Conventional Commits](https://www.conventionalcommits.org/):

feat: tambah export SHS Resmi ke Excel
fix: duplikat kolom kode rekening di tabel SHS
refactor: pisah query search ke server-side
docs: update README dengan fitur v0.4


### Hal yang Perlu Diperhatikan

- Logika bisnis **harus** di backend Rust, bukan frontend Svelte
- Setiap perubahan schema database **wajib** lewat migrasi baru (tambah versi di `schema.rs`)
- Command Tauri baru **wajib** didaftarkan di `invoke_handler` di `lib.rs`
- Gunakan `toast.success()` / `toast.error()` untuk feedback ke pengguna — bukan `alert()`

---

## Roadmap

### v0.4.x — Penguatan Export & Master Data *(sekarang)*
- [x] Export SHS Resmi ke Excel (format SIPD)
- [x] Multi-rekening per item SHS
- [x] Halaman Kode Kelompok Barang
- [ ] Export PDF kertas kerja
- [ ] README dan dokumentasi kontributor

### v0.5.0 — Simulasi & Perbandingan
- [ ] Simulasi perubahan pagu (potong/tambah/geser)
- [ ] Perbandingan antar versi dokumen
- [ ] Budget Impact Analyzer

### v1.0.0 — Produksi
- [ ] Backup dan restore database
- [ ] Import reconciliation engine
- [ ] Pengujian integritas data

---

## Lisensi

[GNU Affero General Public License v3.0](LICENSE)

Singkatnya: bebas digunakan, dipelajari, dimodifikasi, dan didistribusikan — termasuk untuk deployment server — selama kode tetap terbuka dengan lisensi yang sama.

---

*Dibuat untuk operator OPD yang bekerja keras setiap hari menyusun anggaran.*
'''
open('README.md', 'w').write(content)
print('done:', len(content), 'chars')