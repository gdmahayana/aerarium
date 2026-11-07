<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
    created_at: string;
  }

  let list = $state<TahunAnggaran[]>([]);
  let loading = $state(true);
  let error = $state("");
  let tahunBaru = $state("");
  let submitting = $state(false);

  onMount(async () => { await muat(); });

  async function muat() {
    try {
      loading = true;
      list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  }

  async function tambah() {
    if (!tahunBaru || isNaN(Number(tahunBaru))) return;
    submitting = true;
    try {
      await invoke("tambah_tahun_anggaran", { input: { tahun: Number(tahunBaru) } });
      tahunBaru = "";
      await muat();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function setAktif(id: number) {
    try {
      await invoke("set_tahun_anggaran_aktif", { id });
      await muat();
    } catch (e) { error = String(e); }
  }

  async function hapus(id: number) {
    if (!confirm("Hapus tahun anggaran ini?")) return;
    try {
      await invoke("hapus_tahun_anggaran", { id });
      await muat();
    } catch (e) { error = String(e); }
  }
</script>

<div class="page-header">
  <h2>Tahun Anggaran</h2>
  <p>Kelola tahun anggaran yang digunakan</p>
</div>

{#if error}
  <div class="alert-error">
    {error}
    <button onclick={() => error = ""} style="margin-left:8px; text-decoration:underline; background:none; border:none; color:inherit; cursor:pointer; font-family:inherit;">Tutup</button>
  </div>
{/if}

<div class="card" style="padding:20px; margin-bottom:20px; max-width:420px;">
  <div style="font-size:13px; font-weight:600; color:var(--text); margin-bottom:14px;">Tambah Tahun Anggaran</div>
  <div style="display:flex; gap:10px;">
    <input
      type="number"
      placeholder="Contoh: 2026"
      bind:value={tahunBaru}
      class="input"
      style="max-width:160px;"
    />
    <button onclick={tambah} disabled={submitting} class="btn btn-primary">
      {submitting ? "Menyimpan..." : "Tambah"}
    </button>
  </div>
</div>

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if list.length === 0}
  <div class="card" style="padding:40px; text-align:center;">
    <p style="font-size:13px; color:var(--text-3);">Belum ada tahun anggaran.</p>
  </div>
{:else}
  <div class="card">
    {#each list as item, i}
      <div style="display:flex; align-items:center; justify-content:space-between; padding:14px 20px; {i < list.length - 1 ? 'border-bottom:1px solid var(--border);' : ''}">
        <div style="display:flex; align-items:center; gap:10px;">
          <span style="font-size:18px; font-weight:700; color:var(--text);">{item.tahun}</span>
          {#if item.aktif}
            <span class="badge badge-aktif">Aktif</span>
          {/if}
        </div>
        <div style="display:flex; gap:8px;">
          {#if !item.aktif}
            <button onclick={() => setAktif(item.id)} class="btn btn-ghost">Set Aktif</button>
          {/if}
          <button onclick={() => hapus(item.id)} class="btn btn-danger">Hapus</button>
        </div>
      </div>
    {/each}
  </div>
{/if}
