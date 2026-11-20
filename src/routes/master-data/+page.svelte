<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "$lib/stores/toast";
  import { onMount } from "svelte";

  interface MasterProgram {
    id: number;
    kode: string;
    uraian: string;
    aktif: boolean;
  }

  interface MasterKegiatan {
    id: number;
    master_program_id: number;
    kode: string;
    uraian: string;
    aktif: boolean;
  }

  interface MasterSubKegiatan {
    id: number;
    master_kegiatan_id: number;
    kode: string;
    uraian: string;
    aktif: boolean;
  }

  interface SumberDana {
    id: number;
    tahun_anggaran_id: number;
    kode: string;
    nama: string;
  }

  interface Rekening {
    id: number;
    kode: string;
    uraian: string;
    level: number;
  }

  let tahunAktifId = $state(0);
  let rekeningList = $state<Rekening[]>([]);
  let showFormRekening = $state(false);
  let formRekening = $state({ kode: "", uraian: "" });
  let editRekeningId = $state<number | null>(null);
  let editRekeningUraian = $state("");
  let cariRekening = $state("");
  let submittingRekening = $state(false);
  let showImportRekening = $state(false);
  let importCsv = $state("");
  let importResult = $state("");
  interface SumberDana {
    id: number;
    tahun_anggaran_id: number;
    kode: string;
    nama: string;
  }

  let activeTab = $state<"program"|"sumber_dana"|"rekening">("program");

  let programs = $state<MasterProgram[]>([]);
  let sumberDanaList = $state<SumberDana[]>([]);
  let showFormSD = $state(false);
  let formSD = $state({ kode: "", nama: "" });
  let kegiatanMap = $state<Record<number, MasterKegiatan[]>>({});
  let subKegiatanMap = $state<Record<number, MasterSubKegiatan[]>>({});
  let expandedProgram = $state<number | null>(null);
  let expandedKegiatan = $state<number | null>(null);
  let loading = $state(true);

  // Form state
  let formProgram = $state({ kode: "", uraian: "" });
  let formKegiatan = $state({ master_program_id: 0, kode: "", uraian: "" });
  let formSubKegiatan = $state({ master_kegiatan_id: 0, kode: "", uraian: "" });
  let showFormProgram = $state(false);
  let showFormKegiatan = $state<number | null>(null);
  let showFormSubKegiatan = $state<number | null>(null);

  async function loadRekening() {
    try {
      rekeningList = await invoke<Rekening[]>("get_semua_rekening");
    } catch(e) {}
  }

  async function tambahRekening() {
    if (!formRekening.kode || !formRekening.uraian) return;
    submittingRekening = true;
    try {
      await invoke("cari_atau_buat_rekening", { kode: formRekening.kode, uraian: formRekening.uraian });
      formRekening = { kode: "", uraian: "" };
      showFormRekening = false;
      await loadRekening();
    } catch(e) {}
    finally { submittingRekening = false; }
  }

  async function importRekening() {
    if (!importCsv.trim()) return;
    submittingRekening = true;
    importResult = "";
    try {
      const lines = importCsv.trim().split(String.fromCharCode(10));
      const rows = lines
        .map((r: string) => r.replace(String.fromCharCode(13), "").split(",").map((s: string) => s.trim()))
        .filter((r: string[]) => r.length >= 2 && r[0] && r[1])
        .map((r: string[]) => [r[0], r[1]] as [string, string]);

      if (rows.length === 0) { importResult = "❌ Tidak ada data valid."; return; }
      const count = await invoke<number>("import_rekening_bulk", { data: rows });
      importResult = "✅ " + count + " rekening berhasil diimport.";
      importCsv = "";
      showImportRekening = false;
      await loadRekening();
    } catch(e) { importResult = "❌ " + String(e); }
    finally { submittingRekening = false; }
  }

  async function simpanEditRekening(id: number) {
    if (!editRekeningUraian.trim()) return;
    submittingRekening = true;
    try {
      await invoke("update_rekening_uraian", { id, uraian: editRekeningUraian });
      rekeningList = await invoke<Rekening[]>("get_semua_rekening");
      editRekeningId = null;
    } catch(e) { toast.error(String(e)); }
    finally { submittingRekening = false; }
  }

  async function hapusRekeningMaster(id: number) {
    if (!confirm("Hapus rekening ini dari master data?")) return;
    try {
      await invoke("hapus_rekening", { id });
      await loadRekening();
    } catch(e) {}
  }

  let rekeningFiltered = $derived(
    cariRekening.length < 2 ? rekeningList :
    rekeningList.filter(r =>
      r.kode.toLowerCase().includes(cariRekening.toLowerCase()) ||
      r.uraian.toLowerCase().includes(cariRekening.toLowerCase())
    )
  );

  onMount(async () => {
    try {
      const tahunList = await invoke<any[]>("get_tahun_anggaran");
      const tahunAktif = tahunList.find((t: any) => t.aktif);
      if (tahunAktif) {
        tahunAktifId = tahunAktif.id;
        sumberDanaList = await invoke<SumberDana[]>("get_sumber_dana", { tahunAnggaranId: tahunAktif.id });
      }
    } catch(e) { toast.error(String(e)); }
    await loadPrograms();
    await loadRekening();
  });

  async function loadPrograms() {
    loading = true;
    try {
      programs = await invoke<MasterProgram[]>("get_master_program");
    } catch (e) {
      toast.error(String(e));
    } finally {
      loading = false;
    }
  }

  async function loadKegiatan(programId: number) {
    try {
      const list = await invoke<MasterKegiatan[]>("get_master_kegiatan", { masterProgramId: programId });
      kegiatanMap = { ...kegiatanMap, [programId]: list };
    } catch (e) {
      toast.error(String(e));
    }
  }

  async function loadSubKegiatan(kegiatanId: number) {
    try {
      const list = await invoke<MasterSubKegiatan[]>("get_master_sub_kegiatan", { masterKegiatanId: kegiatanId });
      subKegiatanMap = { ...subKegiatanMap, [kegiatanId]: list };
    } catch (e) {
      toast.error(String(e));
    }
  }

  async function toggleProgram(programId: number) {
    if (expandedProgram === programId) {
      expandedProgram = null;
      return;
    }
    expandedProgram = programId;
    expandedKegiatan = null;
    if (!kegiatanMap[programId]) await loadKegiatan(programId);
  }

  async function toggleKegiatan(kegiatanId: number) {
    if (expandedKegiatan === kegiatanId) {
      expandedKegiatan = null;
      return;
    }
    expandedKegiatan = kegiatanId;
    if (!subKegiatanMap[kegiatanId]) await loadSubKegiatan(kegiatanId);
  }

  async function tambahProgram() {
    if (!formProgram.kode || !formProgram.uraian) return;
    try {
      await invoke("tambah_master_program", { input: formProgram });
      formProgram = { kode: "", uraian: "" };
      showFormProgram = false;
      await loadPrograms();
    await loadRekening();
    } catch (e) {
      toast.error(String(e));
    }
  }

  async function tambahKegiatan(programId: number) {
    if (!formKegiatan.kode || !formKegiatan.uraian) return;
    try {
      await invoke("tambah_master_kegiatan", { input: { ...formKegiatan, master_program_id: programId } });
      formKegiatan = { master_program_id: 0, kode: "", uraian: "" };
      showFormKegiatan = null;
      await loadKegiatan(programId);
    } catch (e) {
      toast.error(String(e));
    }
  }

  async function tambahSubKegiatan(kegiatanId: number, programId: number) {
    if (!formSubKegiatan.kode || !formSubKegiatan.uraian) return;
    try {
      await invoke("tambah_master_sub_kegiatan", { input: { ...formSubKegiatan, master_kegiatan_id: kegiatanId } });
      formSubKegiatan = { master_kegiatan_id: 0, kode: "", uraian: "" };
      showFormSubKegiatan = null;
      await loadSubKegiatan(kegiatanId);
    } catch (e) {
      toast.error(String(e));
    }
  }

  async function toggleAktifProgram(id: number) {
    try {
      await invoke("toggle_master_program", { id });
      await loadPrograms();
    await loadRekening();
    } catch (e) { toast.error(String(e)); }
  }

  async function toggleAktifKegiatan(id: number, programId: number) {
    try {
      await invoke("toggle_master_kegiatan", { id });
      await loadKegiatan(programId);
    } catch (e) { toast.error(String(e)); }
  }

  async function tambahSumberDana() {
    if (!formSD.kode || !formSD.nama || !tahunAktifId) return;
    try {
      await invoke("tambah_sumber_dana", { input: { tahun_anggaran_id: tahunAktifId, kode: formSD.kode, nama: formSD.nama } });
      sumberDanaList = await invoke<SumberDana[]>("get_sumber_dana", { tahunAnggaranId: tahunAktifId });
      formSD = { kode: "", nama: "" };
      showFormSD = false;
    } catch(e) { toast.error(String(e)); }
  }

  async function toggleAktifSubKegiatan(id: number, kegiatanId: number) {
    try {
      await invoke("toggle_master_sub_kegiatan", { id });
      await loadSubKegiatan(kegiatanId);
    } catch (e) { toast.error(String(e)); }
  }
