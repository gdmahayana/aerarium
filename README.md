# Aerarium

Aplikasi desktop open source untuk perencanaan anggaran OPD/SIPD di lingkungan pemerintah daerah Indonesia.

## Tentang

Aerarium membantu operator OPD dalam menyusun rincian belanja dari database harga (SSH/SHBJ), mensimulasikan perubahan pagu anggaran, dan menghasilkan dokumen referensi yang siap digunakan untuk input ke SIPD.

## Fitur

- **Import SSH/SHBJ** — import database harga dari file Excel SIPD
- **Susun Rincian Belanja** — pilih item dari database, mapping otomatis ke kode rekening
- **Simulasi Perubahan Pagu** — adjustment rincian saat pagu naik atau turun
- **Export Dokumen** — output Excel dan PDF sebagai referensi input SIPD

## Stack

- [Tauri v2](https://tauri.app/) — desktop app framework
- [Svelte](https://svelte.dev/) — frontend UI
- [Rust](https://www.rust-lang.org/) — backend & logika bisnis
- [SQLite](https://www.sqlite.org/) — database lokal

## Instalasi

### Prasyarat

- Rust (via rustup)
- Node.js
- pnpm

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

## Lisensi

GNU Affero General Public License v3.0 — lihat [LICENSE](LICENSE) untuk detail.