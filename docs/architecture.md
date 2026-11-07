# Arsitektur Aerarium

## Stack

- **Frontend:** Svelte + TypeScript
- **Backend:** Rust (Tauri v2)
- **Database:** SQLite (via rusqlite, bundled)
- **Build Tool:** Vite + pnpm
- **Desktop Framework:** Tauri v2

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
│ │ ├── commands/ # Tauri commands (dipanggil frontend)
│ │ ├── db/ # Koneksi dan query database
│ │ │ ├── mod.rs # Inisialisasi database
│ │ │ └── schema.rs # Schema tabel
│ │ ├── models/ # Struktur data
│ │ ├── lib.rs # Entry point library
│ │ └── main.rs # Entry point binary
│ └── Cargo.toml
├── docs/ # Dokumentasi project
└── static/ # Asset statis


## Keputusan Teknis

- **Tauri dipilih** atas Electron karena lebih ringan dan mendukung Rust backend
- **Svelte dipilih** atas React/Vue karena ringan, performa baik untuk tabel data besar
- **SQLite bundled** agar tidak perlu install database terpisah di komputer pengguna
- **Self-hosted** — setiap OPD deploy sendiri, data tidak keluar dari komputer mereka
- **Desktop app** — tidak perlu server, cocok untuk lingkungan pemerintah

## Identitas

- **Nama:** Aerarium (perbendaharaan negara Roma kuno)
- **Identifier:** com.gdmahayana.aerarium
- **Lisensi:** GNU Affero General Public License v3.0
- **Repository:** https://github.com/gdmahayana/aerarium