<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";

  interface SubKegiatan {
    id: number;
    kegiatan_id: number;
    kode: string;
    uraian: string;
    pagu: number;
    lokasi: string | null;
    waktu_pelaksanaan: string | null;
    keluaran: string | null;
    target_keluaran: string | null;
  }

  let subKegiatanList = $state<SubKegiatan[]>([]);
  let loading = $state(true);
  let error = $state("");

  let showForm = $state(false);
  let kode = $state("");
  let uraian = $state("");
  let pagu = $state("");
  let lokasi = $state("");
  let waktu = $state("");
  let keluaran = $state("");
  let target = $state("");
  let submitting = $state(false);

  const dokumenId = $derived(Number($page.params.id));
  const kegiatanId = $derived(Number($page.params.kid));

  onMount(async () => {
    try {
      subKegiatanList = await invoke<SubKegiatan[]>("get_sub_kegiatan", { kegiatanId });
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  function formatRupiah(n: number) {
    return new Intl.NumberFormat("id-ID", {
      style: "currency", currency: "IDR", minimumFractionDigits: 0,
    }).format(n);
  }

  async function simpanSubKegiatan() {
    if (!kode || !uraian) return;
    submitting = true;
    try {
      await invoke("tambah_sub_kegiatan", {
        input: {
          kegiatan_id: kegiatanId,
          kode,
          uraian,
          pagu: pagu ? parseFloat(pagu) : null,
          lokasi: lokasi || null,
          waktu_pelaksanaan: waktu || null,
          keluaran: keluaran || null,
          target_keluaran: target || null,
          sumber_dana_id: null,
        }
      });
      subKegiatanList = await invoke<SubKegiatan[]>("get_sub_kegiatan", { kegiatanId });
      kode = ""; uraian = ""; pagu = ""; lokasi = ""; waktu = ""; keluaran = ""; target = "";
      showForm = false;
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function hapusSubKegiatan(id: number) {
    if (!confirm("Hapus sub kegiatan ini?")) return;
    try {
      await invoke("hapus_sub_kegiatan", { id });
      subKegiatanList = subKegiatanList.filter(s => s.id !== id);
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="page-header">
  <div style="display:flex; align-items:center; gap:10px;">
    <a href="/dokumen/{dokumenId}" class="btn btn-ghost" style="font-size:12px;">← Kegiatan</a>
    <h2>Sub Kegiatan</h2>
  </div>
</div>

{#if error}
  <div class="alert-error">
    {error}
    <button onclick={() => error = ""} style="margin-left:8px; text-decoration:underline; background:none; border:none; color:inherit; cursor:pointer;">Tutup</button>
  </div>
{/if}

<div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:16px;">
  <span class="label">Sub Kegiatan</span>
  <button onclick={() => showForm = !showForm} class="btn btn-primary" style="font-size:12px;">
    + Tambah Sub Kegiatan
  </button>
</div>

{#if showForm}
  <div class="card" style="padding:20px; margin-bottom:16px; max-width:560px;">
    <div style="display:flex; flex-direction:column; gap:14px;">
      <div>
        <div class="label" style="margin-bottom:6px;">Kode Sub Kegiatan</div>
        <input type="text" placeholder="2.13.05.2.01.0006" bind:value={kode} class="input" />
      </div>
      <div>
        <div class="label" style="margin-bottom:6px;">Uraian</div>
        <input type="text" placeholder="Nama sub kegiatan..." bind:value={uraian} class="input" />
      </div>
      <div>
        <div class="label" style="margin-bottom:6px;">Pagu <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
        <input type="number" placeholder="0" bind:value={pagu} class="input" style="max-width:200px;" />
      </div>
      <div>
        <div class="label" style="margin-bottom:6px;">Lokasi <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
        <input type="text" placeholder="Semua Kecamatan" bind:value={lokasi} class="input" />
      </div>
      <div>
        <div class="label" style="margin-bottom:6px;">Waktu Pelaksanaan <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
        <input type="text" placeholder="Januari s.d Desember" bind:value={waktu} class="input" />
      </div>
      <div>
        <div class="label" style="margin-bottom:6px;">Keluaran <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
        <input type="text" placeholder="Jumlah laporan..." bind:value={keluaran} class="input" />
      </div>
      <div>
        <div class="label" style="margin-bottom:6px;">Target Keluaran <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
        <input type="text" placeholder="8 Laporan" bind:value={target} class="input" />
      </div>
      <div style="display:flex; gap:10px;">
        <button onclick={simpanSubKegiatan} disabled={submitting} class="btn btn-primary">
          {submitting ? "Menyimpan..." : "Simpan"}
        </button>
        <button onclick={() => showForm = false} class="btn btn-ghost">Batal</button>
      </div>
    </div>
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if subKegiatanList.length === 0}
  <div class="card" style="padding:40px; text-align:center;">
    <p style="font-size:13px; color:var(--text-3);">Belum ada sub kegiatan.</p>
  </div>
{:else}
  <div class="card">
    {#each subKegiatanList as sub, i}
      <div style="display:flex; align-items:center; justify-content:space-between; padding:14px 20px; {i < subKegiatanList.length - 1 ? "border-bottom:1px solid var(--border);" : ""}">
        <div style="flex:1; cursor:pointer;" onclick={() => goto("/dokumen/" + dokumenId + "/kegiatan/" + kegiatanId + "/sub/" + sub.id)}>
          <div style="font-size:12px; color:var(--text-3); font-family:monospace;">{sub.kode}</div>
          <div style="font-size:13px; color:var(--text); margin-top:2px;">{sub.uraian}</div>
          {#if sub.pagu > 0}
            <div style="font-size:12px; color:var(--text-2); margin-top:2px;">{formatRupiah(sub.pagu)}</div>
          {/if}
          {#if sub.lokasi}
            <div style="font-size:11px; color:var(--text-3); margin-top:2px;">{sub.lokasi}</div>
          {/if}
        </div>
        <div style="display:flex; gap:8px;">
          <button onclick={() => goto("/dokumen/" + dokumenId + "/kegiatan/" + kegiatanId + "/sub/" + sub.id)} class="btn btn-ghost" style="font-size:12px;">Buka →</button>
          <button onclick={() => hapusSubKegiatan(sub.id)} class="btn btn-ghost" style="font-size:12px; color:var(--text-3);">Hapus</button>
        </div>
      </div>
    {/each}
  </div>
{/if}
