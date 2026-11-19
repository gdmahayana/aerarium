<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { toast } from "$lib/stores/toast";
  import { onMount } from "svelte";
  import { page } from "$app/stores";
  import { goto } from "$app/navigation";

  interface ShsRencana {
    id: number;
    jenis: string;
    kode: string;
    uraian: string;
    harga: number;
  }

  interface ShsResmi {
    id: number;
    jenis: string;
    kode: string;
    uraian: string;
    spesifikasi: string | null;
    satuan: string;
    harga: number;
  }

  const dokumenId = $derived(Number($page.params.id));

  let loading = $state(true);
  let submitting = $state(false);
  let sukses = $state("");

  let shsRencanaList = $state<ShsRencana[]>([]);
  let tahunAktifId = $state<number | null>(null);

  // Map: shs_rencana_id -> shs_resmi_id yang dipilih
  let pemetaan = $state<Record<number, number>>({});
  // Pencarian per baris
  let cari = $state<Record<number, string>>({});
  // Hasil search per baris
  let shsResults = $state<Record<number, ShsResmi[]>>({});
  // Item yang sudah dipilih per baris
  let dipilihMap = $state<Record<number, ShsResmi>>({});
  let searchTimers = $state<Record<number, ReturnType<typeof setTimeout>>>({});

  async function searchShs(rencanaId: number, jenis: string, query: string) {
    if (query.length < 2) { shsResults = { ...shsResults, [rencanaId]: [] }; return; }
    if (searchTimers[rencanaId]) clearTimeout(searchTimers[rencanaId]);
    searchTimers = { ...searchTimers, [rencanaId]: setTimeout(async () => {
      try {
        const results = await invoke<ShsResmi[]>("search_shs", {
          tahunAnggaranId: tahunAktifId,
          query,
          sumber: "resmi",
          limit: 10,
        });
        shsResults = { ...shsResults, [rencanaId]: results };
      } catch(e) { console.error(e); }
    }, 250)};
  }

  function formatRp(n: number) {
    return new Intl.NumberFormat("id-ID", {
      style: "currency", currency: "IDR", minimumFractionDigits: 0
    }).format(n);
  }

  onMount(async () => {
    try {
      const taList = await invoke<any[]>("get_tahun_anggaran");
      const taAktif = taList.find((t: any) => t.aktif);
      if (!taAktif) return;
      tahunAktifId = taAktif.id;

      const rencana = await invoke<[number, string, string, string, number][]>(
        "get_shs_rencana_dipakai", { dokumenId }
      );
      shsRencanaList = rencana.map(([id, jenis, kode, uraian, harga]) =>
        ({ id, jenis, kode, uraian, harga })
      );

      // Auto-match: cari SHS resmi dengan kode sama via search
      for (const r of shsRencanaList) {
        try {
          const matches = await invoke<ShsResmi[]>("search_shs", {
            tahunAnggaranId: tahunAktifId,
            query: r.kode,
            sumber: "resmi",
            limit: 1,
          });
          if (matches.length > 0 && matches[0].kode === r.kode) {
            pemetaan = { ...pemetaan, [r.id]: matches[0].id };
            dipilihMap = { ...dipilihMap, [r.id]: matches[0] };
          }
        } catch(e) {}
      }
    } catch(e) {
      toast.error(String(e));
    } finally {
      loading = false;
    }
  });

  async function terapkan() {
    const entries = Object.entries(pemetaan)
      .filter(([, v]) => v > 0)
      .map(([k, v]) => [Number(k), Number(v)]);

    if (entries.length === 0) {
      toast.warning("Belum ada pemetaan yang dipilih.");
      return;
    }

    submitting = true;

    try {
      const n = await invoke<number>("terapkan_pemetaan_shs", {
        dokumenId,
        pemetaan: entries,
      });
      sukses = `✅ ${n} item berhasil diperbarui ke SHS Resmi.`;
      setTimeout(() => goto("/dokumen/" + dokumenId), 1500);
    } catch(e) {
      toast.error(String(e));
    } finally {
      submitting = false;
    }
  }
</script>

<div class="page-header">
  <div style="display:flex; align-items:center; gap:10px;">
    <a href="/dokumen/{dokumenId}" class="btn btn-ghost" style="font-size:12px;">← Kembali</a>
    <h2>Petakan SHS Rencana → Resmi</h2>
  </div>
</div>


