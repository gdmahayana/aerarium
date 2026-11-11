<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

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

  const badgeClass: Record<string, string> = {
    draft: "badge-draft",
    konsep: "badge-konsep",
    ditetapkan: "badge-ditetapkan",
    dikunci: "badge-dikunci",
    diarsipkan: "badge-diarsipkan",
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

  // Kelompokkan dokumen per jenis
  const jenisList = ["RKA-SKPD", "DPA-SKPD", "RKA-SKPD Perubahan", "DPPA-SKPD"];
  const jenisDokumenResmi = ["RKA-SKPD", "DPA-SKPD", "RKA-SKPD Perubahan", "DPPA-SKPD"];

  function dokumenPerJenis(jenis: string) {
    return dokumenList.filter(d => d.jenis === jenis).sort((a, b) => b.versi - a.versi);
  }
</script>

<div class="page-header">
  <h2>Dokumen</h2>
  <p>Kelola dokumen anggaran — RKA-SKPD, DPA-SKPD, dan perubahannya</p>
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
  <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:20px;">
    <div style="font-size:13px; color:var(--text-2);">
      Tahun Anggaran: <span style="font-weight:700; color:var(--text);">{tahunAktif.tahun}</span>
    </div>
    <a href="/dokumen/buat" class="btn btn-primary">+ Buat Dokumen</a>
  </div>

  {#if dokumenList.length === 0}
    <div class="card" style="padding:48px; text-align:center;">
      <p style="font-size:13px; color:var(--text-3); margin-bottom:16px;">Belum ada dokumen.</p>
      <a href="/dokumen/buat" class="btn btn-secondary">Buat Dokumen Pertama</a>
    </div>
  {:else}
    <!-- Kertas Kerja -->
    {#if dokumenPerJenis("Kertas Kerja").length > 0}
      <div class="card" style="margin-bottom:16px; border-left:3px solid var(--accent);">
        <div style="padding:14px 20px; border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;">
          <div>
            <span class="label">Kertas Kerja</span>
            <span style="font-size:11px; color:var(--text-3); margin-left:8px;">— dokumen persiapan internal</span>
          </div>
        </div>
        {#each dokumenPerJenis("Kertas Kerja") as dok, i}
          <div style="display:flex; align-items:center; justify-content:space-between; padding:12px 20px; {i < dokumenPerJenis("Kertas Kerja").length - 1 ? "border-bottom:1px solid var(--border);" : ""}">
            <div>
              <span style="font-size:13px; font-weight:600; color:var(--text);">v{dok.versi}</span>
              {#if dok.catatan}
                <div style="font-size:11px; color:var(--text-3); margin-top:2px;">{dok.catatan}</div>
              {/if}
            </div>
            <div style="display:flex; align-items:center; gap:10px;">
              <span class="badge {badgeClass[dok.status] || 'badge-draft'}">{dok.status}</span>
              <a href="/dokumen/{dok.id}" class="btn btn-ghost" style="font-size:12px;">Buka →</a>
            </div>
          </div>
        {/each}
      </div>
    {/if}

    <div style="display:flex; flex-direction:column; gap:16px;">
      {#each jenisList as jenis}
        {#if dokumenPerJenis(jenis).length > 0}
          <div class="card">
            <div style="padding:14px 20px; border-bottom:1px solid var(--border);">
              <span class="label">{jenis}</span>
            </div>
            {#each dokumenPerJenis(jenis) as dok, i}
              <div style="display:flex; align-items:center; justify-content:space-between; padding:12px 20px; {i < dokumenPerJenis(jenis).length - 1 ? 'border-bottom:1px solid var(--border);' : ''}">
                <div>
                  <span style="font-size:13px; font-weight:600; color:var(--text);">v{dok.versi}</span>
                  {#if dok.nomor_se}
                    <span style="font-size:12px; color:var(--text-3); margin-left:8px;">SE: {dok.nomor_se}</span>
                  {/if}
                  {#if dok.catatan}
                    <div style="font-size:11px; color:var(--text-3); margin-top:2px;">{dok.catatan}</div>
                  {/if}
                </div>
                <div style="display:flex; align-items:center; gap:10px;">
                  <span class="badge {badgeClass[dok.status] || 'badge-draft'}">{dok.status}</span>
                  <a href="/dokumen/{dok.id}" class="btn btn-ghost" style="font-size:12px;">Buka →</a>
                </div>
              </div>
            {/each}
          </div>
        {/if}
      {/each}
    </div>
  {/if}
{/if}
