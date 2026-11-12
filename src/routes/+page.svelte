<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
    created_at: string;
  }

  interface Dokumen {
    id: number;
    tahun_anggaran_id: number;
    jenis: string;
    versi: number;
    nomor_se: string | null;
    status: string;
    created_at: string;
  }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let dokumenList = $state<Dokumen[]>([]);
  let loading = $state(true);
  let summary = $state<[number, number, number] | null>(null);

  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;
      if (tahunAktif) {
        dokumenList = await invoke<Dokumen[]>("get_dokumen", {
          tahunAnggaranId: tahunAktif.id
        });
        summary = await invoke<[number, number, number]>("get_dashboard_summary", { tahunAnggaranId: tahunAktif.id });
      }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  function formatRp(n: number) {
    if (n >= 1_000_000_000) return "Rp " + (n / 1_000_000_000).toFixed(1) + " M";
    if (n >= 1_000_000) return "Rp " + (n / 1_000_000).toFixed(1) + " jt";
    return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(n);
  }

  const badgeClass: Record<string, string> = {
    draft: "badge-draft",
    konsep: "badge-konsep",
    ditetapkan: "badge-ditetapkan",
    dikunci: "badge-dikunci",
    diarsipkan: "badge-diarsipkan",
  };
</script>

<div class="page-header">
  <h2>Dashboard</h2>
  <p>Plan with Confidence. Change with Control.</p>
</div>

{#if loading}
  <p style="color:var(--text-3); font-size:13px;">Memuat data...</p>
{:else if !tahunAktif}
  <div class="card" style="padding:48px; text-align:center; max-width:400px; margin:60px auto;">
    <p style="font-size:13px; color:var(--text-2); margin-bottom:4px;">Belum ada tahun anggaran aktif.</p>
    <p style="font-size:12px; color:var(--text-3); margin-bottom:20px;">Mulai dengan menambahkan tahun anggaran</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Tambah Tahun Anggaran</a>
  </div>
{:else}
  <div style="display:grid; grid-template-columns:repeat(3,1fr); gap:16px; margin-bottom:28px;">
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:8px;">Tahun Anggaran</div>
      <div style="font-size:36px; font-weight:700; color:var(--text); letter-spacing:-0.03em;">{tahunAktif.tahun}</div>
      <span class="badge badge-aktif" style="margin-top:8px;">Aktif</span>
    </div>
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:8px;">Total Dokumen</div>
      <div style="font-size:36px; font-weight:700; color:var(--text); letter-spacing:-0.03em;">{dokumenList.length}</div>
    </div>
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:8px;">Dokumen Aktif</div>
      <div style="font-size:36px; font-weight:700; color:var(--text); letter-spacing:-0.03em;">
        {dokumenList.filter(d => d.status === "konsep" || d.status === "draft").length}
      </div>
    </div>
  </div>

  {#if summary}
  <div style="display:grid; grid-template-columns:2fr 1fr; gap:16px; margin-bottom:28px;">
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:12px;">Pagu vs Rincian</div>
      <div style="display:flex; justify-content:space-between; margin-bottom:6px;">
        <span style="font-size:12px; color:var(--text-2);">Total Pagu</span>
        <span style="font-size:13px; font-weight:600;">{formatRp(summary[0])}</span>
      </div>
      <div style="display:flex; justify-content:space-between; margin-bottom:10px;">
        <span style="font-size:12px; color:var(--text-2);">Total Rincian</span>
        <span style="font-size:13px; font-weight:600; color:{summary[1] > summary[0] ? '#c00000' : 'var(--text)'};">
          {formatRp(summary[1])}
        </span>
      </div>
      <div style="background:var(--border); border-radius:4px; height:8px; overflow:hidden;">
        <div style="height:100%; border-radius:4px; background:{summary[1] > summary[0] ? '#c00000' : 'var(--accent)'}; width:{Math.min(summary[0] > 0 ? (summary[1]/summary[0])*100 : 0, 100)}%;"></div>
      </div>
      <div style="font-size:11px; color:var(--text-3); margin-top:6px; text-align:right;">
        {summary[0] > 0 ? ((summary[1]/summary[0])*100).toFixed(1) : 0}% terisi
      </div>
    </div>
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:8px;">Sub Kegiatan ⚠️</div>
      <div style="font-size:36px; font-weight:700; color:{summary[2] > 0 ? '#856404' : 'var(--text)'}; letter-spacing:-0.03em;">{summary[2]}</div>
      <div style="font-size:12px; color:var(--text-3); margin-top:4px;">belum SHS Resmi</div>
    </div>
  </div>
  {/if}

  <div class="card">
    <div style="display:flex; align-items:center; justify-content:space-between; padding:16px 20px; border-bottom:1px solid var(--border);">
      <span class="label">Dokumen</span>
      <a href="/dokumen/buat" class="btn btn-primary" style="font-size:12px; padding:5px 12px;">+ Buat Dokumen</a>
    </div>

    {#if dokumenList.length === 0}
      <div style="padding:40px; text-align:center;">
        <p style="font-size:13px; color:var(--text-3);">Belum ada dokumen untuk tahun ini.</p>
      </div>
    {:else}
      {#each dokumenList as dok}
        <div style="display:flex; align-items:center; justify-content:space-between; padding:12px 20px; border-bottom:1px solid var(--border);">
          <div>
            <span style="font-size:13px; font-weight:600; color:var(--text);">{dok.jenis}</span>
            <span style="font-size:13px; color:var(--text-3);"> v{dok.versi}</span>
            {#if dok.nomor_se}
              <div style="font-size:11px; color:var(--text-3); margin-top:2px;">SE: {dok.nomor_se}</div>
            {/if}
          </div>
          <span class="badge {badgeClass[dok.status] || 'badge-draft'}">{dok.status}</span>
        </div>
      {/each}
    {/if}
  </div>
{/if}
