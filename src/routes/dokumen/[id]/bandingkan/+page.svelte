<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { jenisConfig, statusConfig } from "$lib/dokumen-config";
  import { toast } from "$lib/stores/toast";
  import { onMount } from "svelte";
  import { page } from "$app/stores";

  interface Dokumen {
    id: number;
    jenis: string;
    versi: number;
    nomor_se: string | null;
    status: string;
    asal_id: number | null;
    ditetapkan_at: string | null;
  }

  interface SubKegiatanDiff {
    master_sub_kegiatan_id: number;
    kode: string;
    uraian: string;
    pagu_a: number | null;
    pagu_b: number | null;
    total_rincian_a: number | null;
    total_rincian_b: number | null;
    selisih_pagu: number;
    selisih_rincian: number;
    status: "tambah" | "hapus" | "ubah" | "sama";
  }

  interface PerbandinganResult {
    dokumen_a: Dokumen;
    dokumen_b: Dokumen;
    sub_kegiatan: SubKegiatanDiff[];
    total_pagu_a: number;
    total_pagu_b: number;
    total_rincian_a: number;
    total_rincian_b: number;
  }

  const dokumenId = $derived(Number($page.params.id));

  let semuaDokumen = $state<Dokumen[]>([]);
  let bandingkanId = $state<number | null>(null);
  let hasil = $state<PerbandinganResult | null>(null);
  let loading = $state(false);
  let loadingDokumen = $state(true);
  let filterStatus = $state<string>("tambah,hapus,ubah");

  onMount(async () => {
    try {
      const list = await invoke<any[]>("get_tahun_anggaran");
      const aktif = list.find((t: any) => t.aktif);
      if (!aktif) return;
      const docs = await invoke<Dokumen[]>("get_dokumen", { tahunAnggaranId: aktif.id });
      semuaDokumen = docs.filter(d => d.id !== dokumenId);
    } catch(e) { toast.error(String(e)); }
    finally { loadingDokumen = false; }
  });

  async function bandingkan() {
    if (!bandingkanId) return;
    loading = true;
    hasil = null;
    try {
      hasil = await invoke<PerbandinganResult>("get_perbandingan_dokumen", {
        idA: dokumenId,
        idB: bandingkanId,
      });
    } catch(e) { toast.error(String(e)); }
    finally { loading = false; }
  }

  function formatRp(n: number | null) {
    if (n === null) return "—";
    return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(n);
  }

  function formatSelisih(n: number) {
    const prefix = n > 0 ? "+" : "";
    return prefix + new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(n);
  }

  const warnaBadge: Record<string, string> = {
    tambah: "background:#065f46; color:#fff;",
    hapus:  "background:#9f1239; color:#fff;",
    ubah:   "background:#92400e; color:#fff;",
    sama:   "background:var(--surface-2); color:var(--text-3);",
  };

  const labelBadge: Record<string, string> = {
    tambah: "+ TAMBAH",
    hapus:  "− HAPUS",
    ubah:   "~ UBAH",
    sama:   "= SAMA",
  };

  let filterList = $derived(filterStatus.split(",").filter(Boolean));

  let hasilFiltered = $derived(
    hasil ? hasil.sub_kegiatan.filter(s => filterList.includes(s.status)) : []
  );

  let dokumenPembanding = $derived(
    semuaDokumen.find(d => d.id === bandingkanId) ?? null
  );
<\/script>

<div class="page-header">
  <div style="display:flex; align-items:center; gap:8px;">
    <a href="/dokumen/{dokumenId}" style="font-size:12px; color:var(--text-3); text-decoration:none;">← Kembali</a>
  </div>
  <h2>Perbandingan Dokumen</h2>
  <p>Bandingkan pagu dan rincian antar dua dokumen anggaran</p>
</div>

