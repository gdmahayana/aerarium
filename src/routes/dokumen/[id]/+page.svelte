<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";

  interface Dokumen {
    id: number;
    jenis: string;
    versi: number;
    nomor_se: string | null;
    tanggal_se: string | null;
    status: string;
    catatan: string | null;
  }

  interface MasterSubKegiatan {
    id: number;
    master_kegiatan_id: number;
    kode: string;
    uraian: string;
    aktif: boolean;
  }

  interface DokumenSubKegiatan {
    id: number;
    dokumen_id: number;
    master_sub_kegiatan_id: number;
    pagu: number;
    sumber_dana_id: number | null;
    lokasi: string | null;
    waktu_pelaksanaan: string | null;
    keluaran: string | null;
    target_keluaran: string | null;
  }

  interface SumberDana {
    id: number;
    kode: string;
    nama: string;
  }

  let dokumen = $state<Dokumen | null>(null);
  let dskList = $state<DokumenSubKegiatan[]>([]);
  let masterSubList = $state<MasterSubKegiatan[]>([]);
  let sumberDanaList = $state<SumberDana[]>([]);
  let loading = $state(true);
  let error = $state("");
  let showForm = $state(false);
  let submitting = $state(false);

  let form = $state({
    master_sub_kegiatan_id: 0,
    pagu: "",
    sumber_dana_id: "",
    lokasi: "",
    waktu_pelaksanaan: "",
    keluaran: "",
    target_keluaran: "",
  });

  const dokumenId = $derived(Number($page.params.id));

  onMount(async () => {
    try {
      const tahunList = await invoke<any[]>("get_tahun_anggaran");
      const tahunAktif = tahunList.find((t: any) => t.aktif);
      if (!tahunAktif) return;

      const list = await invoke<Dokumen[]>("get_dokumen", { tahunAnggaranId: tahunAktif.id });
      dokumen = list.find(d => d.id === dokumenId) || null;

      if (dokumen) {
        [dskList, masterSubList, sumberDanaList] = await Promise.all([
          invoke<DokumenSubKegiatan[]>("get_dokumen_sub_kegiatan", { dokumenId }),
          invoke<MasterSubKegiatan[]>("get_master_sub_kegiatan_aktif"),
          invoke<SumberDana[]>("get_sumber_dana", { tahunAnggaranId: tahunAktif.id }),
        ]);
      }
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

  function namaSubKegiatan(masterSubKegiatanId: number) {
    const sub = masterSubList.find(s => s.id === masterSubKegiatanId);
    return sub ? sub.kode + " — " + sub.uraian : "("+masterSubKegiatanId+")";
  }

  // Sub kegiatan master yang belum ditambahkan ke dokumen ini
  function masterSubTersedia() {
    const sudahAda = new Set(dskList.map(d => d.master_sub_kegiatan_id));
    return masterSubList.filter(s => !sudahAda.has(s.id));
  }

  async function simpan() {
    if (!form.master_sub_kegiatan_id) return;
    submitting = true;
    try {
      await invoke("tambah_dokumen_sub_kegiatan", {
        input: {
          dokumen_id: dokumenId,
          master_sub_kegiatan_id: Number(form.master_sub_kegiatan_id),
          pagu: form.pagu ? parseFloat(form.pagu) : null,
          sumber_dana_id: form.sumber_dana_id ? Number(form.sumber_dana_id) : null,
          lokasi: form.lokasi || null,
          waktu_pelaksanaan: form.waktu_pelaksanaan || null,
          keluaran: form.keluaran || null,
          target_keluaran: form.target_keluaran || null,
        }
      });
      dskList = await invoke<DokumenSubKegiatan[]>("get_dokumen_sub_kegiatan", { dokumenId });
      form = { master_sub_kegiatan_id: 0, pagu: "", sumber_dana_id: "", lokasi: "", waktu_pelaksanaan: "", keluaran: "", target_keluaran: "" };
      showForm = false;
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function hapus(id: number) {
    if (!confirm("Hapus sub kegiatan ini dari dokumen?")) return;
    try {
      await invoke("hapus_dokumen_sub_kegiatan", { id });
      dskList = dskList.filter(d => d.id !== id);
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="page-header">
  <div style="display:flex; align-items:center; gap:10px;">
    <a href="/dokumen" class="btn btn-ghost" style="font-size:12px;">← Dokumen</a>
    <h2>{dokumen ? dokumen.jenis + " v" + dokumen.versi : "Memuat..."}</h2>
  </div>
  {#if dokumen}
    <span class="badge badge-{dokumen.status}">{dokumen.status}</span>
  {/if}
</div>

{#if error}
  <div class="alert-error">
    {error}
    <button onclick={() => error = ""} style="margin-left:8px; text-decoration:underline; background:none; border:none; color:inherit; cursor:pointer;">Tutup</button>
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !dokumen}
  <p style="font-size:13px; color:var(--text-3);">Dokumen tidak ditemukan.</p>
{:else}
  {#if dokumen.nomor_se}
    <div style="font-size:12px; color:var(--text-3); margin-bottom:20px;">
      SE: {dokumen.nomor_se}{dokumen.tanggal_se ? " — " + dokumen.tanggal_se : ""}
    </div>
  {/if}

  <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:16px;">
    <span class="label">Sub Kegiatan</span>
    {#if masterSubTersedia().length > 0}
      <button onclick={() => showForm = !showForm} class="btn btn-primary" style="font-size:12px;">
        + Tambah Sub Kegiatan
      </button>
    {:else if masterSubList.length === 0}
      <a href="/master-data" class="btn btn-ghost" style="font-size:12px;">Atur Master Data →</a>
    {/if}
  </div>

  {#if showForm}
    <div class="card" style="padding:20px; margin-bottom:16px; max-width:600px;">
      <div style="display:flex; flex-direction:column; gap:14px;">
        <div>
          <div class="label" style="margin-bottom:6px;">Sub Kegiatan</div>
          <select class="input" bind:value={form.master_sub_kegiatan_id}>
            <option value={0}>-- Pilih Sub Kegiatan --</option>
            {#each masterSubTersedia() as sub}
              <option value={sub.id}>{sub.kode} — {sub.uraian}</option>
            {/each}
          </select>
        </div>
        <div style="display:flex; gap:12px;">
          <div style="flex:1;">
            <div class="label" style="margin-bottom:6px;">Pagu</div>
            <input type="number" placeholder="0" bind:value={form.pagu} class="input" />
          </div>
          <div style="flex:1;">
            <div class="label" style="margin-bottom:6px;">Sumber Dana <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
            <select class="input" bind:value={form.sumber_dana_id}>
              <option value="">-- Pilih --</option>
              {#each sumberDanaList as sd}
                <option value={sd.id}>{sd.kode} — {sd.nama}</option>
              {/each}
            </select>
          </div>
        </div>
        <div>
          <div class="label" style="margin-bottom:6px;">Lokasi <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
          <input type="text" placeholder="Kab. ..." bind:value={form.lokasi} class="input" />
        </div>
        <div style="display:flex; gap:12px;">
          <div style="flex:1;">
            <div class="label" style="margin-bottom:6px;">Waktu Pelaksanaan</div>
            <input type="text" placeholder="Jan — Des 2025" bind:value={form.waktu_pelaksanaan} class="input" />
          </div>
          <div style="flex:1;">
            <div class="label" style="margin-bottom:6px;">Keluaran</div>
            <input type="text" placeholder="Dokumen / Laporan / ..." bind:value={form.keluaran} class="input" />
          </div>
          <div style="flex:1;">
            <div class="label" style="margin-bottom:6px;">Target Keluaran</div>
            <input type="text" placeholder="1 Dokumen" bind:value={form.target_keluaran} class="input" />
          </div>
        </div>
        <div style="display:flex; gap:10px;">
          <button onclick={simpan} disabled={submitting || !form.master_sub_kegiatan_id} class="btn btn-primary">
            {submitting ? "Menyimpan..." : "Simpan"}
          </button>
          <button onclick={() => showForm = false} class="btn btn-ghost">Batal</button>
        </div>
      </div>
    </div>
  {/if}

  {#if dskList.length === 0}
    <div class="card" style="padding:40px; text-align:center;">
      <p style="font-size:13px; color:var(--text-3); margin-bottom:12px;">Belum ada sub kegiatan.</p>
      {#if masterSubList.length === 0}
        <p style="font-size:12px; color:var(--text-3);">Tambah sub kegiatan di <a href="/master-data" style="color:var(--accent);">Master Data</a> terlebih dahulu.</p>
      {/if}
    </div>
  {:else}
    <div class="card">
      {#each dskList as dsk, i}
        <div style="display:flex; align-items:center; justify-content:space-between; padding:14px 20px; {i < dskList.length - 1 ? "border-bottom:1px solid var(--border);" : ""}">
          <div style="flex:1; cursor:pointer;" onclick={() => goto("/dokumen/" + dokumenId + "/sub/" + dsk.id)}>
            <div style="font-size:12px; color:var(--text-3); font-family:monospace;">{namaSubKegiatan(dsk.master_sub_kegiatan_id)}</div>
            {#if dsk.pagu > 0}
              <div style="font-size:12px; color:var(--text-2); margin-top:2px;">{formatRupiah(dsk.pagu)}</div>
            {/if}
            {#if dsk.lokasi}
              <div style="font-size:11px; color:var(--text-3); margin-top:2px;">{dsk.lokasi}</div>
            {/if}
          </div>
          <div style="display:flex; gap:8px;">
            <button onclick={() => goto("/dokumen/" + dokumenId + "/sub/" + dsk.id)} class="btn btn-ghost" style="font-size:12px;">Rincian →</button>
            <button onclick={() => hapus(dsk.id)} class="btn btn-ghost" style="font-size:12px; color:var(--text-3);">Hapus</button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
{/if}
