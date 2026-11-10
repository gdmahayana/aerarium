<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
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

  let programs = $state<MasterProgram[]>([]);
  let kegiatanMap = $state<Record<number, MasterKegiatan[]>>({});
  let subKegiatanMap = $state<Record<number, MasterSubKegiatan[]>>({});
  let expandedProgram = $state<number | null>(null);
  let expandedKegiatan = $state<number | null>(null);
  let loading = $state(true);
  let error = $state("");

  // Form state
  let formProgram = $state({ kode: "", uraian: "" });
  let formKegiatan = $state({ master_program_id: 0, kode: "", uraian: "" });
  let formSubKegiatan = $state({ master_kegiatan_id: 0, kode: "", uraian: "" });
  let showFormProgram = $state(false);
  let showFormKegiatan = $state<number | null>(null);
  let showFormSubKegiatan = $state<number | null>(null);

  onMount(async () => {
    await loadPrograms();
  });

  async function loadPrograms() {
    loading = true;
    try {
      programs = await invoke<MasterProgram[]>("get_master_program");
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function loadKegiatan(programId: number) {
    try {
      const list = await invoke<MasterKegiatan[]>("get_master_kegiatan", { masterProgramId: programId });
      kegiatanMap = { ...kegiatanMap, [programId]: list };
    } catch (e) {
      error = String(e);
    }
  }

  async function loadSubKegiatan(kegiatanId: number) {
    try {
      const list = await invoke<MasterSubKegiatan[]>("get_master_sub_kegiatan", { masterKegiatanId: kegiatanId });
      subKegiatanMap = { ...subKegiatanMap, [kegiatanId]: list };
    } catch (e) {
      error = String(e);
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
    } catch (e) {
      error = String(e);
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
      error = String(e);
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
      error = String(e);
    }
  }

  async function toggleAktifProgram(id: number) {
    try {
      await invoke("toggle_master_program", { id });
      await loadPrograms();
    } catch (e) { error = String(e); }
  }

  async function toggleAktifKegiatan(id: number, programId: number) {
    try {
      await invoke("toggle_master_kegiatan", { id });
      await loadKegiatan(programId);
    } catch (e) { error = String(e); }
  }

  async function toggleAktifSubKegiatan(id: number, kegiatanId: number) {
    try {
      await invoke("toggle_master_sub_kegiatan", { id });
      await loadSubKegiatan(kegiatanId);
    } catch (e) { error = String(e); }
  }
</script>

<div class="page-header">
  <h2>Master Data</h2>
  <p>Kelola program, kegiatan, dan sub kegiatan dari Renstra/Renja</p>
</div>

{#if error}
  <div class="alert-error">{error}</div>
{/if}

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
