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
    koef2: number | null;
    satuan2: string | null;
    koef3: number | null;
    satuan3: string | null;
    koef4: number | null;
    satuan4: string | null;
    harga: number;
    ppn: number;
    realisasi: number;
    dikunci: boolean;
  }

  interface RincianKeterangan {
    id: number;
    label: string;
    items: RincianItem[];
  }

  interface RincianPengelompokan {
    id: number;
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

  const dokumenId = $derived(Number($page.params.id));
  const subId = $derived(Number($page.params.sid));

  let rekeningList = $state<RincianRekening[]>([]);
  let loading = $state(true);
  let error = $state("");
  let submitting = $state(false);

  // Form rekening
  let showFormRekening = $state(false);
  let fRekKode = $state("");
  let fRekUraian = $state("");

  // Form pengelompokan
  let activeRekId = $state<number | null>(null);
  let showFormPg = $state(false);
  let fPgLabel = $state("");

  // Form keterangan
  let activePgId = $state<number | null>(null);
  let showFormKet = $state(false);
  let fKetLabel = $state("");

  // Form item (modal)
  let showModalItem = $state(false);
  let activeKetId = $state<number | null>(null);
  let koefCount = $state(1);
  let fItem = $state({
    uraian: "", spesifikasi: "",
    volume: "1", satuan: "",
    koef2: "", satuan2: "",
    koef3: "", satuan3: "",
    koef4: "", satuan4: "",
    harga: "", ppn: false,
  });

  let totalItem = $derived(() => {
    const v1 = parseFloat(fItem.volume) || 0;
    const v2 = fItem.koef2 ? (parseFloat(fItem.koef2) || 1) : 1;
    const v3 = fItem.koef3 ? (parseFloat(fItem.koef3) || 1) : 1;
    const v4 = fItem.koef4 ? (parseFloat(fItem.koef4) || 1) : 1;
    const h = parseFloat(fItem.harga) || 0;
    const total = v1 * v2 * v3 * v4 * h;
    return fItem.ppn ? total * 1.11 : total;
  });

  function koefString(item: RincianItem) {
    let parts = [item.volume + " " + item.satuan];
    if (item.koef2 && item.satuan2) parts.push(item.koef2 + " " + item.satuan2);
    if (item.koef3 && item.satuan3) parts.push(item.koef3 + " " + item.satuan3);
    if (item.koef4 && item.satuan4) parts.push(item.koef4 + " " + item.satuan4);
    return parts.join(" × ");
  }

  function totalItemCalc(item: RincianItem) {
    const v1 = item.volume;
    const v2 = item.koef2 || 1;
    const v3 = item.koef3 || 1;
    const v4 = item.koef4 || 1;
    const total = v1 * v2 * v3 * v4 * item.harga;
    return item.ppn > 0 ? total * 1.11 : total;
  }

  function totalKeterangan(ket: RincianKeterangan) {
    return ket.items.reduce((s, i) => s + totalItemCalc(i), 0);
  }

  function totalPengelompokan(pg: RincianPengelompokan) {
    return pg.keteranganList.reduce((s, k) => s + totalKeterangan(k), 0);
  }

  function totalSemua() {
    return rekeningList.reduce((s, r) => s + r.pengelompokanList.reduce((s2, p) => s2 + totalPengelompokan(p), 0), 0);
  }

  function formatRp(n: number) {
    return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(n);
  }

  async function muat() {
    try {
      const rekList = await invoke<any[]>("get_rincian_rekening", { dokumenSubKegiatanId: subId });
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
    } catch(e) { error = String(e); }
  }

  onMount(async () => { await muat(); loading = false; });

  async function tambahRekening() {
    if (!fRekKode || !fRekUraian) return;
    submitting = true;
    try {
      const rekId = await invoke<number>("cari_atau_buat_rekening", { kode: fRekKode, uraian: fRekUraian });
      await invoke("tambah_rincian_rekening", { input: { dokumen_sub_kegiatan_id: subId, rekening_id: rekId } });
      fRekKode = ""; fRekUraian = ""; showFormRekening = false;
      await muat();
    } catch(e) { error = String(e); } finally { submitting = false; }
  }

  async function tambahPengelompokan() {
    if (!fPgLabel || !activeRekId) return;
    submitting = true;
    try {
      await invoke("tambah_rincian_pengelompokan", { input: { rincian_rekening_id: activeRekId, label: fPgLabel } });
      fPgLabel = ""; showFormPg = false;
      await muat();
    } catch(e) { error = String(e); } finally { submitting = false; }
  }

  async function tambahKeterangan() {
    if (!fKetLabel || !activePgId) return;
    submitting = true;
    try {
      await invoke("tambah_rincian_keterangan", { input: { rincian_pengelompokan_id: activePgId, label: fKetLabel } });
      fKetLabel = ""; showFormKet = false;
      await muat();
    } catch(e) { error = String(e); } finally { submitting = false; }
  }

  function bukaModalItem(ketId: number) {
    activeKetId = ketId;
    koefCount = 1;
    fItem = { uraian: "", spesifikasi: "", volume: "1", satuan: "", koef2: "", satuan2: "", koef3: "", satuan3: "", koef4: "", satuan4: "", harga: "", ppn: false };
    showModalItem = true;
  }

  async function simpanItem() {
    if (!fItem.uraian || !fItem.satuan || !fItem.harga || !activeKetId) return;
    submitting = true;
    try {
      await invoke("tambah_rincian_item", {
        input: {
          rincian_keterangan_id: activeKetId,
          shs_id: null,
          uraian: fItem.uraian,
          spesifikasi: fItem.spesifikasi || null,
          volume: parseFloat(fItem.volume) || 1,
          satuan: fItem.satuan,
          koef2: fItem.koef2 ? parseFloat(fItem.koef2) : null,
          satuan2: fItem.satuan2 || null,
          koef3: fItem.koef3 ? parseFloat(fItem.koef3) : null,
          satuan3: fItem.satuan3 || null,
          koef4: fItem.koef4 ? parseFloat(fItem.koef4) : null,
          satuan4: fItem.satuan4 || null,
          harga: parseFloat(fItem.harga),
          ppn: fItem.ppn ? 11 : null,
        }
      });
      showModalItem = false;
      await muat();
    } catch(e) { error = String(e); } finally { submitting = false; }
  }

  async function hapusItem(id: number) {
    if (!confirm("Hapus item ini?")) return;
    try { await invoke("hapus_rincian_item", { id }); await muat(); }
    catch(e) { error = String(e); }
  }
</script>

<!-- Modal Item -->
{#if showModalItem}
  <div style="position:fixed; inset:0; background:rgba(0,0,0,0.6); z-index:100; display:flex; align-items:center; justify-content:center;" onclick={() => showModalItem = false}>
    <div style="background:var(--surface); border:1px solid var(--border); border-radius:10px; width:560px; max-height:90vh; overflow-y:auto; padding:24px;" onclick={(e) => e.stopPropagation()}>
      <div style="display:flex; justify-content:space-between; align-items:center; margin-bottom:20px;">
        <h3 style="font-size:15px; font-weight:700; color:var(--text); margin:0;">Tambah Item Belanja</h3>
        <button onclick={() => showModalItem = false} style="background:none; border:none; color:var(--text-3); font-size:18px; cursor:pointer;">×</button>
      </div>

      <div style="display:flex; flex-direction:column; gap:14px;">
        <!-- Komponen -->
        <div>
          <div class="label" style="margin-bottom:6px;">Komponen <span style="color:var(--accent);">*</span></div>
          <input class="input" bind:value={fItem.uraian} placeholder="Nama barang / jasa..." autofocus />
        </div>
        <div>
          <div class="label" style="margin-bottom:6px;">Spesifikasi</div>
          <input class="input" bind:value={fItem.spesifikasi} placeholder="Deskripsi teknis..." />
        </div>

        <!-- Koefisien -->
        <div style="background:var(--surface-2); border-radius:8px; padding:14px;">
          <div class="label" style="margin-bottom:10px;">Koefisien (Volume)</div>

          <!-- Koef 1 -->
          <div style="display:flex; gap:8px; margin-bottom:8px; align-items:center;">
            <span style="font-size:11px; color:var(--text-3); width:16px;">1</span>
            <input class="input" style="flex:1;" type="number" bind:value={fItem.volume} placeholder="Volume" />
            <input class="input" style="flex:1;" bind:value={fItem.satuan} placeholder="Satuan (Kotak, Lembar...)" />
          </div>

          <!-- Koef 2 -->
          {#if koefCount >= 2}
            <div style="display:flex; gap:8px; margin-bottom:8px; align-items:center;">
              <span style="font-size:11px; color:var(--text-3); width:16px;">×</span>
              <input class="input" style="flex:1;" type="number" bind:value={fItem.koef2} placeholder="Volume 2" />
              <input class="input" style="flex:1;" bind:value={fItem.satuan2} placeholder="Satuan 2" />
            </div>
          {/if}

          <!-- Koef 3 -->
          {#if koefCount >= 3}
            <div style="display:flex; gap:8px; margin-bottom:8px; align-items:center;">
              <span style="font-size:11px; color:var(--text-3); width:16px;">×</span>
              <input class="input" style="flex:1;" type="number" bind:value={fItem.koef3} placeholder="Volume 3" />
              <input class="input" style="flex:1;" bind:value={fItem.satuan3} placeholder="Satuan 3" />
            </div>
          {/if}

          <!-- Koef 4 -->
          {#if koefCount >= 4}
            <div style="display:flex; gap:8px; margin-bottom:8px; align-items:center;">
              <span style="font-size:11px; color:var(--text-3); width:16px;">×</span>
              <input class="input" style="flex:1;" type="number" bind:value={fItem.koef4} placeholder="Volume 4" />
              <input class="input" style="flex:1;" bind:value={fItem.satuan4} placeholder="Satuan 4" />
            </div>
          {/if}

          {#if koefCount < 4}
            <button class="btn btn-ghost" style="font-size:11px; margin-top:4px;" onclick={() => koefCount++}>+ Koefisien</button>
          {/if}
        </div>

        <!-- Harga -->
        <div style="display:flex; gap:12px; align-items:flex-end;">
          <div style="flex:1;">
            <div class="label" style="margin-bottom:6px;">Harga Satuan <span style="color:var(--accent);">*</span></div>
            <input class="input" type="number" bind:value={fItem.harga} placeholder="0" />
          </div>
          <div style="padding-bottom:8px;">
            <label style="display:flex; align-items:center; gap:8px; font-size:12px; color:var(--text-2); cursor:pointer;">
              <input type="checkbox" bind:checked={fItem.ppn} />
              PPN 11%
            </label>
          </div>
        </div>

        <!-- Preview total -->
        {#if fItem.harga}
          <div style="background:var(--surface-2); border-radius:6px; padding:12px; display:flex; justify-content:space-between; align-items:center;">
            <div style="font-size:12px; color:var(--text-3);">
              {fItem.volume || 0} {fItem.satuan}
              {#if fItem.koef2} × {fItem.koef2} {fItem.satuan2}{/if}
              {#if fItem.koef3} × {fItem.koef3} {fItem.satuan3}{/if}
              {#if fItem.koef4} × {fItem.koef4} {fItem.satuan4}{/if}
              × {formatRp(parseFloat(fItem.harga) || 0)}
              {#if fItem.ppn} + PPN{/if}
            </div>
            <div style="font-size:15px; font-weight:700; color:var(--text);">
              {formatRp(totalItem())}
            </div>
          </div>
        {/if}

        <div style="display:flex; gap:10px; padding-top:4px;">
          <button onclick={simpanItem} disabled={submitting || !fItem.uraian || !fItem.satuan || !fItem.harga} class="btn btn-primary">
            {submitting ? "Menyimpan..." : "Simpan"}
          </button>
          <button onclick={() => showModalItem = false} class="btn btn-ghost">Batal</button>
        </div>
      </div>
    </div>
  </div>
{/if}

<!-- Header -->
<div class="page-header">
  <div style="display:flex; align-items:center; gap:10px;">
    <a href="/dokumen/{dokumenId}" class="btn btn-ghost" style="font-size:12px;">← Sub Kegiatan</a>
    <h2>Rincian Belanja</h2>
  </div>
  <div style="display:flex; align-items:center; gap:12px;">
    {#if totalSemua() > 0}
      <span style="font-size:13px; font-weight:600; color:var(--text-2);">{formatRp(totalSemua())}</span>
    {/if}
    <button onclick={() => { showFormRekening = !showFormRekening; }} class="btn btn-primary" style="font-size:12px;">+ Rekening</button>
  </div>
</div>

{#if error}
  <div class="alert-error">{error}<button onclick={() => error = ""} style="margin-left:8px; background:none; border:none; color:inherit; cursor:pointer; text-decoration:underline;">Tutup</button></div>
{/if}

<!-- Form Rekening -->
{#if showFormRekening}
  <div class="card" style="padding:16px; margin-bottom:16px; max-width:600px;">
    <div style="display:flex; gap:10px; align-items:flex-end;">
      <div style="flex:0 0 200px;">
        <div class="label" style="margin-bottom:6px;">Kode Rekening</div>
        <input class="input" bind:value={fRekKode} placeholder="5.1.02.01.001" />
      </div>
      <div style="flex:1;">
        <div class="label" style="margin-bottom:6px;">Uraian</div>
        <input class="input" bind:value={fRekUraian} placeholder="Belanja Bahan Habis Pakai..." />
      </div>
      <button onclick={tambahRekening} disabled={submitting} class="btn btn-primary">Simpan</button>
      <button onclick={() => showFormRekening = false} class="btn btn-ghost">Batal</button>
    </div>
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if rekeningList.length === 0}
  <div class="card" style="padding:48px; text-align:center;">
    <p style="font-size:13px; color:var(--text-3); margin-bottom:8px;">Belum ada rincian belanja.</p>
    <p style="font-size:12px; color:var(--text-3);">Mulai dengan menambah kode rekening.</p>
  </div>
{:else}
  {#each rekeningList as rek}
    <div class="card" style="margin-bottom:16px; overflow:hidden;">

      <!-- Header Rekening -->
      <div style="padding:12px 20px; background:var(--surface-2); border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;">
        <div style="display:flex; align-items:baseline; gap:12px;">
          <span style="font-size:11px; font-family:monospace; color:var(--text-3);">{rek.rekening_kode}</span>
          <span style="font-size:13px; font-weight:600; color:var(--text);">{rek.rekening_uraian}</span>
        </div>
        <button onclick={() => { activeRekId = rek.id; showFormPg = true; activePgId = null; showFormKet = false; }} class="btn btn-ghost" style="font-size:11px;">+ Pengelompokan</button>
      </div>

      <!-- Form Pengelompokan -->
      {#if showFormPg && activeRekId === rek.id}
        <div style="padding:12px 20px; border-bottom:1px solid var(--border); background:var(--surface-2);">
          <div style="display:flex; gap:8px; align-items:flex-end;">
            <div style="flex:1;">
              <div class="label" style="margin-bottom:4px;"># Label Pengelompokan</div>
              <input class="input" bind:value={fPgLabel} placeholder="Belanja ATK, Fotocopy, dst..." autofocus />
            </div>
            <button onclick={tambahPengelompokan} disabled={submitting} class="btn btn-primary">Simpan</button>
            <button onclick={() => showFormPg = false} class="btn btn-ghost">Batal</button>
          </div>
        </div>
      {/if}

      {#each rek.pengelompokanList as pg}
        <!-- Header Pengelompokan -->
        <div style="padding:10px 20px; border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center;">
          <div style="display:flex; align-items:center; gap:8px;">
            <span style="font-size:11px; color:var(--text-3); font-weight:600;">[#]</span>
            <span style="font-size:12px; font-weight:600; color:var(--text);">{pg.label}</span>
            {#if pg.sumber_dana_nama}
              <span style="font-size:11px; color:var(--text-3);">— {pg.sumber_dana_nama}</span>
            {/if}
          </div>
          <div style="display:flex; gap:10px; align-items:center;">
            <span style="font-size:12px; color:var(--text-2);">{formatRp(totalPengelompokan(pg))}</span>
            <button onclick={() => { activePgId = pg.id; showFormKet = true; activeRekId = null; showFormPg = false; }} class="btn btn-ghost" style="font-size:11px;">+ Keterangan</button>
          </div>
        </div>

        <!-- Form Keterangan -->
        {#if showFormKet && activePgId === pg.id}
          <div style="padding:10px 20px 10px 44px; border-bottom:1px solid var(--border); background:var(--surface-2);">
            <div style="display:flex; gap:8px; align-items:flex-end;">
              <div style="flex:1;">
                <div class="label" style="margin-bottom:4px;">- Label Keterangan</div>
                <input class="input" bind:value={fKetLabel} placeholder="Alat Tulis Kantor, Transport, dst..." autofocus />
              </div>
              <button onclick={tambahKeterangan} disabled={submitting} class="btn btn-primary">Simpan</button>
              <button onclick={() => showFormKet = false} class="btn btn-ghost">Batal</button>
            </div>
          </div>
        {/if}

        {#each pg.keteranganList as ket}
          <!-- Header Keterangan -->
          <div style="padding:8px 20px 8px 44px; border-bottom:1px solid var(--border); display:flex; justify-content:space-between; align-items:center; background:rgba(255,255,255,0.01);">
            <div style="display:flex; align-items:center; gap:8px;">
              <span style="font-size:11px; color:var(--text-3);">[-]</span>
              <span style="font-size:12px; color:var(--text-2);">{ket.label}</span>
            </div>
            <div style="display:flex; gap:10px; align-items:center;">
              <span style="font-size:12px; color:var(--text-2);">{formatRp(totalKeterangan(ket))}</span>
              <button onclick={() => bukaModalItem(ket.id)} class="btn btn-ghost" style="font-size:11px;">+ Item</button>
            </div>
          </div>

          <!-- Item List -->
          {#each ket.items as item, idx}
            <div style="padding:10px 20px 10px 64px; {idx < ket.items.length - 1 ? 'border-bottom:1px solid var(--border);' : ''} display:flex; justify-content:space-between; align-items:flex-start;">
              <div style="flex:1;">
                <div style="font-size:13px; color:var(--text); font-weight:500;">{item.uraian}</div>
                {#if item.spesifikasi}
                  <div style="font-size:11px; color:var(--text-3); margin-top:2px;">{item.spesifikasi}</div>
                {/if}
                <div style="font-size:12px; color:var(--text-3); margin-top:4px; font-family:monospace;">
                  {koefString(item)} × {formatRp(item.harga)}
                  {#if item.ppn > 0}<span style="color:var(--accent); margin-left:4px;">+PPN</span>{/if}
                </div>
              </div>
              <div style="display:flex; align-items:center; gap:12px;">
                <span style="font-size:13px; font-weight:600; color:var(--text); font-variant-numeric:tabular-nums;">{formatRp(totalItemCalc(item))}</span>
                <button onclick={() => hapusItem(item.id)} class="btn btn-ghost" style="font-size:11px; color:var(--text-3);">Hapus</button>
              </div>
            </div>
          {/each}
        {/each}
      {/each}
    </div>
  {/each}
{/if}
