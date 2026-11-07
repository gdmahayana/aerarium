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

  onMount(async () => {
    await muat();
  });

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
      await invoke("tambah_tahun_anggaran", {
        input: { tahun: Number(tahunBaru) }
      });
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
    } catch (e) {
      error = String(e);
    }
  }

  async function hapus(id: number) {
    if (!confirm("Hapus tahun anggaran ini?")) return;
    try {
      await invoke("hapus_tahun_anggaran", { id });
      await muat();
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="p-6">
  <div class="mb-6">
    <h2 class="text-xl font-bold">Tahun Anggaran</h2>
    <p class="text-sm text-gray-500 mt-1">Kelola tahun anggaran yang digunakan</p>
  </div>

  {#if error}
    <div class="mb-4 px-4 py-3 bg-red-50 border border-red-200 rounded text-sm text-red-600">
      {error}
      <button onclick={() => error = ""} class="ml-2 underline">Tutup</button>
    </div>
  {/if}

  <!-- Form tambah -->
  <div class="bg-white border border-gray-200 rounded-lg p-4 mb-6">
    <h3 class="text-sm font-semibold mb-3">Tambah Tahun Anggaran</h3>
    <div class="flex gap-2">
      <input
        type="number"
        placeholder="Contoh: 2026"
        bind:value={tahunBaru}
        class="border border-gray-300 rounded px-3 py-2 text-sm w-40 focus:outline-none focus:border-gray-500"
      />
      <button
        onclick={tambah}
        disabled={submitting}
        class="px-4 py-2 bg-gray-900 text-white text-sm rounded hover:bg-gray-700 disabled:opacity-50"
      >
        {submitting ? "Menyimpan..." : "Tambah"}
      </button>
    </div>
  </div>

  <!-- Daftar -->
  {#if loading}
    <p class="text-sm text-gray-400">Memuat...</p>
  {:else if list.length === 0}
    <div class="border border-dashed border-gray-300 rounded-lg p-6 text-center">
      <p class="text-sm text-gray-400">Belum ada tahun anggaran.</p>
    </div>
  {:else}
    <div class="bg-white border border-gray-200 rounded-lg divide-y divide-gray-100">
      {#each list as item}
        <div class="flex items-center justify-between px-4 py-3">
          <div class="flex items-center gap-3">
            <span class="text-sm font-medium">{item.tahun}</span>
            {#if item.aktif}
              <span class="text-xs px-2 py-0.5 bg-green-100 text-green-700 rounded-full">Aktif</span>
            {/if}
          </div>
          <div class="flex gap-2">
            {#if !item.aktif}
              <button
                onclick={() => setAktif(item.id)}
                class="text-xs px-3 py-1.5 border border-gray-300 rounded hover:bg-gray-50"
              >
                Set Aktif
              </button>
            {/if}
            <button
              onclick={() => hapus(item.id)}
              class="text-xs px-3 py-1.5 border border-red-200 text-red-600 rounded hover:bg-red-50"
            >
              Hapus
            </button>
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>
