<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
  }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let loading = $state(true);
  let submitting = $state(false);
  let error = $state("");

  let jenis = $state("RKA-SKPD");
  let nomor_se = $state("");
  let tanggal_se = $state("");
  let catatan = $state("");

  const jenisList = [
    { value: "RKA-SKPD", label: "RKA-SKPD" },
    { value: "DPA-SKPD", label: "DPA-SKPD" },
    { value: "RKA-SKPD Perubahan", label: "RKA-SKPD Perubahan" },
    { value: "DPPA-SKPD", label: "DPPA-SKPD" },
  ];

  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;
    } catch (e) {
      error = String(e);
    } finally {
      loading = false;
    }
  });

  async function simpan() {
    if (!tahunAktif) return;
    submitting = true;
    try {
      await invoke("buat_dokumen", {
        input: {
          tahun_anggaran_id: tahunAktif.id,
          jenis,
          nomor_se: nomor_se || null,
          tanggal_se: tanggal_se || null,
          catatan: catatan || null,
        }
      });
      goto("/dokumen");
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }
</script>

<div class="page-header">
  <h2>Buat Dokumen</h2>
  <p>Buat dokumen anggaran baru berdasarkan Surat Edaran</p>
</div>

{#if error}
  <div class="alert-error">
    {error}
    <button onclick={() => error = ""} style="margin-left:8px; text-decoration:underline; background:none; border:none; color:inherit; cursor:pointer; font-family:inherit;">Tutup</button>
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !tahunAktif}
  <div class="card" style="padding:40px; text-align:center; max-width:400px;">
    <p style="font-size:13px; color:var(--text-2); margin-bottom:16px;">Belum ada tahun anggaran aktif.</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Atur Tahun Anggaran</a>
  </div>
{:else}
  <div class="card" style="max-width:520px; padding:24px;">
    <div style="display:flex; flex-direction:column; gap:18px;">

      <!-- Jenis -->
      <div>
        <div class="label" style="margin-bottom:8px;">Jenis Dokumen</div>
        <div style="display:flex; flex-wrap:wrap; gap:8px;">
          {#each jenisList as item}
            <button
              onclick={() => jenis = item.value}
              class="btn {jenis === item.value ? 'btn-primary' : 'btn-ghost'}"
              style="font-size:12px;"
            >
              {item.label}
            </button>
          {/each}
        </div>
      </div>

      <!-- Nomor SE -->
      <div>
        <div class="label" style="margin-bottom:8px;">Nomor Surat Edaran</div>
        <input
          type="text"
          placeholder="Contoh: 900/123/BPKPD/2025"
          bind:value={nomor_se}
          class="input"
        />
      </div>

      <!-- Tanggal SE -->
      <div>
        <div class="label" style="margin-bottom:8px;">Tanggal Surat Edaran</div>
        <input
          type="date"
          bind:value={tanggal_se}
          class="input"
          style="max-width:200px;"
        />
      </div>

      <!-- Catatan -->
      <div>
        <div class="label" style="margin-bottom:8px;">Catatan <span style="color:var(--text-3); font-weight:400; text-transform:none; letter-spacing:0;">(opsional)</span></div>
        <textarea
          placeholder="Keterangan tambahan..."
          bind:value={catatan}
          class="input"
          style="height:80px; resize:vertical;"
        ></textarea>
      </div>

      <!-- Info tahun -->
      <div style="padding:12px; background:var(--surface-2); border-radius:6px; border:1px solid var(--border);">
        <span style="font-size:12px; color:var(--text-3);">Tahun Anggaran: </span>
        <span style="font-size:12px; font-weight:600; color:var(--text);">{tahunAktif.tahun}</span>
      </div>

      <!-- Actions -->
      <div style="display:flex; gap:10px; padding-top:4px;">
        <button onclick={simpan} disabled={submitting} class="btn btn-primary">
          {submitting ? "Menyimpan..." : "Buat Dokumen"}
        </button>
        <a href="/dokumen" class="btn btn-ghost">Batal</a>
      </div>

    </div>
  </div>
{/if}