</script>

<div class="page-header">
  <h2>Master Data</h2>
  <p>Kelola program, kegiatan, dan sub kegiatan dari Renstra/Renja</p>
</div>


<!-- Tab switcher -->
<div style="display:flex; gap:0; border:1px solid var(--border); border-radius:8px; overflow:hidden; margin-bottom:24px; width:fit-content;">
  <button
    style="padding:8px 20px; font-size:12px; font-weight:600; border:none; cursor:pointer; font-family:inherit; background:{activeTab === 'program' ? 'var(--accent)' : 'transparent'}; color:{activeTab === 'program' ? 'white' : 'var(--text-3)'}; "
    onclick={() => activeTab = 'program'}
  >Program & Kegiatan</button>
  <button
    style="padding:8px 20px; font-size:12px; font-weight:600; border:none; cursor:pointer; font-family:inherit; background:{activeTab === 'sumber_dana' ? 'var(--accent)' : 'transparent'}; color:{activeTab === 'sumber_dana' ? 'white' : 'var(--text-3)'}; border-left:1px solid var(--border);"
    onclick={() => activeTab = 'sumber_dana'}
  >Sumber Dana</button>
  <button
    style="padding:8px 20px; font-size:12px; font-weight:600; border:none; cursor:pointer; font-family:inherit; background:{activeTab === 'rekening' ? 'var(--accent)' : 'transparent'}; color:{activeTab === 'rekening' ? 'white' : 'var(--text-3)'}; border-left:1px solid var(--border);"
    onclick={() => activeTab = 'rekening'}
  >Rekening {rekeningList.length > 0 ? '(' + rekeningList.length + ')' : ''}</button>
