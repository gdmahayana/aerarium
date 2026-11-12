<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { save } from "@tauri-apps/plugin-dialog";
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
  let editingId = $state<number | null>(null);
  let showDetail = $state(false);
  let cariSub = $state("");

  let form = $state({
    master_sub_kegiatan_id: 0,
    pagu: "",
    sumber_dana_id: "",
    lokasi: "",
    waktu_pelaksanaan: "",
    keluaran: "",
    target_keluaran: "",
  });

  let editForm = $state({
    pagu: "",
    sumber_dana_id: "",
    lokasi: "",
    waktu_pelaksanaan: "",
    keluaran: "",
    target_keluaran: "",
  });

  const dokumenId = $derived(Number($page.params.id));

  let tahunAktifId = $state(0);

  onMount(async () => {
    try {
      const tahunList = await invoke<any[]>("get_tahun_anggaran");
      const tahunAktif = tahunList.find((t: any) => t.aktif);
      if (!tahunAktif) return;
      tahunAktifId = tahunAktif.id;

      const list = await invoke<Dokumen[]>("get_dokumen", { tahunAnggaranId: tahunAktif.id });
      dokumen = list.find(d => d.id === dokumenId) || null;

      if (dokumen) {
        [dskList, masterSubList, sumberDanaList] = await Promise.all([
          invoke<DokumenSubKegiatan[]>("get_dokumen_sub_kegiatan", { dokumenId }),
          invoke<MasterSubKegiatan[]>("get_master_sub_kegiatan_aktif"),
          invoke<SumberDana[]>("get_sumber_dana", { tahunAnggaranId: tahunAktif.id }),
        ]);
        await muatStatus();
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

  function namaSubKegiatan(id: number) {
    const sub = masterSubList.find(s => s.id === id);
    return sub ? sub.kode + " — " + sub.uraian : String(id);
  }

  function kodeSubKegiatan(id: number) {
    return masterSubList.find(s => s.id === id)?.kode || "";
  }

  function masterSubTersedia() {
    const sudahAda = new Set(dskList.map(d => d.master_sub_kegiatan_id));
    return masterSubList.filter(s => !sudahAda.has(s.id));
  }

  let subTersediaFiltered = $derived(
    masterSubTersedia().filter(s =>
      cariSub === "" ||
      s.kode.toLowerCase().includes(cariSub.toLowerCase()) ||
      s.uraian.toLowerCase().includes(cariSub.toLowerCase())
    )
  );

  function bukaForm() {
    showForm = true;
    showDetail = false;
    cariSub = "";
    form = { master_sub_kegiatan_id: 0, pagu: "", sumber_dana_id: "", lokasi: "", waktu_pelaksanaan: "", keluaran: "", target_keluaran: "" };
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
      showForm = false;
      await muatStatus();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  function bukaEdit(dsk: DokumenSubKegiatan) {
    editingId = dsk.id;
    editForm = {
      pagu: dsk.pagu ? String(dsk.pagu) : "",
      sumber_dana_id: dsk.sumber_dana_id ? String(dsk.sumber_dana_id) : "",
      lokasi: dsk.lokasi || "",
      waktu_pelaksanaan: dsk.waktu_pelaksanaan || "",
      keluaran: dsk.keluaran || "",
      target_keluaran: dsk.target_keluaran || "",
    };
  }

  async function simpanEdit(dsk: DokumenSubKegiatan) {
    submitting = true;
    try {
      await invoke("update_dokumen_sub_kegiatan", {
        id: dsk.id,
        pagu: editForm.pagu ? parseFloat(editForm.pagu) : 0,
        sumberDanaId: editForm.sumber_dana_id ? Number(editForm.sumber_dana_id) : null,
        lokasi: editForm.lokasi || null,
        waktuPelaksanaan: editForm.waktu_pelaksanaan || null,
        keluaran: editForm.keluaran || null,
        targetKeluaran: editForm.target_keluaran || null,
      });
      dskList = await invoke<DokumenSubKegiatan[]>("get_dokumen_sub_kegiatan", { dokumenId });
      editingId = null;
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
      if (editingId === id) editingId = null;
    } catch (e) {
      error = String(e);
    }
  }

  let totalPagu = $derived(dskList.reduce((s, d) => s + d.pagu, 0));
  let statusRincian = $state<Record<number, boolean>>({});

  async function muatStatus() {
    try {
      const hasil = await invoke<[number, boolean][]>("cek_status_rincian", { dokumenId });
      const map: Record<number, boolean> = {};
      for (const [id, adaWarning] of hasil) map[id] = adaWarning;
      statusRincian = map;
    } catch(e) {}
  }

  let exporting = $state(false);
  let exportMsg = $state("");

  async function jadikanRka() {
    const adaWarning = Object.values(statusRincian).some(v => v);
    if (adaWarning) {
      error = "Ada sub kegiatan dengan item belum SHS Resmi. Selesaikan terlebih dahulu.";
      return;
    }
    if (!confirm("Jadikan Kertas Kerja ini sebagai RKA-SKPD? Semua rincian akan disalin.")) return;
    try {
      const rkaId = await invoke<number>("jadikan_rka", { kertasKerjaId: dokumenId });
      goto("/dokumen/" + rkaId);
    } catch(e) {
      error = String(e);
    }
  }

  async function exportKertasKerja() {
    if (!dokumen) return;
    try {
      exporting = true;
      exportMsg = "";
      const outputPath = await save({
        defaultPath: `Kertas_Kerja_${dokumen.jenis}_v${dokumen.versi}.xlsx`,
        filters: [{ name: "Excel", extensions: ["xlsx"] }],
      });
      if (!outputPath) return;
      await invoke("export_kertas_kerja", { dokumenId, outputPath });
      exportMsg = "✅ File berhasil diekspor.";
    } catch (e) {
      exportMsg = "❌ " + String(e);
    } finally {
      exporting = false;
      setTimeout(() => exportMsg = "", 4000);
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
    <button
      onclick={exportKertasKerja}
      disabled={exporting || dskList.length === 0}
      class="btn btn-ghost"
      style="font-size:12px; margin-left:auto;"
    >
      {exporting ? "Mengekspor..." : "⬇ Export Excel"}
    </button>
    {#if dokumen.jenis === "Kertas Kerja" && (dokumen.status === "draft" || dokumen.status === "konsep")}
      <button
        onclick={jadikanRka}
        class="btn btn-primary"
        style="font-size:12px;"
      >
        → Jadikan RKA
      </button>
    {/if}
  {/if}
</div>

{#if exportMsg}
  <div style="font-size:12px; padding:8px 14px; border-radius:6px; margin-bottom:12px; background:var(--surface-2); color:var(--text-2);">
    {exportMsg}
  </div>
{/if}

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

  <!-- Header -->
  <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:16px;">
    <div>
      <span class="label">Sub Kegiatan</span>
      {#if totalPagu > 0}
        <span style="font-size:12px; color:var(--text-2); margin-left:10px;">Total pagu: {formatRupiah(totalPagu)}</span>
      {/if}
    </div>
    {#if !showForm && masterSubTersedia().length > 0}
      <button onclick={bukaForm} class="btn btn-primary" style="font-size:12px;">+ Tambah Sub Kegiatan</button>
    {:else if masterSubList.length === 0}
      <a href="/master-data" class="btn btn-ghost" style="font-size:12px;">Atur Master Data →</a>
    {/if}
  </div>

  <!-- Form tambah -->
  {#if showForm}
    <div class="card" style="padding:20px; margin-bottom:20px; border:1px solid var(--accent); max-width:640px;">
      <!-- Search sub kegiatan -->
      <div style="margin-bottom:14px;">
        <div class="label" style="margin-bottom:6px;">Cari Sub Kegiatan</div>
        <input
          class="input" type="text"
          placeholder="Ketik kode atau nama..."
          bind:value={cariSub}
          autofocus
        />
      </div>

      <!-- List pilihan -->
      {#if cariSub || !form.master_sub_kegiatan_id}
        <div style="border:1px solid var(--border); border-radius:6px; max-height:200px; overflow-y:auto; margin-bottom:14px;">
          {#if subTersediaFiltered.length === 0}
            <div style="padding:16px; font-size:12px; color:var(--text-3); text-align:center;">Tidak ada hasil</div>
          {:else}
            {#each subTersediaFiltered as sub}
              <button
                style="width:100%; text-align:left; padding:10px 14px; background:{form.master_sub_kegiatan_id === sub.id ? "var(--accent)" : "transparent"}; border:none; border-bottom:1px solid var(--border); cursor:pointer; display:flex; gap:12px; align-items:baseline;"
                onclick={() => { form.master_sub_kegiatan_id = sub.id; cariSub = ""; }}
              >
                <span style="font-size:11px; font-family:monospace; color:{form.master_sub_kegiatan_id === sub.id ? "white" : "var(--text-3)"}; min-width:140px;">{sub.kode}</span>
                <span style="font-size:12px; color:{form.master_sub_kegiatan_id === sub.id ? "white" : "var(--text)"}">{sub.uraian}</span>
              </button>
            {/each}
          {/if}
        </div>
      {/if}

      <!-- Terpilih -->
      {#if form.master_sub_kegiatan_id}
        <div style="padding:10px 14px; background:var(--surface-2); border-radius:6px; margin-bottom:14px; display:flex; justify-content:space-between; align-items:center;">
          <div>
            <div style="font-size:11px; color:var(--text-3); margin-bottom:2px;">Terpilih</div>
            <div style="font-size:12px; color:var(--text); font-weight:600;">{namaSubKegiatan(form.master_sub_kegiatan_id)}</div>
          </div>
          <button class="btn btn-ghost" style="font-size:11px;" onclick={() => { form.master_sub_kegiatan_id = 0; }}>Ganti</button>
        </div>
      {/if}

      <!-- Toggle detail -->
      <button
        class="btn btn-ghost" style="font-size:12px; margin-bottom:14px; padding:4px 0;"
        onclick={() => showDetail = !showDetail}
      >{showDetail ? "▾" : "▸"} {showDetail ? "Sembunyikan detail" : "Isi pagu & detail (opsional)"}</button>

      {#if showDetail}
        <div style="display:flex; flex-direction:column; gap:12px; margin-bottom:14px; padding:14px; background:var(--surface-2); border-radius:6px;">
          <div style="display:flex; gap:12px;">
            <div style="flex:1;">
              <div class="label" style="margin-bottom:6px;">Pagu</div>
              <input type="number" placeholder="0" bind:value={form.pagu} class="input" />
            </div>
            <div style="flex:1;">
              <div class="label" style="margin-bottom:6px;">Sumber Dana</div>
              <select class="input" bind:value={form.sumber_dana_id}>
                <option value="">— pilih —</option>
                {#each sumberDanaList as sd}
                  <option value={sd.id}>{sd.kode} — {sd.nama}</option>
                {/each}
              </select>
            </div>
          </div>
          <div>
            <div class="label" style="margin-bottom:6px;">Lokasi</div>
            <input type="text" placeholder="Kab. ..." bind:value={form.lokasi} class="input" />
          </div>
          <div style="display:flex; gap:12px;">
            <div style="flex:1;">
              <div class="label" style="margin-bottom:6px;">Waktu Pelaksanaan</div>
              <input type="text" placeholder="Jan — Des 2025" bind:value={form.waktu_pelaksanaan} class="input" />
            </div>
            <div style="flex:1;">
              <div class="label" style="margin-bottom:6px;">Keluaran</div>
              <input type="text" placeholder="Dokumen / Laporan" bind:value={form.keluaran} class="input" />
            </div>
            <div style="flex:1;">
              <div class="label" style="margin-bottom:6px;">Target Keluaran</div>
              <input type="text" placeholder="1 Dokumen" bind:value={form.target_keluaran} class="input" />
            </div>
          </div>
        </div>
      {/if}

      <div style="display:flex; gap:10px;">
        <button onclick={simpan} disabled={submitting || !form.master_sub_kegiatan_id} class="btn btn-primary">
          {submitting ? "Menyimpan..." : "Simpan"}
        </button>
        <button onclick={() => showForm = false} class="btn btn-ghost">Batal</button>
      </div>
    </div>
  {/if}

  <!-- List sub kegiatan -->
  {#if dskList.length === 0}
    <div class="card" style="padding:40px; text-align:center;">
      <p style="font-size:13px; color:var(--text-3); margin-bottom:12px;">Belum ada sub kegiatan.</p>
      {#if masterSubList.length === 0}
        <p style="font-size:12px; color:var(--text-3);">
          Tambah sub kegiatan di <a href="/master-data" style="color:var(--accent);">Master Data</a> terlebih dahulu.
        </p>
      {/if}
    </div>
  {:else}
    <div style="display:flex; flex-direction:column; gap:8px;">
      {#each dskList as dsk}
        <div class="card" style="overflow:hidden;">
          <!-- Baris utama -->
          <div style="display:flex; align-items:center; gap:12px; padding:14px 20px;">
            <div style="flex:1; cursor:pointer;" onclick={() => goto("/dokumen/" + dokumenId + "/sub/" + dsk.id)}>
              <div style="display:flex; align-items:center; gap:8px;">
                <span style="font-size:11px; font-family:monospace; color:var(--text-3);">{kodeSubKegiatan(dsk.master_sub_kegiatan_id)}</span>
                {#if statusRincian[dsk.id]}
                  <span title="Ada item belum SHS Resmi" style="font-size:11px; background:#fff3cd; color:#856404; border:1px solid #ffc107; border-radius:4px; padding:1px 6px;">⚠️ Belum Resmi</span>
                {/if}
              </div>
              <div style="font-size:13px; font-weight:600; color:var(--text); margin-top:2px;">{masterSubList.find(s => s.id === dsk.master_sub_kegiatan_id)?.uraian || ""}</div>
              <div style="display:flex; gap:16px; margin-top:4px;">
                {#if dsk.pagu > 0}
                  <span style="font-size:12px; color:var(--text-2);">{formatRupiah(dsk.pagu)}</span>
                {:else}
                  <span style="font-size:12px; color:var(--text-3);">pagu belum diisi</span>
                {/if}
                {#if dsk.lokasi}
                  <span style="font-size:11px; color:var(--text-3);">📍 {dsk.lokasi}</span>
                {/if}
                {#if dsk.waktu_pelaksanaan}
                  <span style="font-size:11px; color:var(--text-3);">🗓 {dsk.waktu_pelaksanaan}</span>
                {/if}
              </div>
            </div>
            <div style="display:flex; gap:6px;">
              <button onclick={() => bukaEdit(dsk)} class="btn btn-ghost" style="font-size:12px;">Edit</button>
              <button onclick={() => goto("/dokumen/" + dokumenId + "/sub/" + dsk.id)} class="btn btn-primary" style="font-size:12px;">Rincian →</button>
              <button onclick={() => hapus(dsk.id)} class="btn btn-ghost" style="font-size:12px; color:var(--text-3);">Hapus</button>
            </div>
          </div>

          <!-- Panel edit inline -->
          {#if editingId === dsk.id}
            <div style="border-top:1px solid var(--border); padding:16px 20px; background:var(--surface-2); display:flex; flex-direction:column; gap:12px;">
              <div style="display:flex; gap:12px;">
                <div style="flex:1;">
                  <div class="label" style="margin-bottom:6px;">Pagu</div>
                  <input type="number" placeholder="0" bind:value={editForm.pagu} class="input" />
                </div>
                <div style="flex:1;">
                  <div class="label" style="margin-bottom:6px;">Sumber Dana</div>
                  <select class="input" bind:value={editForm.sumber_dana_id}>
                    <option value="">— pilih —</option>
                    {#each sumberDanaList as sd}
                      <option value={sd.id}>{sd.kode} — {sd.nama}</option>
                    {/each}
                  </select>
                </div>
              </div>
              <div>
                <div class="label" style="margin-bottom:6px;">Lokasi</div>
                <input type="text" placeholder="Kab. ..." bind:value={editForm.lokasi} class="input" />
              </div>
              <div style="display:flex; gap:12px;">
                <div style="flex:1;">
                  <div class="label" style="margin-bottom:6px;">Waktu Pelaksanaan</div>
                  <input type="text" bind:value={editForm.waktu_pelaksanaan} class="input" />
                </div>
                <div style="flex:1;">
                  <div class="label" style="margin-bottom:6px;">Keluaran</div>
                  <input type="text" bind:value={editForm.keluaran} class="input" />
                </div>
                <div style="flex:1;">
                  <div class="label" style="margin-bottom:6px;">Target Keluaran</div>
                  <input type="text" bind:value={editForm.target_keluaran} class="input" />
                </div>
              </div>
              <div style="display:flex; gap:8px;">
                <button class="btn btn-primary" style="font-size:12px;" disabled={submitting} onclick={() => simpanEdit(dsk)}>{submitting ? "Menyimpan..." : "Simpan"}</button>
                <button class="btn btn-ghost" style="font-size:12px;" onclick={() => editingId = null}>Batal</button>
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
{/if}