{#if sukses}
  <div style="padding:12px 16px; background:#d1e7dd; color:#0a3622; border-radius:8px; margin-bottom:16px; font-size:13px;">
    {sukses}
  </div>
{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if shsRencanaList.length === 0}
  <div class="card" style="padding:48px; text-align:center;">
    <p style="font-size:13px; color:var(--text-3);">Tidak ada SHS Rencana yang dipakai di dokumen ini.</p>
    <p style="font-size:12px; color:var(--text-3); margin-top:8px;">Semua item mungkin sudah menggunakan SHS Resmi.</p>
  </div>
{:else}
  <div style="font-size:12px; color:var(--text-3); margin-bottom:16px;">
    Ditemukan <strong>{shsRencanaList.length}</strong> SHS Rencana dipakai. Pilih padanannya dari SHS Resmi, lalu klik Terapkan.
  </div>

  <div class="card" style="overflow:hidden; margin-bottom:20px;">
    <!-- Header tabel -->
    <div style="display:grid; grid-template-columns:1fr 24px 1fr; gap:0; padding:10px 20px; background:var(--surface-2); border-bottom:1px solid var(--border);">
      <span class="label">SHS Rencana</span>
      <span></span>
      <span class="label">SHS Resmi (pilih padanan)</span>
    </div>

    {#each shsRencanaList as r}
      {@const dipilih = pemetaan[r.id] ? (dipilihMap[r.id] || null) : null}
      <div style="display:grid; grid-template-columns:1fr 24px 1fr; gap:0; padding:16px 20px; border-bottom:1px solid var(--border); align-items:start;">

        <!-- Kolom kiri: SHS Rencana -->
        <div>
          <div style="font-size:11px; font-family:monospace; color:var(--text-3);">[{r.jenis}] {r.kode}</div>
          <div style="font-size:13px; font-weight:600; color:var(--text); margin-top:2px;">{r.uraian}</div>
          <div style="font-size:12px; color:var(--text-3); margin-top:2px;">{formatRp(r.harga)}</div>
        </div>

        <!-- Panah -->
        <div style="text-align:center; padding-top:14px; color:var(--text-3);">→</div>

        <!-- Kolom kanan: Pilih SHS Resmi -->
        <div>
          {#if dipilih}
            <div style="padding:8px 12px; background:var(--surface-2); border:1px solid var(--accent); border-radius:6px; margin-bottom:8px; display:flex; justify-content:space-between; align-items:start;">
              <div>
                <div style="font-size:11px; font-family:monospace; color:var(--text-3);">[{dipilih.jenis}] {dipilih.kode}</div>
                <div style="font-size:13px; font-weight:600; color:var(--text);">{dipilih.uraian}</div>
                <div style="font-size:12px; color:{dipilih.harga !== r.harga ? '#856404' : 'var(--text-3)'};">
                  {formatRp(dipilih.harga)}
                  {#if dipilih.harga !== r.harga}
                    ({dipilih.harga > r.harga ? "+" : ""}{formatRp(dipilih.harga - r.harga)})
                  {/if}
                </div>
              </div>
              <button class="btn btn-ghost" style="font-size:11px;" onclick={() => { pemetaan = { ...pemetaan, [r.id]: 0 }; dipilihMap = { ...dipilihMap }; delete dipilihMap[r.id]; cari = { ...cari, [r.id]: "" }; }}>Ganti</button>
            </div>
          {:else}
            <input
              class="input"
              style="margin-bottom:8px; font-size:12px;"
              placeholder="Cari SHS Resmi..."
              bind:value={cari[r.id]}
              oninput={() => searchShs(r.id, r.jenis, cari[r.id] || "")}
            />
            <div style="border:1px solid var(--border); border-radius:6px; max-height:180px; overflow-y:auto;">
              {#if (cari[r.id] || "").length === 0}
                <div style="padding:12px; font-size:12px; color:var(--text-3); text-align:center;">Ketik untuk mencari...</div>
              {:else if (shsResults[r.id] || []).length === 0}
                <div style="padding:12px; font-size:12px; color:var(--text-3); text-align:center;">Tidak ditemukan</div>
              {:else}
                {#each (shsResults[r.id] || []) as resmi}
                  <button
                    style="width:100%; text-align:left; padding:10px 12px; background:transparent; border:none; border-bottom:1px solid var(--border); cursor:pointer; display:flex; flex-direction:column; gap:2px;"
                    onclick={() => { pemetaan = { ...pemetaan, [r.id]: resmi.id }; dipilihMap = { ...dipilihMap, [r.id]: resmi }; cari = { ...cari, [r.id]: "" }; shsResults = { ...shsResults, [r.id]: [] }; }}
                  >
                    <span style="font-size:11px; font-family:monospace; color:var(--text-3);">[{resmi.jenis}] {resmi.kode}</span>
                    <span style="font-size:12px; color:var(--text);">{resmi.uraian}</span>
                    <span style="font-size:11px; color:var(--text-3);">{formatRp(resmi.harga)} / {resmi.satuan}</span>
                  </button>
                {/each}
              {/if}
            </div>
          {/if}
        </div>
      </div>
    {/each}
  </div>

  <div style="display:flex; gap:10px; align-items:center;">
    <button
      onclick={terapkan}
      disabled={submitting || Object.values(pemetaan).filter(v => v > 0).length === 0}
      class="btn btn-primary"
    >
      {submitting ? "Menerapkan..." : "Terapkan Pemetaan (" + Object.values(pemetaan).filter(v => v > 0).length + ")"}
    </button>
    <a href="/dokumen/{dokumenId}" class="btn btn-ghost">Batal</a>
    <span style="font-size:12px; color:var(--text-3); margin-left:8px;">
      {Object.values(pemetaan).filter(v => v > 0).length} dari {shsRencanaList.length} dipetakan
    </span>
  </div>
{/if}