<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  interface Dokumen {
    id: number;
    tahun_anggaran_id: number;
    jenis: string;
    versi: number;
    nomor_se: string | null;
    tanggal_se: string | null;
    status: string;
    asal_id: number | null;
    catatan: string | null;
    created_at: string;
  }

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
  }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let dokumenList = $state<Dokumen[]>([]);
  let loading = $state(true);
  let error = $state("");

  const badgeColor: Record<string, string> = {
    draft:      "#6b7280",
    konsep:     "#2563eb",
    ditetapkan: "#16a34a",
    dikunci:    "#9333ea",
    diarsipkan: "#9ca3af",
  };

  const badgeBg: Record<string, string> = {
    draft:      "#f3f4f6",
    konsep:     "#dbeafe",
    ditetapkan: "#dcfce7",
    dikunci:    "#f3e8ff",
    diarsipkan: "#f9fafb",
  };

  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;
      if (tahunAktif) {
        dokumenList = await invoke<Dokumen[]>("get_dokumen", {
          tahunAnggaranId: tahunAktif.id
        });
      }
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  function byJenis(jenis: string) {
    return dokumenList.filter(d => d.jenis === jenis).sort((a, b) => b.versi - a.versi);
  }

  function aktif(jenis: string) {
    return byJenis(jenis).find(d => d.status !== "dikunci" && d.status !== "diarsipkan");
  }

  // Tahapan timeline
  const tahapan = [
    {
      jenis: "Kertas Kerja",
      icon: "📋",
      warna: "#6366f1",
      deskripsi: "Survei kebutuhan & estimasi harga",
      tunggal: true,
    },
    {
      jenis: "Pagu Indikatif",
      icon: "📊",
      warna: "#0ea5e9",
      deskripsi: "Alokasi dari Bappeda, SHS Resmi",
      tunggal: true,
    },
    {
      jenis: "RKA-SKPD",
      icon: "📄",
      warna: "#10b981",
      deskripsi: "Rencana Kerja Anggaran",
      tunggal: false,
    },
    {
      jenis: "DPA-SKPD",
      icon: "📁",
      warna: "#f59e0b",
      deskripsi: "Dokumen Pelaksanaan Anggaran",
      tunggal: true,
    },
    {
      jenis: "RKA-SKPD Perubahan",
      icon: "📄",
      warna: "#10b981",
      deskripsi: "Rencana Kerja Anggaran Perubahan",
      tunggal: false,
    },
    {
      jenis: "DPPA-SKPD",
      icon: "📁",
      warna: "#f59e0b",
      deskripsi: "Dokumen Pelaksanaan Perubahan Anggaran",
      tunggal: true,
    },
  ];
</script>

