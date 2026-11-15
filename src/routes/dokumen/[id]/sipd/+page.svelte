<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { page } from "$app/stores";

  interface RincianItem {
    id: number;
    uraian: string;
    spesifikasi: string | null;
    volume: number;
    satuan: string;
    koef2: number | null; satuan2: string | null;
    koef3: number | null; satuan3: string | null;
    koef4: number | null; satuan4: string | null;
    harga: number;
    ppn: number;
    shs_kode: string | null;
    pagu_indikatif_id: number | null;
    realisasi: number;
    dikunci: boolean;
  }
  interface RincianKeterangan { id: number; label: string; items: RincianItem[]; }
  interface RincianPengelompokan { id: number; label: string; jenis: string; keteranganList: RincianKeterangan[]; }
  interface RincianRekening { id: number; rekening_kode: string | null; rekening_uraian: string | null; pengelompokanList: RincianPengelompokan[]; }
  interface DokumenSubKegiatan {
    id: number; master_sub_kegiatan_id: number;
    pagu: number; bebas_shs: boolean;
  }
  interface MasterSubKegiatan { id: number; kode: string; uraian: string; }
  interface Dokumen { id: number; jenis: string; versi: number; }

  const dokumenId = $derived(Number($page.params.id));

  let dokumen = $state<Dokumen | null>(null);
  let dskList = $state<DokumenSubKegiatan[]>([]);
  let masterSubList = $state<MasterSubKegiatan[]>([]);
  let rincianMap = $state<Record<number, RincianRekening[]>>({});
  let loading = $state(true);
  let error = $state("");
  let expandedDsk = $state<Set<number>>(new Set());

  function formatRp(n: number) {
    return new Intl.NumberFormat("id-ID", { style: "currency", currency: "IDR", minimumFractionDigits: 0 }).format(n);
  }

  function totalItemCalc(item: RincianItem) {
    const v = item.volume * (item.koef2 || 1) * (item.koef3 || 1) * (item.koef4 || 1);
    return v * item.harga * (item.ppn > 0 ? 1.11 : 1);
  }

  function koefString(item: RincianItem) {
    let parts = [item.volume + " " + item.satuan];
    if (item.koef2 && item.satuan2) parts.push(item.koef2 + " " + item.satuan2);
    if (item.koef3 && item.satuan3) parts.push(item.koef3 + " " + item.satuan3);
    if (item.koef4 && item.satuan4) parts.push(item.koef4 + " " + item.satuan4);
    return parts.join(" × ");
  }

  function namaSubKegiatan(id: number) {
    const s = masterSubList.find(s => s.id === id);
    return s ? s.kode + " — " + s.uraian : String(id);
  }

  async function muatRincian(dskId: number) {
    if (rincianMap[dskId]) return;
    try {
      const rekList = await invoke<any[]>("get_rincian_rekening", { dokumenSubKegiatanId: dskId });
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
      rincianMap = { ...rincianMap, [dskId]: result };
    } catch(e) { error = String(e); }
  }

  async function toggleDsk(dskId: number) {
    const next = new Set(expandedDsk);
    if (next.has(dskId)) {
      next.delete(dskId);
    } else {
      next.add(dskId);
      await muatRincian(dskId);
    }
    expandedDsk = next;
  }

  async function expandAll() {
    const next = new Set(dskList.map(d => d.id));
    for (const dsk of dskList) {
      await muatRincian(dsk.id);
    }
    expandedDsk = next;
  }

  onMount(async () => {
    try {
      const tahunList = await invoke<any[]>("get_tahun_anggaran");
      const ta = tahunList.find((t: any) => t.aktif);
      if (!ta) return;
      const dokList = await invoke<any[]>("get_dokumen", { tahunAnggaranId: ta.id });
      dokumen = dokList.find((d: any) => d.id === dokumenId) || null;
      [dskList, masterSubList] = await Promise.all([
        invoke("get_dokumen_sub_kegiatan", { dokumenId }),
        invoke("get_master_sub_kegiatan_aktif"),
      ]);
    } catch(e) { error = String(e); }
    finally { loading = false; }
  });
</script>

