<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
    created_at: string;
  }

  interface Dokumen {
    id: number;
    tahun_anggaran_id: number;
    jenis: string;
    versi: number;
    nomor_se: string | null;
    status: string;
    created_at: string;
  }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let dokumenList = $state<Dokumen[]>([]);
  let loading = $state(true);

  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;

      if (tahunAktif) {
        dokumenList = await invoke<Dokumen[]>("get_dokumen", {
          tahunAnggaranId: tahunAktif.id
        });
      }
    } catch (e) {
      console.error(e);
    } finally {
      loading = false;
    }
  });

  const statusColor: Record<string, string> = {
    draft: "bg-gray-100 text-gray-600",
    konsep: "bg-blue-100 text-blue-700",
    ditetapkan: "bg-green-100 text-green-700",
    dikunci: "bg-yellow-100 text-yellow-700",
    diarsipkan: "bg-red-100 text-red-700",
  };
</script>

<div class="p-6">
  <!-- Header -->
  <div class="mb-6">
    <h2 class="text-xl font-bold">Dashboard</h2>
    <p class="text-sm text-gray-500 mt-1">
      Plan with Confidence. Change with Control.
    </p>
  </div>

  {#if loading}
    <p class="text-sm text-gray-400">Memuat data...</p>
  {:else if !tahunAktif}
    <!-- Belum ada tahun anggaran -->
    <div class="border border-dashed border-gray-300 rounded-lg p-8 text-center">
      <p class="text-gray-500 text-sm">Belum ada tahun anggaran aktif.</p>
      <a
        href="/tahun-anggaran"
        class="inline-block mt-3 px-4 py-2 bg-gray-900 text-white text-sm rounded hover:bg-gray-700"
      >
        Tambah Tahun Anggaran
      </a>
    </div>
  {:else}
    <!-- Info tahun aktif -->
    <div class="bg-white border border-gray-200 rounded-lg p-4 mb-6">
      <p class="text-xs text-gray-500 uppercase tracking-wide">Tahun Anggaran Aktif</p>
      <p class="text-3xl font-bold mt-1">{tahunAktif.tahun}</p>
    </div>

    <!-- Daftar dokumen -->
    <div>
      <div class="flex items-center justify-between mb-3">
        <h3 class="text-sm font-semibold text-gray-700">Dokumen</h3>
        <a
          href="/dokumen/buat"
          class="text-xs px-3 py-1.5 bg-gray-900 text-white rounded hover:bg-gray-700"
        >
          + Buat Dokumen
        </a>
      </div>

      {#if dokumenList.length === 0}
        <div class="border border-dashed border-gray-300 rounded-lg p-6 text-center">
          <p class="text-sm text-gray-400">Belum ada dokumen untuk tahun ini.</p>
        </div>
      {:else}
        <div class="bg-white border border-gray-200 rounded-lg divide-y divide-gray-100">
          {#each dokumenList as dok}
            <div class="flex items-center justify-between px-4 py-3">
              <div>
                <p class="text-sm font-medium">{dok.jenis} v{dok.versi}</p>
                {#if dok.nomor_se}
                  <p class="text-xs text-gray-400 mt-0.5">SE: {dok.nomor_se}</p>
                {/if}
              </div>
              <span class="text-xs px-2 py-1 rounded-full {statusColor[dok.status] || ''}">{dok.status}</span>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  {/if}
</div>
