<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { jenisConfig, statusConfig } from "$lib/dokumen-config";
  import { toast } from "$lib/stores/toast";

  interface TahunAnggaran { id: number; tahun: number; aktif: boolean; }
  interface Dokumen {
    id: number; jenis: string; versi: number;
    nomor_se: string | null; status: string; created_at: string;
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
        [dokumenList, summary] = await Promise.all([
          invoke<Dokumen[]>("get_dokumen", { tahunAnggaranId: tahunAktif.id }),
          invoke<[number, number, number]>("get_dashboard_summary", { tahunAnggaranId: tahunAktif.id }),
        ]);
      }
    } catch(e) { toast.error(String(e)); }
    finally { loading = false; }
  });

  function formatRp(n: number) {
    if (n >= 1_000_000_000) return "Rp " + (n/1_000_000_000).toFixed(1) + " M";
    if (n >= 1_000_000)     return "Rp " + (n/1_000_000).toFixed(1) + " jt";
    return new Intl.NumberFormat("id-ID", { style:"currency", currency:"IDR", minimumFractionDigits:0 }).format(n);
  }

  // Dokumen terkini — 5 terakhir, urutkan by created_at desc
  let dokumenTerkini = $derived(
    [...dokumenList]
      .sort((a, b) => b.created_at.localeCompare(a.created_at))
      .slice(0, 5)
  );

  // Breakdown per grup
  let persiapanCount    = $derived(dokumenList.filter(d => ["Kertas Kerja","Pagu Indikatif"].includes(d.jenis)).length);
  let penganggaranCount = $derived(dokumenList.filter(d => ["RKA-SKPD","DPA-SKPD"].includes(d.jenis)).length);
  let perubahanCount    = $derived(dokumenList.filter(d => ["RKA-SKPD Perubahan","DPPA-SKPD"].includes(d.jenis)).length);
  let draftCount        = $derived(dokumenList.filter(d => d.status === "draft" || d.status === "konsep").length);
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
  <!-- Row 1: Tahun + breakdown dokumen -->
  <div style="display:grid; grid-template-columns:1fr 1fr 1fr 1fr; gap:16px; margin-bottom:20px;">
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:8px;">Tahun Anggaran</div>
      <div style="font-size:36px; font-weight:700; color:var(--accent); letter-spacing:-0.03em;">{tahunAktif.tahun}</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:6px;">{dokumenList.length} dokumen total</div>
    </div>
    <div class="card" style="padding:20px; cursor:pointer;" onclick={() => goto('/dokumen')}>
      <div class="label" style="margin-bottom:8px;">Persiapan</div>
      <div style="font-size:36px; font-weight:700; color:#6366f1; letter-spacing:-0.03em;">{persiapanCount}</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:6px;">Kertas Kerja · Pagu Indikatif</div>
    </div>
    <div class="card" style="padding:20px; cursor:pointer;" onclick={() => goto('/dokumen')}>
      <div class="label" style="margin-bottom:8px;">Penganggaran</div>
      <div style="font-size:36px; font-weight:700; color:#10b981; letter-spacing:-0.03em;">{penganggaranCount}</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:6px;">RKA-SKPD · DPA-SKPD</div>
    </div>
    <div class="card" style="padding:20px; cursor:pointer;" onclick={() => goto('/dokumen')}>
      <div class="label" style="margin-bottom:8px;">Perubahan</div>
      <div style="font-size:36px; font-weight:700; color:#f59e0b; letter-spacing:-0.03em;">{perubahanCount}</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:6px;">RKA Perubahan · DPPA-SKPD</div>
    </div>
  </div>

  <!-- Row 2: Pagu vs Rincian + Warning SHS -->
  {#if summary}
  <div style="display:grid; grid-template-columns:2fr 1fr 1fr; gap:16px; margin-bottom:20px;">
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:12px;">Pagu vs Rincian</div>
      <div style="display:flex; justify-content:space-between; margin-bottom:6px;">
        <span style="font-size:12px; color:var(--text-2);">Total Pagu</span>
        <span style="font-size:13px; font-weight:600;">{formatRp(summary[0])}</span>
      </div>
      <div style="display:flex; justify-content:space-between; margin-bottom:10px;">
        <span style="font-size:12px; color:var(--text-2);">Total Rincian</span>
        <span style="font-size:13px; font-weight:600; color:{summary[1] > summary[0] ? '#f87171' : 'var(--text)'};">
          {formatRp(summary[1])}
        </span>
      </div>
      <div style="background:var(--border); border-radius:4px; height:8px; overflow:hidden;">
        <div style="height:100%; border-radius:4px; background:{summary[1] > summary[0] ? '#f87171' : 'var(--accent)'}; width:{Math.min(summary[0] > 0 ? (summary[1]/summary[0])*100 : 0, 100)}%;"></div>
      </div>
      <div style="font-size:11px; color:var(--text-3); margin-top:6px; text-align:right;">
        {summary[0] > 0 ? ((summary[1]/summary[0])*100).toFixed(1) : 0}% terisi
      </div>
    </div>
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:8px;">Belum SHS Resmi</div>
      <div style="font-size:36px; font-weight:700; color:{summary[2] > 0 ? '#fbbf24' : '#4ade80'}; letter-spacing:-0.03em;">{summary[2]}</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:4px;">sub kegiatan {summary[2] === 0 ? '✓ semua resmi' : 'perlu ditindak'}</div>
    </div>
    <div class="card" style="padding:20px;">
      <div class="label" style="margin-bottom:8px;">Dalam Proses</div>
      <div style="font-size:36px; font-weight:700; color:var(--text); letter-spacing:-0.03em;">{draftCount}</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:4px;">dokumen draft / konsep</div>
    </div>
  </div>
  {/if}

  <!-- Row 3: Dokumen terkini -->
  <div class="card">
    <div style="display:flex; align-items:center; justify-content:space-between; padding:16px 20px; border-bottom:1px solid var(--border);">
      <span class="label">Terkini</span>
      <a href="/dokumen" class="btn btn-ghost" style="font-size:12px;">Lihat Semua →</a>
    </div>
    {#if dokumenTerkini.length === 0}
      <div style="padding:40px; text-align:center;">
        <p style="font-size:13px; color:var(--text-3); margin-bottom:12px;">Belum ada dokumen.</p>
        <a href="/dokumen/buat" class="btn btn-primary" style="font-size:12px;">+ Buat Dokumen</a>
      </div>
    {:else}
      {#each dokumenTerkini as dok}
        {@const jc = jenisConfig(dok.jenis)}
        {@const sc = statusConfig(dok.status)}
        <div
          style="display:flex; align-items:center; gap:14px; padding:12px 20px; border-bottom:1px solid var(--border); cursor:pointer;"
          onclick={() => goto('/dokumen/' + dok.id)}
          onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = 'var(--surface-2)'}
          onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = 'transparent'}
        >
          <span style="font-size:14px; color:{jc.warna}; width:20px; text-align:center;">{jc.icon}</span>
          <div style="flex:1;">
            <span style="font-size:13px; font-weight:600; color:var(--text);">{dok.jenis}</span>
            <span style="font-size:12px; color:var(--text-3);"> v{dok.versi}</span>
            {#if dok.nomor_se}
              <span style="font-size:11px; color:var(--text-3); margin-left:8px;">SE: {dok.nomor_se}</span>
            {/if}
          </div>
          <span style="font-size:10px; font-weight:700; padding:2px 8px; border-radius:4px; background:{sc.bg}; color:{sc.color}; letter-spacing:0.05em; text-transform:uppercase;">{sc.label}</span>
        </div>
      {/each}
    {/if}
  </div>
{/if}
