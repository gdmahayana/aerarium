# Workflow Pengembangan Aerarium

## Git Workflow

### Branch Strategy

main → kode stabil, siap release
dev → development aktif
feat/xxx → fitur baru
fix/xxx → bugfix
docs/xxx → dokumentasi
refactor/xxx → refactoring


### Alur Kerja

feat/nama-fitur → dev → main


- Semua development di branch feat/, fix/, docs/, refactor/
- Merge ke dev setelah selesai dan tested
- Merge ke main hanya saat siap release dengan semver tag
- Tidak ada commit langsung ke main

### Commit Convention

Menggunakan Conventional Commits (conventionalcommits.org):

feat: tambah import SSH dari Excel
fix: perbaiki kalkulasi total pagu
docs: update README instalasi
refactor: reorganisasi struktur database
chore: update dependensi Tauri
test: tambah unit test kalkulasi pagu


### Versioning

Menggunakan Semantic Versioning (semver.org):

v0.1.0 → MVP pertama
v0.2.0 → fitur baru signifikan
v0.1.1 → bugfix minor
v1.0.0 → self-hosting dan kolaborasi


## Repository

- **Primary:** https://github.com/gdmahayana/aerarium
- **Mirror:** git.gdmahayana.my.id (Gitea, private, auto-sync dari GitHub)

Gitea sync otomatis dari GitHub — tidak perlu push manual ke dua remote.

## Setup Development

### Prasyarat

- Arch Linux (atau distro lain)
- Rust via rustup (yay -S rustup)
- Node.js via fnm
- pnpm
- Git
- VSCodium

### Dependensi Sistem (Arch Linux)

```bash
sudo pacman -S webkit2gtk-4.1 base-devel curl wget file openssl \
  appmenu-gtk-module gtk3 libappindicator-gtk3 librsvg libvips
```

### Clone dan Setup

```bash
git clone git@github.com:gdmahayana/aerarium.git
cd aerarium
pnpm install
```

### Development

```bash
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

### Cek Backend Saja

```bash
cargo build --manifest-path src-tauri/Cargo.toml
```

## Struktur Dokumentasi

Dokumentasi disimpan di folder docs/ dalam repo:

| File | Isi |
|---|---|
| architecture.md | Filosofi, stack, arsitektur, roadmap |
| database.md | Schema, konsep database, relasi |
| features.md | Fitur, status, roadmap detail |
| workflow.md | Git workflow, setup, konvensi |

Jika konteks hilang, minta file docs/ yang relevan untuk melanjutkan.

## Konvensi Kode

### Rust (Backend)
- Gunakan snake_case untuk nama fungsi dan variabel
- Struct menggunakan PascalCase
- Error handling wajib — tidak boleh unwrap() di production code
- Logika bisnis di domain layer, bukan di commands

### Svelte + TypeScript (Frontend)
- Komponen menggunakan PascalCase
- Store menggunakan camelCase dengan suffix Store
- Semua props harus punya tipe TypeScript
- Tidak ada logika bisnis di frontend — semua lewat Tauri commands

### Database
- Semua query menggunakan prepared statements
- Transaksi untuk operasi yang melibatkan lebih dari satu tabel
- Foreign key selalu aktif (PRAGMA foreign_keys = ON)
- Migrasi schema dicatat di tabel migrasi

## Catatan Penting

- Aerarium bukan pengganti SIPD
- Status dokumen internal Aerarium tidak sama dengan status penetapan resmi
- Logika bisnis penting dijalankan di backend Rust, bukan frontend
- Realisasi tidak boleh dihapus karena perubahan pagu
- Bagian yang belum terealisasi tetap bisa disesuaikan