# Workflow Pengembangan Aerarium

## Git Workflow

### Branch Strategy

main → kode stabil, siap release
dev → development aktif
feat/xxx → fitur baru
fix/xxx → bugfix


### Alur Kerja

feat/nama-fitur → dev → main


- Semua development di branch `feat/` atau `fix/`
- Merge ke `dev` setelah selesai dan tested
- Merge ke `main` hanya saat siap release (semver tag)

### Commit Convention

Menggunakan [Conventional Commits](https://www.conventionalcommits.org/):

feat: tambah import SSH dari Excel
fix: perbaiki kalkulasi total pagu
docs: update README instalasi
refactor: reorganisasi struktur database
chore: update dependensi Tauri


## Versioning

Menggunakan [Semantic Versioning](https://semver.org/):

v0.1.0 → MVP pertama
v0.2.0 → fitur baru signifikan
v0.1.1 → bugfix minor


## Repository

- **Primary:** https://github.com/gdmahayana/aerarium
- **Mirror:** git.gdmahayana.my.id (Gitea, private, auto-sync dari GitHub)

## Setup Development

```bash
# Clone repo
git clone git@github.com:gdmahayana/aerarium.git
cd aerarium

# Install dependensi
pnpm install

# Jalankan dev mode
pnpm tauri dev

# Build
pnpm tauri build
```

## Struktur Commit Pertama

chore: init tauri + svelte project
chore: setup project structure
docs: update README