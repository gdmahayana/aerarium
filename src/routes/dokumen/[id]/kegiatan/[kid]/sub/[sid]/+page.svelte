<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";

  interface RincianItem {
    id: number;
    rincian_keterangan_id: number;
    uraian: string;
    spesifikasi: string | null;
    volume: number;
    satuan: string;
    harga: number;
    ppn: number;
    realisasi: number;
    dikunci: boolean;
  }

  interface RincianKeterangan {
    id: number;
    rincian_pengelompokan_id: number;
    label: string;
    items: RincianItem[];
  }

  interface RincianPengelompokan {
    id: number;
    rincian_rekening_id: number;
    label: string;
    jenis: string;
    sumber_dana_nama: string | null;
    keteranganList: RincianKeterangan[];
  }

  interface RincianRekening {
    id: number;
    rekening_kode: string | null;
    rekening_uraian: string | null;
    pengelompokanList: RincianPengelompokan[];
  }

  let rekeningList = $state<RincianRekening[]>([]);
  let loading = $state(true);
  let error = $state("");

  const dokumenId = $derived(Number($page.params.id));
  const kegiatanId = $derived(Number($page.params.kid));
  const subKegiatanId = $derived(Number($page.params.sid));

  // Form state
  let showFormRekening = $state(false);
  let kodeRekening = $state("");
  let uraianRekening = $state("");

  let activeRekeningId = $state<number | null>(null);
  let showFormPengelompokan = $state(false);
  let labelPengelompokan = $state("");

  let activeKeteranganParentId = $state<number | null>(null);
  let showFormKeterangan = $state(false);
  let labelKeterangan = $state("");

  let activeItemParentId = $state<number | null>(null);
  let showFormItem = $state(false);
  let itemUraian = $state("");
  let itemSpek = $state("");
  let itemVolume = $state("1");
  let itemSatuan = $state("");
  let itemHarga = $state("");
  let submitting = $state(false);

  function formatRupiah(n: number) {
    return new Intl.NumberFormat("id-ID", {
      style: "currency", currency: "IDR", minimumFractionDigits: 0,
    }).format(n);
  }

  function jumlah(item: RincianItem) {
    return item.volume * item.harga;
  }

  function totalKeterangan(ket: RincianKeterangan) {
    return ket.items.reduce((s, i) => s + jumlah(i), 0);
  }

  function totalPengelompokan(p: RincianPengelompokan) {
    return p.keteranganList.reduce((s, k) => s + totalKeterangan(k), 0);
  }

  async function muat() {
    try {
      const rekList = await invoke<any[]>("get_rincian_rekening", { subKegiatanId });
      const result: RincianRekening[] = [];
      for (const rek of rekList) {
        const pgList = await invoke<any[]>("get_rincian_pengelompokan", { rincianRekeningId: rek.id });
        const pengelompokanList: RincianPengelompokan[] = [];
        for (const pg of pgList) {
          const ketList = await invoke<any[]>("get_rincian_keterangan", { rincianPengelompokanId: pg.id });
          const keteranganList: RincianKeterangan[] = [];
          for (const ket of ketList) {
            const items = await invoke<RincianItem[]>("get_rincian_item", { rincianKeteranganId: ket.id });
            keteranganList.push({ ...ket, items });
          }
          pengelompokanList.push({ ...pg, keteranganList });
        }
        result.push({ ...rek, pengelompokanList });
      }
      rekeningList = result;
    } catch (e) {
      error = String(e);
    }
  }

  onMount(async () => {
    await muat();
    loading = false;
  });

  async function tambahRekening() {
    if (!kodeRekening || !uraianRekening) return;
    submitting = true;
    try {
      // Cari atau buat rekening
      const rekId = await invoke<number>("cari_atau_buat_rekening", {
        kode: kodeRekening, uraian: uraianRekening
      });
      await invoke("tambah_rincian_rekening", {
        input: { sub_kegiatan_id: subKegiatanId, rekening_id: rekId }
      });
      kodeRekening = ""; uraianRekening = "";
      showFormRekening = false;
      await muat();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function tambahPengelompokan() {
    if (!labelPengelompokan || !activeRekeningId) return;
    submitting = true;
    try {
      await invoke("tambah_rincian_pengelompokan", {
        input: { rincian_rekening_id: activeRekeningId, label: labelPengelompokan }
      });
      labelPengelompokan = "";
      showFormPengelompokan = false;
      await muat();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function tambahKeterangan() {
    if (!labelKeterangan || !activeKeteranganParentId) return;
    submitting = true;
    try {
      await invoke("tambah_rincian_keterangan", {
        input: { rincian_pengelompokan_id: activeKeteranganParentId, label: labelKeterangan }
      });
      labelKeterangan = "";
      showFormKeterangan = false;
      await muat();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function tambahItem() {
    if (!itemUraian || !itemSatuan || !itemHarga || !activeItemParentId) return;
    submitting = true;
    try {
      await invoke("tambah_rincian_item", {
        input: {
          rincian_keterangan_id: activeItemParentId,
          uraian: itemUraian,
          spesifikasi: itemSpek || null,
          volume: parseFloat(itemVolume) || 1,
          satuan: itemSatuan,
          harga: parseFloat(itemHarga),
          shs_id: null,
        }
      });
      itemUraian = ""; itemSpek = ""; itemVolume = "1"; itemSatuan = ""; itemHarga = "";
      showFormItem = false;
      await muat();
    } catch (e) {
      error = String(e);
    } finally {
      submitting = false;
    }
  }

  async function hapusItem(id: number) {
    if (!confirm("Hapus item ini?")) return;
    try {
      await invoke("hapus_rincian_item", { id });
      await muat();
    } catch (e) {
      error = String(e);
    }
  }
</script>

<div class="page-header">
  <div style="display:flex; align-items:center; gap:10px;">
    <a href="/dokumen/{dokumenId}/kegiatan/{kegiatanId}" class="btn btn-ghost" style="font-size:12px;">← Sub Kegiatan</a>
    <h2>Rincian Belanja</h2>
  </div>
  <button onclick={() => showFormRekening = !showFormRekening} class="btn btn-primary" style="font-size:12px;">
    + Tambah Rekening
  </button>
</div>

{#if error}
  <div class="alert-error">
    {error}
    <button onclick={() => error = ""} style="margin-left:8px; text-decoration:underline; background:none; border:none; color:inherit; cursor:pointer;">Tutup</button>
  </div>
{/if}

{#if showFormRekening}
  <div class="card" style="padding:20px; margin-bottom:16px; max-width:560px;">
    <div style="display:flex; flex-direction:column; gap:12px;">
      <div>
        <div class="label" style="margin-bottom:6px;">Kode Rekening</div>
        <input type="text" placeholder="5.1.02.01.001.00004" bind:value={kodeRekening} class="input" />
      </div>
      <div>
        <div class="label" style="margin-bottom:6px;">Uraian Rekening</div>
        <input type="text" placeholder="Belanja Bahan-Bahan Bakar dan Pelumas" bind:value={uraianRekening} class="input" />
      </div>
      <div style="display:flex; gap:10px;">
        <button onclick={tambahRekening} disabled={submitting} class="btn btn-primary">Simpan</button>
        <button onclick={() => showFormRekening = false} class="btn btn-ghost">Batal</button>
      </div>
    </div>
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if rekeningList.length === 0}
  <div class="card" style="padding:40px; text-align:center;">
    <p style="font-size:13px; color:var(--text-3);">Belum ada rincian belanja.</p>
  </div>
{:else}
  {#each rekeningList as rek}
    <div class="card" style="margin-bottom:16px; overflow:hidden;">
      <!-- Header Rekening -->
      <div style="padding:12px 20px; background:var(--surface-2); border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;">
        <div>
          <span style="font-size:11px; font-family:monospace; color:var(--text-3);">{rek.rekening_kode}</span>
          <span style="font-size:13px; font-weight:600; color:var(--text); margin-left:10px;">{rek.rekening_uraian}</span>
        </div>
        <button onclick={() => { activeRekeningId = rek.id; showFormPengelompokan = true; }} class="btn btn-ghost" style="font-size:11px;">+ Pengelompokan</button>
      </div>

      {#if showFormPengelompokan && activeRekeningId === rek.id}
        <div style="padding:16px 20px; border-bottom:1px solid var(--border); background:var(--surface-2);">
          <div style="display:flex; gap:10px; align-items:flex-end;">
            <div style="flex:1;">
              <div class="label" style="margin-bottom:6px;">Label [ # ]</div>
              <input type="text" placeholder="Belanja Fotocopy" bind:value={labelPengelompokan} class="input" />
            </div>
            <button onclick={tambahPengelompokan} disabled={submitting} class="btn btn-primary">Simpan</button>
            <button onclick={() => showFormPengelompokan = false} class="btn btn-ghost">Batal</button>
          </div>
        </div>
      {/if}

      {#each rek.pengelompokanList as pg}
        <!-- Pengelompokan [ # ] -->
        <div style="padding:10px 20px; border-bottom:1px solid var(--border); background:rgba(255,255,255,0.02); display:flex; justify-content:space-between; align-items:center;">
          <div>
            <span style="font-size:11px; color:var(--text-3); margin-right:6px;">[#]</span>
            <span style="font-size:12px; font-weight:600; color:var(--text);">{pg.label}</span>
            {#if pg.sumber_dana_nama}
              <span style="font-size:11px; color:var(--text-3); margin-left:8px;">— {pg.sumber_dana_nama}</span>
            {/if}
          </div>
          <div style="display:flex; gap:8px; align-items:center;">
            <span style="font-size:12px; color:var(--text-2);">{formatRupiah(totalPengelompokan(pg))}</span>
            <button onclick={() => { activeKeteranganParentId = pg.id; showFormKeterangan = true; }} class="btn btn-ghost" style="font-size:11px;">+ Keterangan</button>
          </div>
        </div>

        {#if showFormKeterangan && activeKeteranganParentId === pg.id}
          <div style="padding:12px 20px 12px 40px; border-bottom:1px solid var(--border); background:var(--surface-2);">
            <div style="display:flex; gap:10px; align-items:flex-end;">
              <div style="flex:1;">
                <div class="label" style="margin-bottom:6px;">Label [ - ]</div>
                <input type="text" placeholder="Belanja Fotocopy" bind:value={labelKeterangan} class="input" />
              </div>
              <button onclick={tambahKeterangan} disabled={submitting} class="btn btn-primary">Simpan</button>
              <button onclick={() => showFormKeterangan = false} class="btn btn-ghost">Batal</button>
            </div>
          </div>
        {/if}

        {#each pg.keteranganList as ket}
          <!-- Keterangan [ - ] -->
          <div style="padding:8px 20px 8px 40px; border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;">
            <div>
              <span style="font-size:11px; color:var(--text-3); margin-right:6px;">[-]</span>
              <span style="font-size:12px; color:var(--text-2);">{ket.label}</span>
            </div>
            <div style="display:flex; gap:8px; align-items:center;">
              <span style="font-size:12px; color:var(--text-2);">{formatRupiah(totalKeterangan(ket))}</span>
              <button onclick={() => { activeItemParentId = ket.id; showFormItem = true; }} class="btn btn-ghost" style="font-size:11px;">+ Item</button>
            </div>
          </div>

          {#if showFormItem && activeItemParentId === ket.id}
            <div style="padding:14px 20px 14px 60px; border-bottom:1px solid var(--border); background:var(--surface-2);">
              <div style="display:flex; flex-direction:column; gap:10px;">
                <div style="display:flex; gap:10px;">
                  <div style="flex:2;">
                    <div class="label" style="margin-bottom:4px;">Uraian</div>
                    <input type="text" placeholder="Nama item" bind:value={itemUraian} class="input" />
                  </div>
                  <div style="flex:2;">
                    <div class="label" style="margin-bottom:4px;">Spesifikasi</div>
                    <input type="text" placeholder="Deskripsi..." bind:value={itemSpek} class="input" />
                  </div>
                </div>
                <div style="display:flex; gap:10px;">
                  <div style="flex:1;">
                    <div class="label" style="margin-bottom:4px;">Volume</div>
                    <input type="number" bind:value={itemVolume} class="input" />
                  </div>
                  <div style="flex:1;">
                    <div class="label" style="margin-bottom:4px;">Satuan</div>
                    <input type="text" placeholder="Lembar" bind:value={itemSatuan} class="input" />
                  </div>
                  <div style="flex:2;">
                    <div class="label" style="margin-bottom:4px;">Harga Satuan</div>
                    <input type="number" placeholder="0" bind:value={itemHarga} class="input" />
                  </div>
                </div>
                <div style="display:flex; gap:10px;">
                  <button onclick={tambahItem} disabled={submitting} class="btn btn-primary">Simpan</button>
                  <button onclick={() => showFormItem = false} class="btn btn-ghost">Batal</button>
                </div>
              </div>
            </div>
          {/if}

          {#each ket.items as item, i}
            <div style="padding:8px 20px 8px 60px; {i < ket.items.length - 1 ? "border-bottom:1px solid var(--border);" : ""} display:flex; justify-content:space-between; align-items:flex-start;">
              <div>
                <div style="font-size:13px; color:var(--text);">{item.uraian}</div>
                {#if item.spesifikasi}
                  <div style="font-size:11px; color:var(--text-3); margin-top:2px;">{item.spesifikasi}</div>
                {/if}
                <div style="font-size:12px; color:var(--text-2); margin-top:2px;">
                  {item.volume} {item.satuan} x {formatRupiah(item.harga)}
                </div>
              </div>
              <div style="display:flex; gap:10px; align-items:center;">
                <span style="font-size:13px; font-weight:600; color:var(--text); font-variant-numeric:tabular-nums;">{formatRupiah(jumlah(item))}</span>
                <button onclick={() => hapusItem(item.id)} class="btn btn-ghost" style="font-size:11px; color:var(--text-3);">Hapus</button>
              </div>
            </div>
          {/each}
        {/each}
      {/each}
    </div>
  {/each}
{/if}