<!-- Header -->
<div style="display:flex; align-items:center; justify-content:space-between; margin-bottom:24px;">
  <div style="display:flex; align-items:center; gap:12px;">
    <a href="/dokumen/{dokumenId}" style="font-size:12px; color:var(--text-3);">← Dokumen</a>
    <h2 style="font-size:16px; font-weight:700; color:var(--text);">
      View SIPD
      {#if dokumen}<span style="font-size:12px; font-weight:400; color:var(--text-3); margin-left:8px;">{dokumen.jenis} v{dokumen.versi}</span>{/if}
    </h2>
  </div>
  <button onclick={expandAll} class="btn btn-primary" style="font-size:12px;">Buka Semua</button>
</div>

{#if error}
  <div class="alert-error">{error}</div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else}
  <div style="display:flex; flex-direction:column; gap:12px;">
    {#each dskList as dsk, dskIdx}
      {@const expanded = expandedDsk.has(dsk.id)}
      {@const rincian = rincianMap[dsk.id] || []}

      <div style="border:1px solid var(--border); border-radius:8px; overflow:hidden;">
        <!-- Header sub kegiatan -->
        <div
          style="display:flex; align-items:center; justify-content:space-between; padding:12px 16px; background:var(--surface-2); cursor:pointer;"
          onclick={() => toggleDsk(dsk.id)}
        >
          <div style="display:flex; align-items:center; gap:10px;">
            <span style="font-size:12px; color:var(--text-3); font-weight:700;">{dskIdx + 1}.</span>
            <span style="font-size:13px; font-weight:600; color:var(--text);">{namaSubKegiatan(dsk.master_sub_kegiatan_id)}</span>
          </div>
          <div style="display:flex; align-items:center; gap:12px;">
            <span style="font-size:12px; color:var(--accent); font-weight:600;">{formatRp(dsk.pagu)}</span>
            <span style="font-size:12px; color:var(--text-3);">{expanded ? "▾" : "▸"}</span>
          </div>
        </div>

        {#if expanded}
          {#each rincian as rek}
            <!-- Header Rekening -->
            <div style="padding:8px 16px; background:#0ea5e908; border-top:1px solid var(--border); border-bottom:1px solid var(--border);">
              <span style="font-size:11px; font-family:monospace; color:var(--text-3); margin-right:10px;">{rek.rekening_kode}</span>
              <span style="font-size:12px; font-weight:600; color:var(--text);">{rek.rekening_uraian}</span>
            </div>

            {#each rek.pengelompokanList as pg}
              <!-- Header Pengelompokan -->
              <div style="padding:6px 16px 6px 32px; border-bottom:1px solid var(--border); background:rgba(255,255,255,0.01);">
                <span style="font-size:11px; color:var(--text-3); font-weight:600;"># {pg.label}</span>
              </div>

              {#each pg.keteranganList as ket}
                <!-- Header Keterangan -->
                <div style="padding:5px 16px 5px 48px; border-bottom:1px solid var(--border); background:rgba(255,255,255,0.01);">
                  <span style="font-size:11px; color:var(--text-3);">- {ket.label}</span>
                </div>

                <!-- Item list -->
                {#each ket.items as item, idx}
                  <div style="display:flex; align-items:flex-start; gap:12px; padding:10px 16px 10px 64px; border-bottom:1px solid var(--border); {idx % 2 === 0 ? '' : 'background:rgba(255,255,255,0.01);'}">
                    <!-- Nomor -->
                    <div style="min-width:20px; font-size:11px; color:var(--text-3); padding-top:2px;">{idx + 1}</div>

                    <!-- Kode SHS -->
                    <div style="min-width:160px;">
                      {#if item.shs_kode}
                        <div style="font-size:10px; font-family:monospace; background:var(--surface-2); border:1px solid var(--border); border-radius:3px; padding:2px 6px; color:var(--text-3); display:inline-block; margin-bottom:3px;" title="Kode SHS untuk input SIPD">{item.shs_kode}</div>
                      {:else}
                        <div style="font-size:10px; color:var(--text-3); font-style:italic; margin-bottom:3px;">— tanpa SHS</div>
                      {/if}
                    </div>

                    <!-- Detail item -->
                    <div style="flex:1;">
                      <div style="font-size:13px; font-weight:500; color:var(--text);">{item.uraian}</div>
                      {#if item.spesifikasi}
                        <div style="font-size:11px; color:var(--text-3); margin-top:1px;">{item.spesifikasi}</div>
                      {/if}
                      <div style="font-size:11px; color:var(--text-3); margin-top:3px; font-family:monospace;">
                        {koefString(item)} × {formatRp(item.harga)}{item.ppn > 0 ? " +PPN" : ""}
                      </div>
                    </div>

                    <!-- Total -->
                    <div style="min-width:120px; text-align:right;">
                      <div style="font-size:13px; font-weight:600; color:var(--text);">{formatRp(totalItemCalc(item))}</div>
                    </div>
                  </div>
                {/each}
              {/each}
            {/each}
          {/each}
        {/if}
      </div>
    {/each}
  </div>
{/if}
