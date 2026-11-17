// src/lib/dokumen-config.ts
// Satu sumber kebenaran untuk semua konfigurasi jenis dan status dokumen

export type JenisDokumen =
  | 'Kertas Kerja'
  | 'Pagu Indikatif'
  | 'RKA-SKPD'
  | 'DPA-SKPD'
  | 'RKA-SKPD Perubahan'
  | 'DPPA-SKPD'

export type StatusDokumen = 'draft' | 'konsep' | 'ditetapkan' | 'dikunci' | 'diarsipkan';

export interface JenisConfig {
  warna: string;
  warnaGlow: string;
  icon: string;
  tagline: string;
  adaSE: boolean;
}

export interface StatusConfig {
  bg: string;
  color: string;
  label: string;
}

export const JENIS_CONFIG: Record<JenisDokumen, JenisConfig> = {
  'Kertas Kerja':        { warna: '#6366f1', warnaGlow: '#6366f115', icon: '✦', tagline: 'Survei kebutuhan & estimasi harga',           adaSE: false },
  'Pagu Indikatif':      { warna: '#0ea5e9', warnaGlow: '#0ea5e915', icon: '◈', tagline: 'Alokasi Bappeda · SHS Resmi',                 adaSE: false },
  'RKA-SKPD':            { warna: '#10b981', warnaGlow: '#10b98115', icon: '▸', tagline: 'Rencana Kerja & Anggaran',                    adaSE: true  },
  'DPA-SKPD':            { warna: '#f59e0b', warnaGlow: '#f59e0b15', icon: '▸', tagline: 'Dokumen Pelaksanaan Anggaran',                adaSE: true  },
  'RKA-SKPD Perubahan':  { warna: '#10b981', warnaGlow: '#10b98115', icon: '▸', tagline: 'Rencana Kerja & Anggaran Perubahan',          adaSE: true  },
  'DPPA-SKPD':           { warna: '#f59e0b', warnaGlow: '#f59e0b15', icon: '▸', tagline: 'Dokumen Pelaksanaan Perubahan Anggaran',      adaSE: true  },
};

export const STATUS_CONFIG: Record<StatusDokumen, StatusConfig> = {
  draft:      { bg: '#1e293b', color: '#94a3b8', label: 'Draft'      },
  konsep:     { bg: '#0c2a4a', color: '#7dd3fc', label: 'Konsep'     },
  ditetapkan: { bg: '#052e16', color: '#4ade80', label: 'Ditetapkan' },
  dikunci:    { bg: '#2d1f00', color: '#fbbf24', label: 'Dikunci'    },
  diarsipkan: { bg: '#1a1a2e', color: '#6b7280', label: 'Diarsipkan' },
};

// Transisi status yang valid
export const TRANSISI_STATUS: Record<StatusDokumen, StatusDokumen[]> = {
  draft:      ['konsep'],
  konsep:     ['draft', 'ditetapkan'],
  ditetapkan: ['konsep', 'dikunci'],
  dikunci:    ['diarsipkan'],
  diarsipkan: [],
};

// Helper
export function jenisConfig(jenis: string): JenisConfig {
  return JENIS_CONFIG[jenis as JenisDokumen] ?? JENIS_CONFIG['RKA-SKPD'];
}

export function statusConfig(status: string): StatusConfig {
  return STATUS_CONFIG[status as StatusDokumen] ?? STATUS_CONFIG['draft'];
}

// Urutan tahapan untuk tampilan halaman Dokumen
export const TAHAPAN = [
  {
    group: 'Persiapan',
    items: [
      { jenis: 'Kertas Kerja',   warnaGlow: '#6366f130', tunggal: true,  next: null        },
      { jenis: 'Pagu Indikatif', warnaGlow: '#0ea5e930', tunggal: true,  next: 'RKA-SKPD'  },
    ],
  },
  {
    group: 'Penganggaran',
    items: [
      { jenis: 'RKA-SKPD',  warnaGlow: '#10b98130', tunggal: false, next: 'DPA-SKPD' },
      { jenis: 'DPA-SKPD',  warnaGlow: '#f59e0b30', tunggal: true,  next: null       },
    ],
  },
  {
    group: 'Perubahan',
    items: [
      { jenis: 'RKA-SKPD Perubahan',  warnaGlow: '#10b98130', tunggal: false, next: 'DPPA-SKPD'          },
      { jenis: 'DPPA-SKPD',           warnaGlow: '#f59e0b30', tunggal: true,  next: null                 },
    ],
  },
];
