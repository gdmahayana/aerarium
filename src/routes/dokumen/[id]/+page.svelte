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

  interface Kegiatan {
    id: number;
    dokumen_id: number;
    kode: string;
    uraian: string;
    pagu: number;
  }

  let dokumen = $state<Dokumen | null>(null);
  let kegiatanList = $state<Kegiatan[]>([]);
  let loading = $state(true);
  let error = $state("");

  let showForm = $state(false);
  let kode = $state("");
  let uraian = $state("");
  let pagu = $state("");
  let submitting = $state(false);

  const dokumenId = $derived(Number($page.params.id));

  onMount(async () => {
    try {
      const tahunList = await invoke<any[]>("get_tahun_anggaran");
      const tahunAktif = tahunList.find(t => t.aktif);
      if (!tahunAktif) return;
      const list = await invoke<Dokumen[]>("get_dokumen", { tahunAnggaranId: tahunAktif.id });
      dokumen = list.find(d => d.id === dokumenId) || null;
      if (dokumen) {
        kegiatanList = await invoke<Kegiatan[]>("get_kegiatan", { dokumenId });
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

  async function simpanKegiatan() {
    if (!kode || !uraian) return;
    submitting = true;
    try {
      await invoke("tambah_kegiatan", {
        input: {
          dokumen_id: dokumenId,
          kode,
          uraian,
          pagu: pagu ? parseFloat(pagu) : null,
        }
      });
      kegiatanList = await invoke<Kegiatan[]>("get_kegiatan", { dokumenId });
      kode = ""; uraian = ""; pagu = "";
      showForm = false;
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function hapusKegiatan(id: number) {
    if (!confirm("Hapus kegiatan ini?")) return;
    try {
      await invoke("hapus_kegiatan", { id });
      kegiatanList = kegiatanList.filter(k => k.id !== id);
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
    <span class="label">Kegiatan</span>
    <button onclick={() => showForm = !showForm} class="btn btn-primary" style="font-size:12px;">
      + Tambah Kegiatan
    </button>
  </div>

  {#if showForm}
    <div class="card" style="padding:20px; margin-bottom:16px; max-width:560px;">
      <div style="display:flex; flex-direction:column; gap:14px;">
        <div>
          <div class="label" style="margin-bottom:6px;">Kode Kegiatan</div>
          <input type="text" placeholder="2.13.05.2.01" bind:value={kode} class="input" />
        </div>
        <div>
          <div class="label" style="margin-bottom:6px;">Uraian</div>
          <input type="text" placeholder="Nama kegiatan..." bind:value={uraian} class="input" />
        </div>
        <div>
          <div class="label" style="margin-bottom:6px;">Pagu <span style="color:var(--text-3); font-weight:400;">(opsional)</span></div>
          <input type="number" placeholder="0" bind:value={pagu} class="input" style="max-width:200px;" />
        </div>
        <div style="display:flex; gap:10px;">
          <button onclick={simpanKegiatan} disabled={submitting} class="btn btn-primary">
            {submitting ? "Menyimpan..." : "Simpan"}
          </button>
          <button onclick={() => showForm = false} class="btn btn-ghost">Batal</button>
        </div>
      </div>
    </div>
  {/if}

  {#if kegiatanList.length === 0}
    <div class="card" style="padding:40px; text-align:center;">
      <p style="font-size:13px; color:var(--text-3);">Belum ada kegiatan.</p>
    </div>
  {:else}
    <div class="card">
      {#each kegiatanList as keg, i}
        <div style="display:flex; align-items:center; justify-content:space-between; padding:14px 20px; {i < kegiatanList.length - 1 ? "border-bottom:1px solid var(--border);" : ""}">
          <div style="flex:1; cursor:pointer;" onclick={() => goto("/dokumen/" + dokumenId + "/kegiatan/" + keg.id)}>
            <div style="font-size:12px; color:var(--text-3); font-family:monospace;">{keg.kode}</div>
            <div style="font-size:13px; color:var(--text); margin-top:2px;">{keg.uraian}</div>
            {#if keg.pagu > 0}
              <div style="font-size:12px; color:var(--text-2); margin-top:2px;">{formatRupiah(keg.pagu)}</div>
            {/if}
          </div>
          <div style="display:flex; gap:8px;">
            <button onclick={() => goto("/dokumen/" + dokumenId + "/kegiatan/" + keg.id)} class="btn btn-ghost" style="font-size:12px;">Buka →</button>
            <button onclick={() => hapusKegiatan(keg.id)} class="btn btn-ghost" style="font-size:12px; color:var(--text-3);">Hapus</button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
{/if}
