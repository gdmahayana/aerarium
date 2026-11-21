<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "$lib/stores/toast";
  import { onMount } from "svelte";

  interface KodeBarang {
    id: number;
    kode: string;
    uraian: string;
  }

  let list = $state<KodeBarang[]>([]);
  let loading = $state(true);
  let cari = $state("");
  let showForm = $state(false);
  let submitting = $state(false);
  let form = $state({ kode: "", uraian: "" });

  onMount(async () => {
    try {
      const rows = await invoke<[number, string, string][]>("get_kode_barang");
      list = rows.map(([id, kode, uraian]) => ({ id, kode, uraian }));
    } catch(e) { toast.error(String(e)); }
    finally { loading = false; }
  });

  let filtered = $derived(
    cari === ""
      ? list
      : list.filter(k =>
          k.kode.toLowerCase().includes(cari.toLowerCase()) ||
          k.uraian.toLowerCase().includes(cari.toLowerCase())
        )
  );

  // Kelompokkan berdasarkan 4 segmen pertama: 1.1.xx.xx
  let grouped = $derived(() => {
    const map = new Map<string, { label: string; items: KodeBarang[] }>();
    for (const item of filtered) {
      const seg = item.kode.split(".");
      const groupKey = seg.slice(0, 4).join(".");
      if (!map.has(groupKey)) map.set(groupKey, { label: groupKey, items: [] });
      map.get(groupKey)!.items.push(item);
    }
    return [...map.entries()].sort(([a], [b]) => a.localeCompare(b));
  });

  async function tambah() {
    if (!form.kode || !form.uraian) return;
    submitting = true;
    try {
      await invoke("tambah_kode_barang", { kode: form.kode, uraian: form.uraian });
      const rows = await invoke<[number, string, string][]>("get_kode_barang");
      list = rows.map(([id, kode, uraian]) => ({ id, kode, uraian }));
      form = { kode: "", uraian: "" };
      showForm = false;
      toast.success("Kode barang berhasil ditambahkan");
    } catch(e) { toast.error(String(e)); }
    finally { submitting = false; }
  }

  async function hapus(id: number, uraian: string) {
    if (!confirm("Hapus kode barang: " + uraian + "?")) return;
    try {
      await invoke("hapus_kode_barang", { id });
      list = list.filter(k => k.id !== id);
      toast.success("Kode barang dihapus");
    } catch(e) { toast.error(String(e)); }
  }
  function klasifikasi(kode: string): { label: string; warna: string } {
    const seg = kode.split('.');
    const prefix = seg[0];
    const sub = seg.slice(0, 2).join('.');
    if (prefix === '1') {
      if (sub === '1.1') return { label: 'Persediaan', warna: '#065f46' };
      if (sub === '1.3') return { label: 'Peralatan & Mesin', warna: '#6b21a8' };
      if (sub === '1.4') return { label: 'Gedung & Bangunan', warna: '#92400e' };
      if (sub === '1.5') return { label: 'Jalan & Jaringan', warna: '#1e3a5f' };
      if (sub === '1.6') return { label: 'Aset Tetap Lainnya', warna: '#374151' };
      return { label: 'Aset', warna: '#1e3a5f' };
    }
    if (prefix === '8') return { label: 'Beban', warna: '#7c3aed' };
    return { label: 'Lainnya', warna: '#374151' };
  }
</script>

<div class="page-header">
  <h2>Kode Kelompok Barang</h2>
  <p>Referensi kode kelompok dari SIPD — kode <strong>1.x</strong> (Aset/BMD) dan kode <strong>8.x</strong> (Beban akrual). Terisi otomatis saat import SHS Resmi.</p>
</div>

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else}
  <div style="display:flex; align-items:center; gap:10px; margin-bottom:16px; flex-wrap:wrap;">
    <input type="text" placeholder="Cari kode atau uraian..." bind:value={cari} class="input" style="flex:1; min-width:200px; max-width:360px;" />
    <span style="font-size:12px; color:var(--text-3); flex:1;">{filtered.length} dari {list.length} kode barang</span>
    <button onclick={() => showForm = !showForm} class="btn btn-primary" style="font-size:12px;">+ Tambah Manual</button>
  </div>

  {#if showForm}
    <div class="card" style="padding:16px; margin-bottom:16px; border:1px solid var(--accent);">
      <div style="font-size:12px; font-weight:600; color:var(--text-2); margin-bottom:12px;">Tambah Kode Barang</div>
      <div style="display:flex; gap:10px; flex-wrap:wrap; align-items:flex-end;">
        <div style="flex:0 0 220px;">
          <div class="label" style="margin-bottom:4px;">Kode</div>
          <input class="input" bind:value={form.kode} placeholder="1.1.xx.xx.xx.xxxx" style="font-family:monospace;" />
        </div>
        <div style="flex:1; min-width:200px;">
          <div class="label" style="margin-bottom:4px;">Uraian</div>
          <input class="input" bind:value={form.uraian} placeholder="Nama kelompok barang..." autofocus />
        </div>
        <div style="display:flex; gap:8px;">
          <button onclick={tambah} disabled={submitting || !form.kode || !form.uraian} class="btn btn-primary">
            {submitting ? "Menyimpan..." : "Simpan"}
          </button>
          <button onclick={() => showForm = false} class="btn btn-ghost">Batal</button>
        </div>
      </div>
    </div>
  {/if}

  {#if filtered.length === 0}
    <div class="card" style="padding:48px; text-align:center;">
      <p style="font-size:13px; color:var(--text-3);">
        {list.length === 0 ? "Belum ada kode barang. Import SHS Resmi untuk mengisi otomatis." : "Tidak ada hasil untuk pencarian ini."}
      </p>
    </div>
  {:else}
    <div style="display:flex; flex-direction:column; gap:12px;">
      {#each grouped() as [groupKey, group]}
        <div class="card" style="overflow:hidden;">
          <div style="padding:10px 16px; background:var(--surface-2); border-bottom:1px solid var(--border); display:flex; align-items:center; gap:10px;">
            <span style="font-family:monospace; font-size:11px; font-weight:700; color:var(--accent);">{groupKey}</span>
            {#if group.items[0]}
              {@const klas = klasifikasi(group.items[0].kode)}
              <span style="font-size:10px; font-weight:600; padding:2px 8px; border-radius:4px; background:{klas.warna}; color:#fff;">{klas.label}</span>
            {/if}
            <span style="font-size:11px; color:var(--text-3); margin-left:auto;">{group.items.length} item</span>
          </div>
          <table style="width:100%; border-collapse:collapse;">
            <tbody>
              {#each group.items as item, i}
                <tr style="border-bottom:1px solid var(--border); {i % 2 === 1 ? "background:rgba(255,255,255,0.02);" : ""}">
                  <td style="padding:8px 16px; font-family:monospace; font-size:12px; color:var(--text-3); width:220px;">{item.kode}</td>
                  <td style="padding:8px 16px; font-size:13px; color:var(--text);">{item.uraian}</td>
                  <td style="padding:8px 16px; text-align:right;">
                    <button onclick={() => hapus(item.id, item.uraian)} class="btn btn-ghost" style="font-size:11px; color:var(--text-3);">Hapus</button>
                  </td>
                </tr>
              {/each}
            </tbody>
          </table>
        </div>
      {/each}
    </div>
  {/if}
{/if}
