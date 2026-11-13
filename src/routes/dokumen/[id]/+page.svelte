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
  interface SumberDana { id: number; kode: string; nama: string; }

  const dokumenId = $derived(Number($page.params.id));

  let dokumen       = $state<Dokumen | null>(null);
  let dskList       = $state<DokumenSubKegiatan[]>([]);
  let masterSubList = $state<MasterSubKegiatan[]>([]);
  let sumberDanaList= $state<SumberDana[]>([]);
  let loading       = $state(true);
  let error         = $state("");
  let showForm      = $state(false);
  let submitting    = $state(false);
  let editingId     = $state<number | null>(null);
  let showDetail    = $state(false);
  let cariSub       = $state("");
  let tahunAktifId  = $state(0);
  let statusRincian = $state<Record<number, boolean>>({});
  let totalRincian  = $state<Record<number, number>>({});
  let exporting     = $state(false);
  let exportMsg     = $state("");

  let form = $state({
    master_sub_kegiatan_id: 0, pagu: "", sumber_dana_id: "",
    lokasi: "", waktu_pelaksanaan: "", keluaran: "", target_keluaran: "",
  });
  let editForm = $state({
    pagu: "", sumber_dana_id: "", lokasi: "",
    waktu_pelaksanaan: "", keluaran: "", target_keluaran: "",
  });

  // Konfigurasi per jenis dokumen
  const jenisConfig: Record<string, {
    warna: string; warnaGlow: string; icon: string;
    tagline: string; adaSE: boolean;
  }> = {
    "Kertas Kerja":        { warna: "#6366f1", warnaGlow: "#6366f115", icon: "✦", tagline: "Survei kebutuhan & estimasi harga", adaSE: false },
    "Pagu Indikatif":      { warna: "#0ea5e9", warnaGlow: "#0ea5e915", icon: "◈", tagline: "Alokasi Bappeda · SHS Resmi", adaSE: false },
    "RKA-SKPD":            { warna: "#10b981", warnaGlow: "#10b98115", icon: "▸", tagline: "Rencana Kerja & Anggaran", adaSE: true },
    "DPA-SKPD":            { warna: "#f59e0b", warnaGlow: "#f59e0b15", icon: "▸", tagline: "Dokumen Pelaksanaan Anggaran", adaSE: true },
    "RKA-SKPD Perubahan":  { warna: "#10b981", warnaGlow: "#10b98115", icon: "▸", tagline: "RKA Perubahan", adaSE: true },
    "DPPA-SKPD":           { warna: "#f59e0b", warnaGlow: "#f59e0b15", icon: "▸", tagline: "Dokumen Pelaksanaan Perubahan Anggaran", adaSE: true },
  };

  const cfg = $derived(dokumen ? (jenisConfig[dokumen.jenis] || jenisConfig["RKA-SKPD"]) : jenisConfig["RKA-SKPD"]);

  const statusStyle: Record<string, {bg: string, color: string}> = {
    draft:      { bg: "#1e293b", color: "#94a3b8" },
    konsep:     { bg: "#0c2a4a", color: "#7dd3fc" },
    ditetapkan: { bg: "#052e16", color: "#4ade80" },
    dikunci:    { bg: "#2d1f00", color: "#fbbf24" },
    diarsipkan: { bg: "#1a1a2e", color: "#6b7280" },
  };

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
    } catch(e) { error = String(e); }
    finally { loading = false; }
  });

  async function muatStatus() {
    try {
      const hasil = await invoke<[number, boolean][]>("cek_status_rincian", { dokumenId });
      const map: Record<number, boolean> = {};
      for (const [id, adaWarning] of hasil) map[id] = adaWarning;
      statusRincian = map;
    } catch(e) {}
  }

  function formatRp(n: number) {
    if (n >= 1_000_000_000) return "Rp " + (n/1_000_000_000).toFixed(2) + " M";
    if (n >= 1_000_000)     return "Rp " + (n/1_000_000).toFixed(2) + " jt";
    return new Intl.NumberFormat("id-ID", { style:"currency", currency:"IDR", minimumFractionDigits:0 }).format(n);
  }

  function namaSubKegiatan(id: number) {
    const sub = masterSubList.find(s => s.id === id);
    return sub ? sub.kode + " — " + sub.uraian : String(id);
  }
  function kodeSubKegiatan(id: number) {
    return masterSubList.find(s => s.id === id)?.kode || "";
  }
  function uraianSubKegiatan(id: number) {
    return masterSubList.find(s => s.id === id)?.uraian || "";
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

  let totalPagu = $derived(dskList.reduce((s, d) => s + d.pagu, 0));
  let adaWarningGlobal = $derived(Object.values(statusRincian).some(v => v));

  function bukaForm() {
    showForm = true; showDetail = false; cariSub = "";
    form = { master_sub_kegiatan_id: 0, pagu: "", sumber_dana_id: "", lokasi: "", waktu_pelaksanaan: "", keluaran: "", target_keluaran: "" };
  }

  async function simpan() {
    if (!form.master_sub_kegiatan_id) return;
    submitting = true;
    try {
      await invoke("tambah_dokumen_sub_kegiatan", { input: {
        dokumen_id: dokumenId,
        master_sub_kegiatan_id: Number(form.master_sub_kegiatan_id),
        pagu: form.pagu ? parseFloat(form.pagu) : null,
        sumber_dana_id: form.sumber_dana_id ? Number(form.sumber_dana_id) : null,
        lokasi: form.lokasi || null,
        waktu_pelaksanaan: form.waktu_pelaksanaan || null,
        keluaran: form.keluaran || null,
        target_keluaran: form.target_keluaran || null,
      }});
      dskList = await invoke<DokumenSubKegiatan[]>("get_dokumen_sub_kegiatan", { dokumenId });
      showForm = false;
      await muatStatus();
    } catch(e) { error = String(e); }
    finally { submitting = false; }
  }

  function bukaEdit(dsk: DokumenSubKegiatan) {
    editingId = dsk.id;
    editForm = {
      pagu: dsk.pagu ? String(dsk.pagu) : "",
      sumber_dana_id: dsk.sumber_dana_id ? String(dsk.sumber_dana_id) : "",
      lokasi: dsk.lokasi || "", waktu_pelaksanaan: dsk.waktu_pelaksanaan || "",
      keluaran: dsk.keluaran || "", target_keluaran: dsk.target_keluaran || "",
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
      await muatStatus();
    } catch(e) { error = String(e); }
    finally { submitting = false; }
  }

  async function hapus(id: number) {
    if (!confirm("Hapus sub kegiatan ini dari dokumen?")) return;
    try {
      await invoke("hapus_dokumen_sub_kegiatan", { id });
      dskList = dskList.filter(d => d.id !== id);
      if (editingId === id) editingId = null;
    } catch(e) { error = String(e); }
  }

  async function jadikanRka() {
    if (adaWarningGlobal) {
      error = "Ada sub kegiatan dengan item belum SHS Resmi.";
      return;
    }
    if (!confirm("Jadikan dokumen ini sebagai RKA-SKPD? Semua rincian akan disalin.")) return;
    try {
      const rkaId = await invoke<number>("jadikan_rka", { kertasKerjaId: dokumenId });
      goto("/dokumen/" + rkaId);
    } catch(e) { error = String(e); }
  }

  async function jadikanDpa() {
    if (!confirm('Jadikan RKA-SKPD ini sebagai DPA-SKPD? Semua rincian akan disalin.')) return;
    try {
      const id = await invoke<number>('jadikan_dpa', { rkaId: dokumenId });
      goto('/dokumen/' + id);
    } catch(e) { error = String(e); }
  }

  async function jadikanDppa() {
    if (!confirm('Jadikan RKA-SKPD Perubahan ini sebagai DPPA-SKPD? Semua rincian akan disalin.')) return;
    try {
      const id = await invoke<number>('jadikan_dppa', { rkapId: dokumenId });
      goto('/dokumen/' + id);
    } catch(e) { error = String(e); }
  }

  async function buatPergeseran() {
    if (!confirm('Buat versi revisi baru? Dokumen ini akan dikunci dan rinciannya disalin ke versi baru.')) return;
    try {
      const id = await invoke<number>('buat_pergeseran', { dokumenId });
      goto('/dokumen/' + id);
    } catch(e) { error = String(e); }
  }

  async function exportExcel() {
    if (!dokumen) return;
    try {
      exporting = true; exportMsg = "";
      const outputPath = await save({
        defaultPath: `${dokumen.jenis}_v${dokumen.versi}.xlsx`,
        filters: [{ name: "Excel", extensions: ["xlsx"] }],
      });
      if (!outputPath) return;
      await invoke("export_kertas_kerja", { dokumenId, outputPath });
      exportMsg = "✅ Berhasil diekspor.";
    } catch(e) { exportMsg = "❌ " + String(e); }
    finally {
      exporting = false;
      setTimeout(() => exportMsg = "", 4000);
    }
  }
</script>

<!-- ═══ HEADER ═══════════════════════════════════════════════════════════════ -->
{#if dokumen}
  <div style="
    background:{cfg.warnaGlow};
    border-bottom:1px solid {cfg.warna}22;
    padding:20px 32px 0;
    margin:-32px -32px 28px;
  ">
    <!-- Breadcrumb -->
    <div style="display:flex; align-items:center; gap:8px; margin-bottom:16px;">
      <a href="/dokumen" style="font-size:12px; color:var(--text-3);">Dokumen</a>
      <span style="color:var(--text-3); font-size:12px;">›</span>
      <span style="font-size:12px; color:{cfg.warna};">{dokumen.jenis}</span>
    </div>

    <!-- Judul + meta -->
    <div style="display:flex; align-items:start; justify-content:space-between; padding-bottom:20px;">
      <div style="display:flex; align-items:center; gap:14px;">
        <div style="
          width:44px; height:44px; border-radius:10px;
          background:{cfg.warna}22; border:1px solid {cfg.warna}44;
          display:flex; align-items:center; justify-content:center;
          font-size:18px; font-weight:800; color:{cfg.warna};
        ">{cfg.icon}</div>
        <div>
          <div style="display:flex; align-items:center; gap:10px;">
            <h2 style="font-size:18px; color:var(--text);">{dokumen.jenis}</h2>
            <span style="
              font-size:11px; font-weight:700; font-family:monospace;
              color:{cfg.warna}; background:{cfg.warna}18;
              padding:2px 7px; border-radius:4px;
            ">v{dokumen.versi}</span>
            {#if dokumen.status}
              {@const st = statusStyle[dokumen.status] || statusStyle.draft}
              <span style="
                font-size:10px; font-weight:700; padding:2px 8px; border-radius:4px;
                background:{st.bg}; color:{st.color}; letter-spacing:0.05em; text-transform:uppercase;
              ">{dokumen.status}</span>
            {/if}
          </div>
          <div style="font-size:12px; color:var(--text-3); margin-top:3px;">
            {cfg.tagline}
            {#if dokumen.nomor_se}
              <span style="margin-left:12px; color:var(--text-2);">SE: {dokumen.nomor_se}</span>
            {/if}
            {#if dokumen.tanggal_se}
              <span style="margin-left:8px; color:var(--text-3);">{dokumen.tanggal_se}</span>
            {/if}
          </div>
        </div>
      </div>

      <!-- Action buttons kontekstual -->
      <div style="display:flex; gap:8px; align-items:center;">
        {#if dokumen.jenis === "Kertas Kerja"}
          <a href="/dokumen/{dokumenId}/petakan" class="btn btn-ghost" style="font-size:12px;">🔄 Petakan SHS</a>
        {/if}
        <button
          onclick={exportExcel}
          disabled={exporting || dskList.length === 0}
          class="btn btn-ghost" style="font-size:12px;"
        >{exporting ? "..." : "⬇ Export Excel"}</button>
        {#if dokumen.jenis === "Pagu Indikatif" && (dokumen.status === "draft" || dokumen.status === "konsep")}
          <button
            onclick={jadikanRka}
            class="btn btn-primary" style="font-size:12px;"
            title={adaWarningGlobal ? "Ada item belum SHS Resmi" : ""}
          >
            {adaWarningGlobal ? "⚠️ Jadikan RKA" : "→ Jadikan RKA"}
          </button>
        {/if}
      </div>
    </div>

    <!-- Stats bar -->
    {#if dskList.length > 0}
      <div style="display:flex; gap:28px; padding-bottom:16px; border-top:1px solid {cfg.warna}15; padding-top:14px;">
        <div>
          <div style="font-size:10px; color:var(--text-3); text-transform:uppercase; letter-spacing:0.08em; margin-bottom:3px;">Sub Kegiatan</div>
          <div style="font-size:18px; font-weight:700; color:var(--text);">{dskList.length}</div>
        </div>
        <div>
          <div style="font-size:10px; color:var(--text-3); text-transform:uppercase; letter-spacing:0.08em; margin-bottom:3px;">Total Pagu</div>
          <div style="font-size:18px; font-weight:700; color:{cfg.warna};">{formatRp(totalPagu)}</div>
        </div>
        {#if adaWarningGlobal}
          <div>
            <div style="font-size:10px; color:#856404; text-transform:uppercase; letter-spacing:0.08em; margin-bottom:3px;">Belum SHS Resmi</div>
            <div style="font-size:18px; font-weight:700; color:#fbbf24;">
              {Object.values(statusRincian).filter(v => v).length} sub keg
            </div>
          </div>
        {/if}
      </div>
    {/if}
  </div>
{/if}

<!-- ═══ NOTIFIKASI ══════════════════════════════════════════════════════════ -->
{#if exportMsg}
  <div style="font-size:12px; padding:8px 14px; border-radius:6px; margin-bottom:16px; background:var(--surface-2); color:var(--text-2);">
    {exportMsg}
  </div>
{/if}
{#if error}
  <div class="alert-error">
    {error}
    <button onclick={() => error=""} style="margin-left:8px; text-decoration:underline; background:none; border:none; color:inherit; cursor:pointer;">Tutup</button>
  </div>
{/if}

<!-- ═══ KONTEN ══════════════════════════════════════════════════════════════ -->
{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !dokumen}
  <p style="font-size:13px; color:var(--text-3);">Dokumen tidak ditemukan.</p>
{:else}
  <!-- Toolbar sub kegiatan -->
  <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:16px;">
    <span class="label">Sub Kegiatan</span>
    <div style="display:flex; gap:8px;">
      {#if !showForm && masterSubTersedia().length > 0}
        <button onclick={bukaForm} class="btn btn-primary" style="font-size:12px;">+ Tambah Sub Kegiatan</button>
      {:else if masterSubList.length === 0}
        <a href="/master-data" class="btn btn-ghost" style="font-size:12px;">Atur Master Data →</a>
      {/if}
    </div>
  </div>

  <!-- Form tambah -->
  {#if showForm}
    <div class="card" style="padding:20px; margin-bottom:20px; border:1px solid {cfg.warna}44; max-width:640px;">
      <div style="margin-bottom:14px;">
        <div class="label" style="margin-bottom:6px;">Cari Sub Kegiatan</div>
        <input class="input" type="text" placeholder="Ketik kode atau nama..." bind:value={cariSub} autofocus />
      </div>

      {#if cariSub || !form.master_sub_kegiatan_id}
        <div style="border:1px solid var(--border); border-radius:6px; max-height:200px; overflow-y:auto; margin-bottom:14px;">
          {#if subTersediaFiltered.length === 0}
            <div style="padding:16px; font-size:12px; color:var(--text-3); text-align:center;">Tidak ada hasil</div>
          {:else}
            {#each subTersediaFiltered as sub}
              <button
                style="width:100%; text-align:left; padding:10px 14px; background:{form.master_sub_kegiatan_id === sub.id ? cfg.warna + '22' : 'transparent'}; border:none; border-bottom:1px solid var(--border); cursor:pointer; display:flex; gap:12px; align-items:baseline;"
                onclick={() => { form.master_sub_kegiatan_id = sub.id; cariSub = ""; }}
              >
                <span style="font-size:11px; font-family:monospace; color:var(--text-3); min-width:140px;">{sub.kode}</span>
                <span style="font-size:12px; color:var(--text);">{sub.uraian}</span>
              </button>
            {/each}
          {/if}
        </div>
      {/if}

      {#if form.master_sub_kegiatan_id}
        <div style="padding:10px 14px; background:{cfg.warna}10; border:1px solid {cfg.warna}33; border-radius:6px; margin-bottom:14px; display:flex; justify-content:space-between; align-items:center;">
          <div>
            <div style="font-size:11px; color:var(--text-3); margin-bottom:2px;">Terpilih</div>
            <div style="font-size:12px; color:var(--text); font-weight:600;">{namaSubKegiatan(form.master_sub_kegiatan_id)}</div>
          </div>
          <button class="btn btn-ghost" style="font-size:11px;" onclick={() => form.master_sub_kegiatan_id = 0}>Ganti</button>
        </div>
      {/if}

      <button class="btn btn-ghost" style="font-size:12px; margin-bottom:14px; padding:4px 0;" onclick={() => showDetail = !showDetail}>
        {showDetail ? "▾ Sembunyikan detail" : "▸ Isi pagu & detail (opsional)"}
      </button>

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
    <div class="card" style="padding:48px; text-align:center;">
      <div style="font-size:28px; margin-bottom:12px;">{cfg.icon}</div>
      <p style="font-size:13px; color:var(--text-2); margin-bottom:4px;">Belum ada sub kegiatan.</p>
      <p style="font-size:12px; color:var(--text-3);">
        {#if masterSubList.length === 0}
          Tambah sub kegiatan di <a href="/master-data" style="color:{cfg.warna};">Master Data</a> terlebih dahulu.
        {:else}
          Klik "+ Tambah Sub Kegiatan" untuk memulai.
        {/if}
      </p>
    </div>
  {:else}
    <div style="display:flex; flex-direction:column; gap:8px;">
      {#each dskList as dsk}
        {@const warning = statusRincian[dsk.id]}
        <div class="card" style="overflow:hidden; border-left:3px solid {warning ? '#fbbf24' : cfg.warna}22;">

          <!-- Baris utama -->
          <div style="display:flex; align-items:center; gap:12px; padding:14px 20px;">
            <div style="flex:1; cursor:pointer; min-width:0;" onclick={() => goto("/dokumen/" + dokumenId + "/sub/" + dsk.id)}>

              <!-- Kode + warning -->
              <div style="display:flex; align-items:center; gap:8px; margin-bottom:4px;">
                <span style="font-size:10px; font-family:monospace; color:var(--text-3); background:var(--surface-2); padding:1px 6px; border-radius:3px;">
                  {kodeSubKegiatan(dsk.master_sub_kegiatan_id)}
                </span>
                {#if warning}
                  <span style="font-size:10px; background:#2d1f00; color:#fbbf24; border:1px solid #4a3200; border-radius:3px; padding:1px 6px;">
                    ⚠ Belum Resmi
                  </span>
                {/if}
              </div>

              <!-- Nama -->
              <div style="font-size:13px; font-weight:600; color:var(--text); margin-bottom:6px; white-space:nowrap; overflow:hidden; text-overflow:ellipsis;">
                {uraianSubKegiatan(dsk.master_sub_kegiatan_id)}
              </div>

              <!-- Meta -->
              <div style="display:flex; gap:14px; flex-wrap:wrap;">
                {#if dsk.pagu > 0}
                  <span style="font-size:12px; color:{cfg.warna}; font-weight:600;">{formatRp(dsk.pagu)}</span>
                {:else}
                  <span style="font-size:12px; color:var(--text-3); font-style:italic;">pagu belum diisi</span>
                {/if}
                {#if dsk.sumber_dana_id}
                  {@const sd = sumberDanaList.find(s => s.id === dsk.sumber_dana_id)}
                  {#if sd}<span style="font-size:11px; color:var(--text-3);">💰 {sd.kode}</span>{/if}
                {/if}
                {#if dsk.lokasi}
                  <span style="font-size:11px; color:var(--text-3);">📍 {dsk.lokasi}</span>
                {/if}
                {#if dsk.waktu_pelaksanaan}
                  <span style="font-size:11px; color:var(--text-3);">🗓 {dsk.waktu_pelaksanaan}</span>
                {/if}
                {#if dsk.keluaran}
                  <span style="font-size:11px; color:var(--text-3);">📦 {dsk.keluaran}{dsk.target_keluaran ? " · " + dsk.target_keluaran : ""}</span>
                {/if}
              </div>
            </div>

            <!-- Tombol aksi -->
            <div style="display:flex; gap:6px; flex-shrink:0;">
              <button onclick={() => bukaEdit(dsk)} class="btn btn-ghost" style="font-size:12px;">Edit</button>
              <button onclick={() => goto("/dokumen/" + dokumenId + "/sub/" + dsk.id)} class="btn btn-primary" style="font-size:12px; background:{cfg.warna}; border-color:{cfg.warna};">Rincian →</button>
              <button onclick={() => hapus(dsk.id)} class="btn btn-ghost" style="font-size:12px; color:var(--text-3);">✕</button>
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
                <button class="btn btn-primary" style="font-size:12px;" disabled={submitting} onclick={() => simpanEdit(dsk)}>
                  {submitting ? "Menyimpan..." : "Simpan"}
                </button>
                <button class="btn btn-ghost" style="font-size:12px;" onclick={() => editingId = null}>Batal</button>
              </div>
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
{/if}