<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Shs {
    id: number;
    jenis: string;
    sumber: string;
    kode: string;
    kode_kelompok: string | null;
    uraian_kelompok: string | null;
    id_sipd: string | null;
    uraian: string;
    spesifikasi: string | null;
    satuan: string;
    harga: number;
    kode_rekening: string | null;
  }

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
  }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let shsList = $state<Shs[]>([]);
  let tabAktif = $state<'rencana'|'resmi'>('rencana');
  let cari = $state("");
  let filterJenis = $state("");
  let loading = $state(true);
  let error = $state("");
  let importing = $state(false);
  let halaman = $state(1);
  const perHalaman = 100;
  let showForm = $state(false);
  let submitting = $state(false);

  let form = $state({
    jenis: "SSH",
    kode: "",
    uraian: "",
    spesifikasi: "",
    satuan: "",
    harga: "",
  });

  const jenisList = ["SSH", "SBU", "HSPK", "ASB"];

  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;
      if (tahunAktif) {
        shsList = await invoke<Shs[]>("get_shs", { tahunAnggaranId: tahunAktif.id });
      }
    } catch(e) { error = String(e); }
    finally { loading = false; }
  });

  $effect(() => { cari; filterJenis; tabAktif; halaman = 1; });

  let shsTampil = $derived(
    shsFiltered.slice((halaman - 1) * perHalaman, halaman * perHalaman)
  );

  let totalHalaman = $derived(Math.ceil(shsFiltered.length / perHalaman));

  let shsFiltered = $derived(
    shsList.filter(s =>
      s.sumber === tabAktif &&
      (filterJenis === "" || s.jenis === filterJenis) &&
      (cari === "" ||
        s.uraian.toLowerCase().includes(cari.toLowerCase()) ||
        s.kode.toLowerCase().includes(cari.toLowerCase()))
    )
  );

  let totalRencana = $derived(shsList.filter(s => s.sumber === "rencana").length);
  let totalResmi = $derived(shsList.filter(s => s.sumber === "resmi").length);

  function formatRp(n: number) {
    return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(n);
  }

  async function tambah() {
    if (!form.kode || !form.uraian || !form.satuan || !form.harga || !tahunAktif) return;
    submitting = true;
    try {
      await invoke("tambah_shs", {
        tahunAnggaranId: tahunAktif.id,
        jenis: form.jenis,
        sumber: tabAktif,
        kode: form.kode,
        uraian: form.uraian,
        spesifikasi: form.spesifikasi || null,
        satuan: form.satuan,
        harga: parseFloat(form.harga),
      });
      shsList = await invoke<Shs[]>("get_shs", { tahunAnggaranId: tahunAktif.id });
      form = { jenis: "SSH", kode: "", uraian: "", spesifikasi: "", satuan: "", harga: "" };
      showForm = false;
    } catch(e) { error = String(e); }
    finally { submitting = false; }
  }

  async function hapus(id: number, uraian: string) {
    if (!confirm('Hapus ' + uraian + '?')) return;
    try {
      await invoke("hapus_shs", { id });
      shsList = shsList.filter(s => s.id !== id);
    } catch(e) { error = String(e); }
  }

  async function importExcel() {
    if (!tahunAktif) return;
    importing = true;
    try {
      const { open } = await import("@tauri-apps/plugin-dialog");
      const filePath = await open({
        filters: [{ name: "Excel", extensions: ["xlsx", "xls"] }],
      });
      if (!filePath) return;
      const count = await invoke<number>("import_shs", {
        filePath: filePath as string,
        tahunAnggaranId: tahunAktif.id,
        jenis: filterJenis || "SSH",
      });
      shsList = await invoke<Shs[]>("get_shs", { tahunAnggaranId: tahunAktif.id });
      alert("Berhasil import " + count + " item SHS Resmi");
    } catch(e) { error = String(e); }
    finally { importing = false; }
  }
</script>

<div class="page-header">
  <h2>Database SHS</h2>
  <p>Standar Satuan Harga — referensi harga untuk penyusunan rincian belanja</p>
</div>