<div class="page-header">
  <div>
    <h2>Dokumen Anggaran</h2>
    {#if tahunAktif}
      <p>Tahun Anggaran <strong>{tahunAktif.tahun}</strong></p>
    {/if}
  </div>
  <a href="/dokumen/buat" class="btn btn-primary" style="font-size:12px;">+ Buat Dokumen</a>
</div>

{#if error}
  <div class="alert-error">{error}</div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !tahunAktif}
  <div class="card" style="padding:48px; text-align:center; max-width:400px; margin:60px auto;">
    <p style="font-size:13px; color:var(--text-2); margin-bottom:16px;">Belum ada tahun anggaran aktif.</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Atur Tahun Anggaran</a>
  </div>
{:else}
  <div style="display:flex; flex-direction:column; gap:0;">
    {#each tahapan as tahap, idx}
      {@const doks = byJenis(tahap.jenis)}
      {@const ada = doks.length > 0}

      <!-- Koneksi antar tahap -->
      {#if idx > 0}
        <div style="display:flex; align-items:center; padding:0 28px; height:28px;">
          <div style="width:2px; height:100%; background:var(--border); margin-left:19px;"></div>
          {#if idx === 2}
            <span style="font-size:10px; color:var(--text-3); margin-left:12px;">Jadikan RKA →</span>
          {:else if idx === 3}
            <span style="font-size:10px; color:var(--text-3); margin-left:12px;">Jadikan DPA →</span>
          {:else if idx === 5}
            <span style="font-size:10px; color:var(--text-3); margin-left:12px;">Jadikan DPPA →</span>
          {/if}
        </div>
      {/if}

      <!-- Kartu tahap -->
      <div style="display:flex; gap:16px; align-items:start;">
        <!-- Ikon bulat -->
        <div style="
          width:40px; height:40px; border-radius:50%; flex-shrink:0;
          background:{ada ? tahap.warna : 'var(--surface-2)'};
          border:2px solid {ada ? tahap.warna : 'var(--border)'};
          display:flex; align-items:center; justify-content:center;
          font-size:16px; margin-top:2px;
        ">{tahap.icon}</div>

        <!-- Konten -->
        <div style="flex:1;">
          <div style="display:flex; align-items:center; gap:10px; margin-bottom:8px;">
            <span style="font-size:13px; font-weight:700; color:{ada ? 'var(--text)' : 'var(--text-3)'};">
              {tahap.jenis}
            </span>
            <span style="font-size:11px; color:var(--text-3);">{tahap.deskripsi}</span>
            {#if !ada}
              <span style="font-size:11px; color:var(--text-3); margin-left:auto;">— belum dibuat</span>
            {/if}
          </div>

          {#if ada}
            <div style="display:flex; flex-direction:column; gap:6px;">
              {#each doks as dok}
                <div class="card" style="
                  padding:12px 16px;
                  display:flex; align-items:center; justify-content:space-between;
                  border-left:3px solid {tahap.warna};
                  cursor:pointer;
                " onclick={() => goto('/dokumen/' + dok.id)}>
                  <div style="display:flex; align-items:center; gap:12px;">
                    <!-- Badge versi -->
                    <span style="
                      font-size:11px; font-weight:700; font-family:monospace;
                      color:{tahap.warna}; min-width:28px;
                    ">v{dok.versi}</span>

                    <div>
                      {#if dok.nomor_se}
                        <div style="font-size:12px; font-weight:600; color:var(--text);">SE: {dok.nomor_se}</div>
                      {:else}
                        <div style="font-size:12px; color:var(--text-3);">Tanpa nomor SE</div>
                      {/if}
                      {#if dok.catatan}
                        <div style="font-size:11px; color:var(--text-3); margin-top:1px;">{dok.catatan}</div>
                      {/if}
                      {#if dok.tanggal_se}
                        <div style="font-size:11px; color:var(--text-3);">📅 {dok.tanggal_se}</div>
                      {/if}
                    </div>
                  </div>

                  <div style="display:flex; align-items:center; gap:10px;">
                    <!-- Badge status -->
                    <span style="
                      font-size:11px; font-weight:600; padding:3px 8px; border-radius:4px;
                      background:{badgeBg[dok.status] || '#f3f4f6'};
                      color:{badgeColor[dok.status] || '#6b7280'};
                    ">{dok.status}</span>

                    <button
                      class="btn btn-ghost"
                      style="font-size:12px;"
                      onclick={(e) => { e.stopPropagation(); goto('/dokumen/' + dok.id); }}
                    >Buka →</button>
                  </div>
                </div>
              {/each}
            </div>
          {:else if tahap.tunggal}
            <!-- Placeholder buat baru -->
            <a
              href="/dokumen/buat"
              style="
                display:inline-flex; align-items:center; gap:6px;
                font-size:12px; color:var(--text-3); padding:8px 12px;
                border:1px dashed var(--border); border-radius:6px;
                text-decoration:none;
              "
            >+ Buat {tahap.jenis}</a>
          {/if}
        </div>
      </div>
    {/each}
  </div>
{/if}