</div>

{#if activeTab === 'program'}
<div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:16px;">
  <span style="font-size:13px; color:var(--text-3);">{programs.length} program terdaftar</span>
  <button class="btn btn-primary" onclick={() => showFormProgram = !showFormProgram}>+ Tambah Program</button>
</div>

{#if showFormProgram}
  <div class="card" style="padding:16px; margin-bottom:16px; display:flex; gap:10px; align-items:flex-end;">
    <div style="flex:0 0 160px;">
      <label class="label">Kode</label>
      <input class="input" bind:value={formProgram.kode} placeholder="1.02.02" />
    </div>
    <div style="flex:1;">
      <label class="label">Uraian</label>
      <input class="input" bind:value={formProgram.uraian} placeholder="Nama program..." />
    </div>
    <button class="btn btn-primary" onclick={tambahProgram}>Simpan</button>
    <button class="btn btn-ghost" onclick={() => showFormProgram = false}>Batal</button>
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if programs.length === 0}
  <div class="card" style="padding:48px; text-align:center;">
    <p style="font-size:13px; color:var(--text-3);">Belum ada program. Tambah program dari Renstra/Renja.</p>
  </div>
{:else}
  <div style="display:flex; flex-direction:column; gap:8px;">
    {#each programs as prog}
      <div class="card">
        <!-- Program Row -->
        <div style="display:flex; align-items:center; gap:10px; padding:12px 16px; cursor:pointer;" onclick={() => toggleProgram(prog.id)}>
          <span style="font-size:12px; color:var(--text-3); width:16px;">{expandedProgram === prog.id ? "▾" : "▸"}</span>
          <span style="font-size:12px; font-weight:600; color:var(--text-3); min-width:100px;">{prog.kode}</span>
          <span style="font-size:13px; font-weight:600; color:var(--text); flex:1;">{prog.uraian}</span>
          <button
            class="btn btn-ghost" style="font-size:11px; padding:3px 10px;"
            onclick={(e) => { e.stopPropagation(); toggleAktifProgram(prog.id); }}
          >{prog.aktif ? "Nonaktifkan" : "Aktifkan"}</button>
          {#if !prog.aktif}
            <span style="font-size:11px; color:var(--text-3); background:var(--surface-2); padding:2px 8px; border-radius:4px;">nonaktif</span>
          {/if}
        </div>

        <!-- Kegiatan List -->
        {#if expandedProgram === prog.id}
          <div style="border-top:1px solid var(--border); padding:8px 16px 8px 40px;">
            {#each (kegiatanMap[prog.id] || []) as keg}
              <div style="margin-bottom:4px;">
                <div style="display:flex; align-items:center; gap:10px; padding:8px 12px; border-radius:6px; background:var(--surface-2); cursor:pointer;" onclick={() => toggleKegiatan(keg.id)}>
                  <span style="font-size:11px; color:var(--text-3); width:16px;">{expandedKegiatan === keg.id ? "▾" : "▸"}</span>
                  <span style="font-size:11px; font-weight:600; color:var(--text-3); min-width:120px;">{keg.kode}</span>
                  <span style="font-size:12px; font-weight:600; color:var(--text); flex:1;">{keg.uraian}</span>
                  <button
                    class="btn btn-ghost" style="font-size:11px; padding:2px 8px;"
                    onclick={(e) => { e.stopPropagation(); toggleAktifKegiatan(keg.id, prog.id); }}
                  >{keg.aktif ? "Nonaktifkan" : "Aktifkan"}</button>
                  {#if !keg.aktif}
                    <span style="font-size:11px; color:var(--text-3);">nonaktif</span>
                  {/if}
                </div>

                <!-- Sub Kegiatan List -->
                {#if expandedKegiatan === keg.id}
                  <div style="padding:6px 0 6px 32px;">
                    {#each (subKegiatanMap[keg.id] || []) as sub}
                      <div style="display:flex; align-items:center; gap:10px; padding:6px 12px; border-radius:4px;">
                        <span style="font-size:11px; color:var(--text-3); min-width:140px;">{sub.kode}</span>
                        <span style="font-size:12px; color:var(--text); flex:1;">{sub.uraian}</span>
                        <button
                          class="btn btn-ghost" style="font-size:11px; padding:2px 8px;"
                          onclick={() => toggleAktifSubKegiatan(sub.id, keg.id)}
                        >{sub.aktif ? "Nonaktifkan" : "Aktifkan"}</button>
                        {#if !sub.aktif}
                          <span style="font-size:11px; color:var(--text-3);">nonaktif</span>
                        {/if}
                      </div>
                    {/each}

                    <!-- Form tambah sub kegiatan -->
                    {#if showFormSubKegiatan === keg.id}
                      <div style="display:flex; gap:8px; align-items:flex-end; padding:8px 12px; margin-top:4px;">
                        <div style="flex:0 0 160px;">
                          <input class="input" style="font-size:12px;" bind:value={formSubKegiatan.kode} placeholder="Kode sub kegiatan" />
                        </div>
                        <div style="flex:1;">
                          <input class="input" style="font-size:12px;" bind:value={formSubKegiatan.uraian} placeholder="Uraian sub kegiatan..." />
                        </div>
                        <button class="btn btn-primary" style="font-size:12px;" onclick={() => tambahSubKegiatan(keg.id, prog.id)}>Simpan</button>
                        <button class="btn btn-ghost" style="font-size:12px;" onclick={() => showFormSubKegiatan = null}>Batal</button>
                      </div>
                    {:else}
                      <button class="btn btn-ghost" style="font-size:11px; margin-top:4px;" onclick={() => { showFormSubKegiatan = keg.id; formSubKegiatan = { master_kegiatan_id: keg.id, kode: "", uraian: "" }; }}>+ Sub Kegiatan</button>
                    {/if}
                  </div>
                {/if}
              </div>
            {/each}

            <!-- Form tambah kegiatan -->
            {#if showFormKegiatan === prog.id}
              <div style="display:flex; gap:8px; align-items:flex-end; padding:8px 0; margin-top:4px;">
                <div style="flex:0 0 160px;">
                  <input class="input" style="font-size:12px;" bind:value={formKegiatan.kode} placeholder="Kode kegiatan" />
                </div>
                <div style="flex:1;">
                  <input class="input" style="font-size:12px;" bind:value={formKegiatan.uraian} placeholder="Uraian kegiatan..." />
                </div>
                <button class="btn btn-primary" style="font-size:12px;" onclick={() => tambahKegiatan(prog.id)}>Simpan</button>
                <button class="btn btn-ghost" style="font-size:12px;" onclick={() => showFormKegiatan = null}>Batal</button>
              </div>
            {:else}
              <button class="btn btn-ghost" style="font-size:12px; margin-top:8px;" onclick={() => { showFormKegiatan = prog.id; formKegiatan = { master_program_id: prog.id, kode: "", uraian: "" }; }}>+ Kegiatan</button>
            {/if}
          </div>
        {/if}
      </div>
    {/each}
  </div>
{/if}

{/if}

{#if activeTab === 'sumber_dana'}
<div>
  <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
    <div>
      <span class="label">Sumber Dana</span>
      <span style="font-size:12px; color:var(--text-3); margin-left:8px;">per tahun anggaran aktif</span>
    </div>
    <button class="btn btn-primary" style="font-size:12px;" onclick={() => showFormSD = !showFormSD}>+ Tambah Sumber Dana</button>
  </div>

  {#if showFormSD}
    <div class="card" style="padding:16px; margin-bottom:12px; display:flex; gap:10px; align-items:flex-end; max-width:560px;">
      <div style="flex:0 0 140px;">
        <label class="label" style="display:block; margin-bottom:6px;">Kode</label>
        <input class="input" bind:value={formSD.kode} placeholder="APBD / DAK / ..." />
      </div>
      <div style="flex:1;">
        <label class="label" style="display:block; margin-bottom:6px;">Nama</label>
        <input class="input" bind:value={formSD.nama} placeholder="Dana Alokasi Khusus" />
      </div>
      <button class="btn btn-primary" onclick={tambahSumberDana}>Simpan</button>
      <button class="btn btn-ghost" onclick={() => showFormSD = false}>Batal</button>
    </div>
  {/if}

  {#if !tahunAktifId}
    <p style="font-size:12px; color:var(--text-3);">Belum ada tahun anggaran aktif.</p>
  {:else if sumberDanaList.length === 0}
    <div class="card" style="padding:24px; text-align:center;">
      <p style="font-size:12px; color:var(--text-3);">Belum ada sumber dana untuk tahun ini.</p>
    </div>
  {:else}
    <div class="card">
      {#each sumberDanaList as sd, i}
        <div style="display:flex; align-items:center; gap:12px; padding:12px 20px; {i < sumberDanaList.length - 1 ? 'border-bottom:1px solid var(--border);' : ''}">
          <span style="font-size:12px; font-weight:600; font-family:monospace; color:var(--text-3); min-width:100px;">{sd.kode}</span>
          <span style="font-size:13px; color:var(--text); flex:1;">{sd.nama}</span>
        </div>
      {/each}
    </div>
  {/if}
</div>

{/if}

{#if activeTab === 'rekening'}
<div>
  <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:12px;">
    <div>
      <span class="label">Rekening</span>
      <span style="font-size:12px; color:var(--text-3); margin-left:8px;">kode rekening yang pernah dipakai</span>
    </div>
    <div style="display:flex; gap:8px;">
      <button class="btn btn-ghost" style="font-size:12px;" onclick={() => { showImportRekening = !showImportRekening; showFormRekening = false; }}>⬆ Import CSV</button>
      <button class="btn btn-primary" style="font-size:12px;" onclick={() => { showFormRekening = !showFormRekening; showImportRekening = false; }}>+ Tambah Rekening</button>
    </div>
  </div>

  {#if showImportRekening}
    <div class="card" style="padding:16px; margin-bottom:12px; max-width:640px;">
      <div style="font-size:12px; color:var(--text-3); margin-bottom:10px;">
        Paste data dari Excel — format: <code style="background:var(--surface-2); padding:1px 6px; border-radius:3px;">kode,uraian</code> per baris. Pemisah bisa koma, titik koma, atau tab.
      </div>
      <textarea
        class="input"
        style="width:100%; height:160px; font-family:monospace; font-size:12px; resize:vertical;"
        bind:value={importCsv}
        placeholder="5.1.02.01.001,Belanja Alat Tulis Kantor&#10;5.1.02.01.002,Belanja Kertas dan Bahan Cetak&#10;..."
      ></textarea>
      <div style="display:flex; gap:8px; margin-top:10px; align-items:center;">
        <button class="btn btn-primary" onclick={importRekening} disabled={submittingRekening || !importCsv.trim()}>
          {submittingRekening ? 'Mengimport...' : 'Import'}
        </button>
        <button class="btn btn-ghost" onclick={() => { showImportRekening = false; importCsv = ''; importResult = ''; }}>Batal</button>
        {#if importResult}
          <span style="font-size:12px; color:var(--text-2);">{importResult}</span>
        {/if}
      </div>
    </div>
  {/if}

  {#if showFormRekening}
    <div class="card" style="padding:16px; margin-bottom:12px; display:flex; gap:10px; align-items:flex-end; max-width:640px;">
      <div style="flex:0 0 180px;">
        <label class="label" style="display:block; margin-bottom:6px;">Kode Rekening</label>
        <input class="input" bind:value={formRekening.kode} placeholder="5.1.02.01.001" autofocus />
      </div>
      <div style="flex:1;">
        <label class="label" style="display:block; margin-bottom:6px;">Uraian</label>
        <input class="input" bind:value={formRekening.uraian} placeholder="Belanja Bahan Habis Pakai..." />
      </div>
      <button class="btn btn-primary" onclick={tambahRekening} disabled={submittingRekening || !formRekening.kode || !formRekening.uraian}>
        {submittingRekening ? '...' : 'Simpan'}
      </button>
      <button class="btn btn-ghost" onclick={() => showFormRekening = false}>Batal</button>
    </div>
  {/if}

  <!-- Search -->
  {#if rekeningList.length > 0}
    <div style="margin-bottom:12px; max-width:360px;">
      <input class="input" bind:value={cariRekening} placeholder="Cari kode atau uraian..." />
    </div>
  {/if}

  {#if rekeningList.length === 0}
    <div class="card" style="padding:24px; text-align:center;">
      <p style="font-size:12px; color:var(--text-3);">Belum ada rekening. Rekening akan otomatis tersimpan saat dipakai, atau tambah manual di sini.</p>
    </div>
  {:else}
    <div class="card" style="overflow:hidden;">
      <table style="width:100%; border-collapse:collapse; font-size:12px;">
        <thead>
          <tr style="background:var(--surface-2); border-bottom:1px solid var(--border);">
            <th style="text-align:left; padding:10px 16px; color:var(--text-3); font-weight:600; width:180px;">Kode</th>
            <th style="text-align:left; padding:10px 16px; color:var(--text-3); font-weight:600;">Uraian</th>
            <th style="padding:10px 16px; width:60px;"></th>
          </tr>
        </thead>
        <tbody>
          {#each rekeningFiltered as rek, i}
            <tr style="border-bottom:{i < rekeningFiltered.length - 1 ? '1px solid var(--border)' : 'none'};">
              <td style="padding:10px 16px; font-family:monospace; color:var(--text-3); font-weight:600;">{rek.kode}</td>
              <td style="padding:10px 16px; color:var(--text);">
                {#if editRekeningId === rek.id}
                  <input class="input" style="font-size:12px;" bind:value={editRekeningUraian}
                    onkeydown={(e) => { if (e.key === 'Enter') simpanEditRekening(rek.id); if (e.key === 'Escape') editRekeningId = null; }}
                    autofocus />
                {:else}
                  {rek.uraian}
                {/if}
              </td>
              <td style="padding:10px 16px; text-align:right;">
                <div style="display:flex; gap:4px; justify-content:flex-end;">
                  {#if editRekeningId === rek.id}
                    <button onclick={() => simpanEditRekening(rek.id)} disabled={submittingRekening} class="btn btn-primary" style="font-size:11px;">Simpan</button>
                    <button onclick={() => editRekeningId = null} class="btn btn-ghost" style="font-size:11px;">Batal</button>
                  {:else}
                    <button onclick={() => { editRekeningId = rek.id; editRekeningUraian = rek.uraian; }} class="btn btn-ghost" style="font-size:11px;">Edit</button>
                    <button onclick={() => hapusRekeningMaster(rek.id)} class="btn btn-ghost" style="font-size:11px; color:var(--text-3);">✕</button>
                  {/if}
                </div>
              </td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
</div>
{/if}