{#if error}
  <div class="alert-error">
    {error}
    <button onclick={() => error = ""} style="margin-left:8px; text-decoration:underline; background:none; border:none; color:inherit; cursor:pointer;">Tutup</button>
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !tahunAktif}
  <div class="card" style="padding:48px; text-align:center; max-width:400px; margin:60px auto;">
    <p style="font-size:13px; color:var(--text-2); margin-bottom:16px;">Belum ada tahun anggaran aktif.</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Atur Tahun Anggaran</a>
  </div>
{:else}
  <!-- Tab -->
  <div style="display:flex; gap:0; margin-bottom:20px; border-bottom:1px solid var(--border);">
    <button
      onclick={() => { tabAktif = 'rencana'; showForm = false; }}
      style="padding:10px 20px; font-size:13px; font-weight:600; background:none; border:none; cursor:pointer; border-bottom:2px solid {tabAktif === 'rencana' ? 'var(--accent)' : 'transparent'}; color:{tabAktif === 'rencana' ? 'var(--accent)' : 'var(--text-3)'};"
    >SHS Rencana <span style="font-size:11px; margin-left:4px;">({totalRencana})</span></button>
    <button
      onclick={() => { tabAktif = 'resmi'; showForm = false; }}
      style="padding:10px 20px; font-size:13px; font-weight:600; background:none; border:none; cursor:pointer; border-bottom:2px solid {tabAktif === 'resmi' ? 'var(--accent)' : 'transparent'}; color:{tabAktif === 'resmi' ? 'var(--accent)' : 'var(--text-3)'};"
    >SHS Resmi <span style="font-size:11px; margin-left:4px;">({totalResmi})</span></button>
  </div>

  <!-- Deskripsi tab -->
  <div style="font-size:12px; color:var(--text-3); margin-bottom:16px; padding:10px 14px; background:var(--surface-2); border-radius:6px;">
    {#if tabAktif === 'rencana'}
      <strong style="color:var(--text-2);">SHS Rencana</strong> — estimasi harga OPD sebelum SHS resmi terbit.
      Input manual kebutuhan belanja, lalu export untuk dikirim ke BPKAD sebagai usulan survei harga.
    {:else}
      <strong style="color:var(--text-2);">SHS Resmi</strong> — harga yang telah ditetapkan BPKAD setelah survei.
      Input manual mengikuti dokumen resmi, atau import dari file Excel SIPD.
    {/if}
  </div>

  <!-- Toolbar -->
  <div style="display:flex; align-items:center; gap:10px; margin-bottom:16px; flex-wrap:wrap;">
    <input type="text" placeholder="Cari kode atau uraian..." bind:value={cari} class="input" style="flex:1; min-width:200px; max-width:320px;" />
    <select class="input" style="width:120px;" bind:value={filterJenis}>
      <option value="">Semua Jenis</option>
      {#each jenisList as j}
        <option value={j}>{j}</option>
      {/each}
    </select>
    <div style="flex:1;"></div>
    <button onclick={() => { showForm = !showForm; }} class="btn btn-primary" style="font-size:12px;">
      + Input Manual
    </button>
    {#if tabAktif === 'resmi'}
      <button onclick={importExcel} disabled={importing} class="btn btn-ghost" style="font-size:12px;">
        {importing ? "Mengimpor..." : "Import Excel"}
      </button>
    {/if}
  </div>

  <!-- Form input manual -->
  {#if showForm}
    <div class="card" style="padding:16px; margin-bottom:16px; border:1px solid var(--accent);">
      <div style="font-size:12px; font-weight:600; color:var(--text-2); margin-bottom:12px;">
        Tambah SHS {tabAktif === 'rencana' ? 'Rencana' : 'Resmi'} — Input Manual
      </div>
      <div style="display:flex; gap:10px; flex-wrap:wrap; align-items:flex-end;">
        <div style="flex:0 0 100px;">
          <div class="label" style="margin-bottom:4px;">Jenis</div>
          <select class="input" bind:value={form.jenis}>
            {#each jenisList as j}
              <option value={j}>{j}</option>
            {/each}
          </select>
        </div>
        <div style="flex:0 0 160px;">
          <div class="label" style="margin-bottom:4px;">Kode</div>
          <input class="input" bind:value={form.kode} placeholder="Kode SHS..." />
        </div>
        <div style="flex:2; min-width:200px;">
          <div class="label" style="margin-bottom:4px;">Uraian / Nama Barang</div>
          <input class="input" bind:value={form.uraian} placeholder="Nama barang atau jasa..." autofocus />
        </div>
        <div style="flex:1; min-width:160px;">
          <div class="label" style="margin-bottom:4px;">Spesifikasi</div>
          <input class="input" bind:value={form.spesifikasi} placeholder="Spesifikasi teknis..." />
        </div>
        <div style="flex:0 0 120px;">
          <div class="label" style="margin-bottom:4px;">Satuan</div>
          <input class="input" bind:value={form.satuan} placeholder="Unit, Kg, Lembar..." />
        </div>
        <div style="flex:0 0 160px;">
          <div class="label" style="margin-bottom:4px;">Harga</div>
          <input class="input" type="number" bind:value={form.harga} placeholder="0" />
        </div>
        <div style="display:flex; gap:8px;">
          <button onclick={tambah} disabled={submitting || !form.kode || !form.uraian || !form.satuan || !form.harga} class="btn btn-primary">
            {submitting ? "Menyimpan..." : "Simpan"}
          </button>
          <button onclick={() => showForm = false} class="btn btn-ghost">Batal</button>
        </div>
      </div>
    </div>
  {/if}

  <!-- List -->
  {#if shsTampil.length === 0}
    <div class="card" style="padding:48px; text-align:center;">
      {#if shsList.filter(s => s.sumber === tabAktif).length === 0}
        <p style="font-size:13px; color:var(--text-3); margin-bottom:16px;">
          Belum ada SHS {tabAktif === 'rencana' ? 'Rencana' : 'Resmi'}.
        </p>
        {#if tabAktif === 'rencana'}
          <p style="font-size:12px; color:var(--text-3);">Input manual barang/jasa yang dibutuhkan OPD.</p>
        {:else}
          <p style="font-size:12px; color:var(--text-3);">Input manual atau import dari Excel setelah SHS resmi terbit.</p>
        {/if}
      {:else}
        <p style="font-size:13px; color:var(--text-3);">Tidak ada hasil untuk pencarian ini.</p>
      {/if}
    </div>
  {:else}
    <div style="font-size:12px; color:var(--text-3); margin-bottom:8px;">Menampilkan {(halaman-1)*perHalaman+1}–{Math.min(halaman*perHalaman, shsFiltered.length)} dari {shsFiltered.length} item</div>
    <div class="card" style="overflow:hidden;">
      <table style="width:100%; border-collapse:collapse;">
        <thead>
          <tr style="border-bottom:1px solid var(--border);">
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Jenis</th>
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Kode</th>
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Uraian</th>
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Spesifikasi</th>
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Satuan</th>
            <th style="padding:10px 16px; text-align:right; font-size:11px; font-weight:600; color:var(--text-3); text-transform:uppercase;">Harga</th>
            <th style="padding:10px 16px;"></th>
          </tr>
        </thead>
        <tbody>
          {#each shsTampil as item, i}
            <tr style="border-bottom:1px solid var(--border); {i % 2 === 1 ? 'background:rgba(255,255,255,0.02);' : ''}">
              <td style="padding:10px 16px;"><span class="badge">{item.jenis}</span></td>
              <td style="padding:10px 16px; font-size:12px; color:var(--text-3); font-family:monospace;">{item.kode}</td>
              <td style="padding:10px 16px; font-size:13px; color:var(--text);">{item.uraian}</td>
              <td style="padding:10px 16px; font-size:12px; color:var(--text-3);">{item.spesifikasi || '—'}</td>
              <td style="padding:10px 16px; font-size:12px; color:var(--text-2);">{item.satuan}</td>
              <td style="padding:10px 16px; font-size:13px; color:var(--text); text-align:right; font-variant-numeric:tabular-nums;">{formatRp(item.harga)}</td>
              <td style="padding:10px 16px;">
                <button onclick={() => hapus(item.id, item.uraian)} class="btn btn-ghost" style="font-size:11px; color:var(--text-3);">Hapus</button>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>

    {#if totalHalaman > 1}
      <div style="display:flex; align-items:center; justify-content:center; gap:8px; margin-top:16px;">
        <button
          class="btn btn-ghost" style="font-size:12px;"
          disabled={halaman === 1}
          onclick={() => halaman--}
        >← Prev</button>

        {#each Array.from({length: totalHalaman}, (_, i) => i + 1).filter(p => p === 1 || p === totalHalaman || Math.abs(p - halaman) <= 2) as p, i}
          {#if i > 0 && p - (Array.from({length: totalHalaman}, (_, i) => i + 1).filter(p => p === 1 || p === totalHalaman || Math.abs(p - halaman) <= 2)[i-1]) > 1}
            <span style="color:var(--text-3);">...</span>
          {/if}
          <button
            class="btn {halaman === p ? 'btn-primary' : 'btn-ghost'}" style="font-size:12px; min-width:36px;"
            onclick={() => halaman = p}
          >{p}</button>
        {/each}

        <button
          class="btn btn-ghost" style="font-size:12px;"
          disabled={halaman === totalHalaman}
          onclick={() => halaman++}
        >Next →</button>
      </div>
    {/if}
  {/if}
{/if}
