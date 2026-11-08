<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface Ssh {
    id: number;
    tahun_anggaran_id: number;
    kode: string;
    uraian: string;
    satuan: string;
    harga: number;
    created_at: string;
  }

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
  }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let sshList = $state<Ssh[]>([]);
  let loading = $state(true);
  let error = $state("");
  let cari = $state("");
  let importing = $state(false);

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

  let sshTampil = $derived(
    sshList.filter(s =>
      s.uraian.toLowerCase().includes(cari.toLowerCase()) ||
      s.kode.toLowerCase().includes(cari.toLowerCase())
    )
  );

  function formatRupiah(n: number) {
    return new Intl.NumberFormat("id-ID", {
      style: "currency",
      currency: "IDR",
      minimumFractionDigits: 0,
    }).format(n);
  }

  async function importExcel() {
    if (!tahunAktif) return;
    try {
      importing = true;
      // Tauri dialog untuk pilih file
      const { open } = await import("@tauri-apps/plugin-dialog/dist-js/index.js");
      const filePath = await open({
        filters: [{ name: "Excel", extensions: ["xlsx", "xls"] }],
      });
      if (!filePath) return;
      const count = await invoke<number>("import_ssh", {
        filePath: filePath as string,
        tahunAnggaranId: tahunAktif.id,
      });
      alert(`Berhasil import ${count} item SSH`);
    } catch (e) {
      error = String(e);
    } finally {
      importing = false;
    }
  }
</script>

<div class="page-header">
  <h2>Database SSH</h2>
  <p>Standar Satuan Harga — referensi harga untuk penyusunan rincian belanja</p>
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
  <div class="card" style="padding:48px; text-align:center; max-width:400px; margin:60px auto;">
    <p style="font-size:13px; color:var(--text-2); margin-bottom:16px;">Belum ada tahun anggaran aktif.</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Atur Tahun Anggaran</a>
  </div>
{:else}
  <div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:20px; gap:12px;">
    <input
      type="text"
      placeholder="Cari kode atau uraian..."
      bind:value={cari}
      class="input"
      style="max-width:320px;"
    />
    <button onclick={importExcel} disabled={importing} class="btn btn-primary">
      {importing ? "Mengimpor..." : "Import dari Excel"}
    </button>
  </div>

  {#if sshList.length === 0}
    <div class="card" style="padding:48px; text-align:center;">
      <p style="font-size:13px; color:var(--text-2); margin-bottom:4px;">Belum ada data SSH.</p>
      <p style="font-size:12px; color:var(--text-3); margin-bottom:20px;">Import file Excel SSH/SHBJ dari SIPD</p>
      <button onclick={importExcel} class="btn btn-secondary">Import Excel</button>
    </div>
  {:else}
    <div style="font-size:12px; color:var(--text-3); margin-bottom:12px;">
      Menampilkan {sshTampil.length} dari {sshList.length} item
    </div>
    <div class="card" style="overflow:hidden;">
      <table style="width:100%; border-collapse:collapse;">
        <thead>
          <tr style="border-bottom:1px solid var(--border);">
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; text-transform:uppercase; letter-spacing:0.06em; color:var(--text-3);">Kode</th>
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; text-transform:uppercase; letter-spacing:0.06em; color:var(--text-3);">Uraian</th>
            <th style="padding:10px 16px; text-align:left; font-size:11px; font-weight:600; text-transform:uppercase; letter-spacing:0.06em; color:var(--text-3);">Satuan</th>
            <th style="padding:10px 16px; text-align:right; font-size:11px; font-weight:600; text-transform:uppercase; letter-spacing:0.06em; color:var(--text-3);">Harga</th>
          </tr>
        </thead>
        <tbody>
          {#each sshTampil as item, i}
            <tr style="border-bottom:1px solid var(--border); {i % 2 === 1 ? 'background:rgba(255,255,255,0.02);' : ''} transition:background 0.1s;"
              onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = 'var(--surface-2)'}
              onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = i % 2 === 1 ? 'rgba(255,255,255,0.02)' : 'transparent'}
            >
              <td style="padding:10px 16px; font-size:12px; color:var(--text-3); font-family:monospace;">{item.kode}</td>
              <td style="padding:10px 16px; font-size:13px; color:var(--text);">{item.uraian}</td>
              <td style="padding:10px 16px; font-size:12px; color:var(--text-2);">{item.satuan}</td>
              <td style="padding:10px 16px; font-size:13px; color:var(--text); text-align:right; font-variant-numeric:tabular-nums;">{formatRupiah(item.harga)}</td>
            </tr>
          {/each}
        </tbody>
      </table>
    </div>
  {/if}
{/if}