<!-- Pilih dokumen pembanding -->
<div class="card" style="padding:16px; margin-bottom:20px;">
  <div style="font-size:12px; font-weight:600; color:var(--text-2); margin-bottom:12px;">Pilih Dokumen Pembanding</div>
  <div style="display:flex; gap:10px; align-items:flex-end; flex-wrap:wrap;">
    <div style="flex:1; min-width:280px;">
      <div class="label" style="margin-bottom:4px;">Dokumen A (saat ini)</div>
      <div style="padding:8px 12px; background:var(--surface-2); border-radius:6px; font-size:12px; color:var(--text-2);">
        ID #{dokumenId}
      </div>
    </div>
    <div style="font-size:18px; color:var(--text-3); padding-bottom:8px;">vs</div>
    <div style="flex:1; min-width:280px;">
      <div class="label" style="margin-bottom:4px;">Dokumen B (pembanding)</div>
      {#if loadingDokumen}
        <div style="font-size:12px; color:var(--text-3);">Memuat...</div>
      {:else}
        <select class="input" bind:value={bandingkanId}>
          <option value={null}>Pilih dokumen...</option>
          {#each semuaDokumen as dok}
            <option value={dok.id}>
              [{dok.jenis} v{dok.versi}] {dok.status}
              {#if dok.nomor_se} — {dok.nomor_se}{/if}
            </option>
          {/each}
        </select>
      {/if}
    </div>
    <button
      onclick={bandingkan}
      disabled={!bandingkanId || loading}
      class="btn btn-primary"
      style="font-size:12px;"
    >
      {loading ? "Membandingkan..." : "Bandingkan →"}
    </button>
  </div>
</div>

{#if hasil}
  <!-- Summary -->
  <div style="display:grid; grid-template-columns:1fr 1fr 1fr 1fr; gap:12px; margin-bottom:20px;">
    <div class="card" style="padding:14px 16px;">
      <div style="font-size:11px; color:var(--text-3); margin-bottom:4px;">Total Pagu A</div>
      <div style="font-size:14px; font-weight:700;">{formatRp(hasil.total_pagu_a)}</div>
    </div>
    <div class="card" style="padding:14px 16px;">
      <div style="font-size:11px; color:var(--text-3); margin-bottom:4px;">Total Pagu B</div>
      <div style="font-size:14px; font-weight:700;">{formatRp(hasil.total_pagu_b)}</div>
    </div>
    <div class="card" style="padding:14px 16px;">
      <div style="font-size:11px; color:var(--text-3); margin-bottom:4px;">Selisih Pagu</div>
      {@const sel = hasil.total_pagu_b - hasil.total_pagu_a}
      <div style="font-size:14px; font-weight:700; color:{sel > 0 ? '#16a34a' : sel < 0 ? '#dc2626' : 'var(--text)'};">{formatSelisih(sel)}</div>
    </div>
    <div class="card" style="padding:14px 16px;">
      <div style="font-size:11px; color:var(--text-3); margin-bottom:4px;">Sub Kegiatan Berubah</div>
      <div style="font-size:14px; font-weight:700;">{hasil.sub_kegiatan.filter(s => s.status !== "sama").length} dari {hasil.sub_kegiatan.length}</div>
    </div>
  </div>

  <!-- Filter -->
  <div style="display:flex; gap:8px; margin-bottom:12px; flex-wrap:wrap;">
    <span style="font-size:12px; color:var(--text-3); align-self:center;">Tampilkan:</span>
    {#each ["tambah","hapus","ubah","sama"] as s}
      {@const aktif = filterList.includes(s)}
      <button
        onclick={() => {
          const list = filterList.includes(s)
            ? filterList.filter(x => x !== s)
            : [...filterList, s];
          filterStatus = list.join(",");
        }}
        style="font-size:11px; font-weight:600; padding:3px 10px; border-radius:4px; border:2px solid {aktif ? "transparent" : "var(--border)"}; cursor:pointer; {aktif ? warnaBadge[s] : "background:transparent; color:var(--text-3);"};"
      >{labelBadge[s]} ({hasil.sub_kegiatan.filter(x => x.status === s).length})</button>
    {/each}
  </div>

  <!-- Tabel diff -->
  <div class="card" style="overflow:hidden;">
    <table style="width:100%; border-collapse:collapse;">
      <thead>
        <tr style="border-bottom:1px solid var(--border);">
          <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase; width:32px;"></th>
          <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Kode</th>
          <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Sub Kegiatan</th>
          <th style="padding:10px 16px; text-align:right; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Pagu A</th>
          <th style="padding:10px 16px; text-align:right; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Pagu B</th>
          <th style="padding:10px 16px; text-align:right; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Selisih Pagu</th>
          <th style="padding:10px 16px; text-align:right; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Rincian A</th>
          <th style="padding:10px 16px; text-align:right; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Rincian B</th>
        </tr>
      </thead>
      <tbody>
        {#each hasilFiltered as item, i}
          <tr style="border-bottom:1px solid var(--border); {i % 2 === 1 ? "background:rgba(255,255,255,0.02);" : ""}">
            <td style="padding:8px 16px;">
              <span style="font-size:10px; font-weight:700; padding:2px 6px; border-radius:3px; {warnaBadge[item.status]}">{labelBadge[item.status]}</span>
            </td>
            <td style="padding:8px 16px; font-family:monospace; font-size:11px; color:var(--text-3);">{item.kode}</td>
            <td style="padding:8px 16px; font-size:13px; color:var(--text);">{item.uraian}</td>
            <td style="padding:8px 16px; font-size:12px; text-align:right; color:var(--text-2); font-variant-numeric:tabular-nums;">{formatRp(item.pagu_a)}</td>
            <td style="padding:8px 16px; font-size:12px; text-align:right; color:var(--text-2); font-variant-numeric:tabular-nums;">{formatRp(item.pagu_b)}</td>
            <td style="padding:8px 16px; font-size:12px; text-align:right; font-weight:600; font-variant-numeric:tabular-nums; color:{item.selisih_pagu > 0 ? "#16a34a" : item.selisih_pagu < 0 ? "#dc2626" : "var(--text-3)"};">{item.selisih_pagu !== 0 ? formatSelisih(item.selisih_pagu) : "—"}</td>
            <td style="padding:8px 16px; font-size:12px; text-align:right; color:var(--text-3); font-variant-numeric:tabular-nums;">{formatRp(item.total_rincian_a)}</td>
            <td style="padding:8px 16px; font-size:12px; text-align:right; color:var(--text-3); font-variant-numeric:tabular-nums;">{formatRp(item.total_rincian_b)}</td>
          </tr>
        {/each}
        {#if hasilFiltered.length === 0}
          <tr><td colspan="8" style="padding:32px; text-align:center; font-size:13px; color:var(--text-3);">Tidak ada item untuk filter ini</td></tr>
        {/if}
      </tbody>
    </table>
  </div>
{/if}
