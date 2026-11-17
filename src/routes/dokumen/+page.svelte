<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { page } from "$app/stores";
  import { tick } from "svelte";

  interface Dokumen {
    id: number;
    jenis: string;
    versi: number;
    nomor_se: string | null;
    tanggal_se: string | null;
    status: string;
    asal_id: number | null;
    catatan: string | null;
  }

  interface TahunAnggaran {
    id: number;
    tahun: number;
    aktif: boolean;
  }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let dokumenList = $state<Dokumen[]>([]);
  let loading = $state(true);
  let error = $state("");

  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;
      if (tahunAktif) {
        dokumenList = await invoke<Dokumen[]>("get_dokumen", { tahunAnggaranId: tahunAktif.id });
      }
    } catch(e) { error = String(e); }
    finally { loading = false; }
  });

  function byJenis(jenis: string) {
    return dokumenList.filter(d => d.jenis === jenis).sort((a, b) => b.versi - a.versi);
  }

  const statusStyle: Record<string, {bg: string, color: string, label: string}> = {
    draft:      { bg: "#1e293b", color: "#94a3b8", label: "Draft" },
    konsep:     { bg: "#0c2a4a", color: "#7dd3fc", label: "Konsep" },
    ditetapkan: { bg: "#052e16", color: "#4ade80", label: "Ditetapkan" },
    dikunci:    { bg: "#2d1f00", color: "#fbbf24", label: "Dikunci" },
    diarsipkan: { bg: "#1a1a2e", color: "#6b7280", label: "Diarsipkan" },
  };

  // Tahapan — 2 blok: Murni (KK + PI) dan Penganggaran
  const tahapan = [
    {
      group: "Persiapan",
      items: [
        {
          jenis: "Kertas Kerja",
          icon: "✦",
          warna: "#6366f1",
          warnaGlow: "#6366f130",
          tagline: "Survei kebutuhan & estimasi harga",
          ada_se: false,
          tunggal: true,
          next: null,
        },
        {
          jenis: "Pagu Indikatif",
          icon: "◈",
          warna: "#0ea5e9",
          warnaGlow: "#0ea5e930",
          tagline: "Alokasi Bappeda · SHS Resmi",
          ada_se: false,
          tunggal: true,
          next: "RKA-SKPD",
        },
      ]
    },
    {
      group: "Penganggaran",
      items: [
        {
          jenis: "RKA-SKPD",
          icon: "▸",
          warna: "#10b981",
          warnaGlow: "#10b98130",
          tagline: "Rencana Kerja & Anggaran",
          ada_se: true,
          tunggal: false,
          next: "DPA-SKPD",
        },
        {
          jenis: "DPA-SKPD",
          icon: "▸",
          warna: "#f59e0b",
          warnaGlow: "#f59e0b30",
          tagline: "Dokumen Pelaksanaan Anggaran",
          ada_se: true,
          tunggal: true,
          next: null,
        },
      ]
    },
    {
      group: "Perubahan",
      items: [
        {
          jenis: "RKA-SKPD Perubahan",
          icon: "▸",
          warna: "#10b981",
          warnaGlow: "#10b98130",
          tagline: "Rencana Kerja & Anggaran Perubahan",
          ada_se: true,
          tunggal: false,
          next: "DPPA-SKPD",
        },
        {
          jenis: "DPPA-SKPD",
          icon: "▸",
          warna: "#f59e0b",
          warnaGlow: "#f59e0b30",
          tagline: "Dokumen Pelaksanaan Perubahan Anggaran",
          ada_se: true,
          tunggal: true,
          next: "Perubahan DPA-SKPD",
        },
        {
          jenis: "Perubahan DPA-SKPD",
          icon: "⟳",
          warna: "#ef4444",
          warnaGlow: "#ef444430",
          tagline: "Pergeseran Anggaran · DPA",
          ada_se: true,
          tunggal: false,
          next: null,
        },
        {
          jenis: "Perubahan DPPA-SKPD",
          icon: "⟳",
          warna: "#ef4444",
          warnaGlow: "#ef444430",
          tagline: "Pergeseran Anggaran · DPPA",
          ada_se: true,
          tunggal: false,
          next: null,
        },
      ]
    },
  ];

  // ── Git-graph: bangun rantai dokumen dari asal_id ──────────────────────────
  const jenisWarna: Record<string, string> = {
    'Kertas Kerja':        '#6366f1',
    'Pagu Indikatif':      '#0ea5e9',
    'RKA-SKPD':            '#10b981',
    'DPA-SKPD':            '#f59e0b',
    'RKA-SKPD Perubahan':  '#10b981',
    'DPPA-SKPD':           '#f59e0b',
    'Perubahan DPA-SKPD':  '#ef4444',
    'Perubahan DPPA-SKPD': '#ef4444',
  };
  const jenisIcon: Record<string, string> = {
    'Kertas Kerja':        '✦',
    'Pagu Indikatif':      '◈',
    'RKA-SKPD':            '▸',
    'DPA-SKPD':            '▸',
    'RKA-SKPD Perubahan':  '▸',
    'DPPA-SKPD':           '▸',
    'Perubahan DPA-SKPD':  '⟳',
    'Perubahan DPPA-SKPD': '⟳',
  };

  // Kembalikan array of chains; tiap chain = array Dokumen terurut dari root
  function buildChains(list: Dokumen[]): Dokumen[][] {
    const byId = new Map(list.map(d => [d.id, d]));
    const hasParent = new Set(list.filter(d => d.asal_id !== null).map(d => d.asal_id!));
    // Root = dokumen yang tidak jadi anak siapapun
    const roots = list.filter(d => !hasParent.has(d.id) || d.asal_id === null);

    function buildChain(root: Dokumen): Dokumen[] {
      const chain: Dokumen[] = [root];
      let current = root;
      while (true) {
        const child = list.find(d => d.asal_id === current.id);
        if (!child) break;
        chain.push(child);
        current = child;
      }
      return chain;
    }

    // Hanya tampilkan chain yang panjangnya > 1 atau semua kalau mau
    return roots
      .map(r => buildChain(r))
      // .filter(c => c.length > 1) // semua ditampilkan, orphan diberi gaya redup
      .sort((a, b) => b.length - a.length);
  }

  let chains = $derived(buildChains(dokumenList));

  // Filter dari sidebar
  const filterMap: Record<string, string> = {
    persiapan: "Persiapan",
    pagu: "Persiapan",
    rka: "Penganggaran",
    dpa: "Penganggaran",
    perubahan: "Perubahan",
  };
  let activeGroup = $derived(filterMap[$page.url.searchParams.get("filter") || ""] || null);

  let actionLoading = $state<number | null>(null);

  async function quickJadikanDpa(rkaId: number) {
    if (!confirm('Jadikan RKA-SKPD ini sebagai DPA-SKPD?')) return;
    actionLoading = rkaId;
    try {
      const id = await invoke<number>('jadikan_dpa', { rkaId });
      goto('/dokumen/' + id);
    } catch(e) { error = String(e); actionLoading = null; }
  }

  async function quickJadikanDppa(rkapId: number) {
    if (!confirm('Jadikan RKA-SKPD Perubahan ini sebagai DPPA-SKPD?')) return;
    actionLoading = rkapId;
    try {
      const id = await invoke<number>('jadikan_dppa', { rkapId });
      goto('/dokumen/' + id);
    } catch(e) { error = String(e); actionLoading = null; }
  }

  async function quickPergeseran(dokumenId: number) {
    if (!confirm('Buat pergeseran anggaran? Dokumen ini akan dikunci dan rinciannya disalin ke versi baru.')) return;
    actionLoading = dokumenId;
    try {
      const id = await invoke<number>('buat_pergeseran', { dokumenId });
      goto('/dokumen/' + id);
    } catch(e) { error = String(e); actionLoading = null; }
  }

  $effect(() => {
    if (activeGroup) {
      tick().then(() => {
        const el = document.getElementById('group-' + activeGroup);
        if (el) el.scrollIntoView({ behavior: 'smooth', block: 'start' });
      });
    }
  });
</script>

<div class="page-header" style="display:flex; align-items:start; justify-content:space-between;">
  <div>
    <h2>Dokumen Anggaran</h2>
    {#if tahunAktif}
      <p>TA <strong style="color:var(--text);">{tahunAktif.tahun}</strong></p>
    {/if}
  </div>
  <a href="/dokumen/buat" class="btn btn-primary" style="font-size:12px;">+ Buat Dokumen</a>
</div>

{#if error}<div class="alert-error">{error}</div>{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !tahunAktif}
  <div class="card" style="padding:48px; text-align:center; max-width:380px; margin:60px auto;">
    <p style="font-size:13px; color:var(--text-2); margin-bottom:16px;">Belum ada tahun anggaran aktif.</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Atur Tahun Anggaran</a>
  </div>
{:else}
  <div style="display:flex; flex-direction:column; gap:28px;">
    {#each tahapan as group}
      <div>
        <!-- Group label -->
        <div id="group-{group.group}" style="display:flex; align-items:center; gap:10px; margin-bottom:14px;">
          <span class="label" style="color:{activeGroup === group.group ? 'var(--accent)' : ''};">{group.group}</span>
          <div style="flex:1; height:1px; background:{activeGroup === group.group ? 'var(--accent)' : 'var(--border)'};opacity:{activeGroup === group.group ? 0.5 : 1};"></div>
        </div>

        <div style="display:flex; flex-direction:column; gap:8px;">
          {#each group.items as tahap}
            {@const doks = byJenis(tahap.jenis)}
            {@const ada = doks.length > 0}

            <div style="
              border:1px solid {ada ? tahap.warna + '44' : 'var(--border)'};
              border-radius:10px;
              background:{ada ? tahap.warnaGlow : 'transparent'};
              overflow:hidden;
              transition: border-color 0.2s;
            ">
              <!-- Header tahap -->
              <div style="
                display:flex; align-items:center; justify-content:space-between;
                padding:12px 16px;
                border-bottom:{ada ? '1px solid ' + tahap.warna + '22' : 'none'};
              ">
                <div style="display:flex; align-items:center; gap:10px;">
                  <span style="
                    font-size:14px; font-weight:800; color:{ada ? tahap.warna : 'var(--text-3)'};
                    width:20px; text-align:center;
                  ">{tahap.icon}</span>
                  <div>
                    <span style="font-size:13px; font-weight:700; color:{ada ? 'var(--text)' : 'var(--text-3)'};">
                      {tahap.jenis}
                    </span>
                    <span style="font-size:11px; color:var(--text-3); margin-left:8px;">
                      {tahap.tagline}
                    </span>
                  </div>
                </div>
                {#if !ada}
                  <a
                    href="/dokumen/buat"
                    style="font-size:11px; color:var(--text-3); padding:4px 10px; border:1px dashed var(--border); border-radius:5px; text-decoration:none;"
                  >+ Buat</a>
                {/if}
              </div>

              <!-- Daftar dokumen -->
              {#if ada}
                <div style="display:flex; flex-direction:column; gap:0;">
                  {#each doks as dok, i}
                    <div
                      style="
                        display:flex; align-items:center; justify-content:space-between;
                        padding:11px 16px;
                        border-bottom:{i < doks.length - 1 ? '1px solid ' + tahap.warna + '15' : 'none'};
                        cursor:pointer;
                      "
                      onclick={() => goto('/dokumen/' + dok.id)}
                      onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = tahap.warna + '10'}
                      onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = 'transparent'}
                    >
                      <!-- Kiri: info -->
                      <div style="display:flex; align-items:center; gap:14px;">
                        <span style="
                          font-size:10px; font-weight:700; font-family:monospace;
                          color:{tahap.warna}; background:{tahap.warna}18;
                          padding:2px 6px; border-radius:4px; min-width:28px; text-align:center;
                        ">v{dok.versi}</span>

                        <div>
                          {#if tahap.ada_se}
                            {#if dok.nomor_se}
                              <span style="font-size:12px; font-weight:600; color:var(--text);">
                                SE: {dok.nomor_se}
                              </span>
                            {:else}
                              <span style="font-size:12px; color:var(--text-3); font-style:italic;">
                                Tanpa nomor SE
                              </span>
                            {/if}
                            {#if dok.tanggal_se}
                              <span style="font-size:11px; color:var(--text-3); margin-left:8px;">
                                {dok.tanggal_se}
                              </span>
                            {/if}
                          {:else}
                            <span style="font-size:12px; color:var(--text-2);">
                              {dok.catatan || tahap.jenis}
                            </span>
                          {/if}
                        </div>
                      </div>

                      <!-- Kanan: status + tombol -->
                      <div style="display:flex; align-items:center; gap:8px;">
                        <span style="
                          font-size:10px; font-weight:700; padding:3px 8px; border-radius:4px;
                          background:{(statusStyle[dok.status] || statusStyle.draft).bg}; color:{(statusStyle[dok.status] || statusStyle.draft).color};
                          letter-spacing:0.05em; text-transform:uppercase;
                        ">{(statusStyle[dok.status] || statusStyle.draft).label}</span>
                        {#if tahap.jenis === 'RKA-SKPD' && (dok.status === 'draft' || dok.status === 'konsep')}
                          <button
                            onclick={(e) => { e.stopPropagation(); quickJadikanDpa(dok.id); }}
                            disabled={actionLoading === dok.id}
                            style="font-size:11px; padding:3px 9px; border-radius:4px; cursor:pointer; border:1px solid #f59e0b44; background:#f59e0b10; color:#f59e0b; font-family:inherit;"
                          >{actionLoading === dok.id ? '...' : '→ DPA'}</button>
                        {/if}
                        {#if tahap.jenis === 'RKA-SKPD Perubahan' && (dok.status === 'draft' || dok.status === 'konsep')}
                          <button
                            onclick={(e) => { e.stopPropagation(); quickJadikanDppa(dok.id); }}
                            disabled={actionLoading === dok.id}
                            style="font-size:11px; padding:3px 9px; border-radius:4px; cursor:pointer; border:1px solid #f59e0b44; background:#f59e0b10; color:#f59e0b; font-family:inherit;"
                          >{actionLoading === dok.id ? '...' : '→ DPPA'}</button>
                        {/if}
                        {#if (tahap.jenis === 'DPA-SKPD' || tahap.jenis === 'DPPA-SKPD') && (dok.status === 'ditetapkan' || dok.status === 'dikunci')}
                          <button
                            onclick={(e) => { e.stopPropagation(); quickPergeseran(dok.id); }}
                            disabled={actionLoading === dok.id}
                            style="font-size:11px; padding:3px 9px; border-radius:4px; cursor:pointer; border:1px solid #ef444444; background:#ef444410; color:#ef4444; font-family:inherit;"
                          >{actionLoading === dok.id ? '...' : '⟳ Geser'}</button>
                        {/if}
                        <span style="font-size:12px; color:var(--text-3);">→</span>
                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>

            <!-- Panah koneksi antar tahap dalam group -->
            {#if tahap.next && byJenis(tahap.jenis).length > 0}
              <div style="display:flex; align-items:center; gap:8px; padding:4px 0 4px 28px;">
                <div style="width:1px; height:16px; background:var(--border);"></div>
                <span style="font-size:10px; color:var(--text-3);">Jadikan {tahap.next} →</span>
              </div>
            {/if}
          {/each}
        </div>
      </div>
    {/each}
    <!-- ── Git-graph rantai dokumen ──────────────────────────────────────── -->
    {#if chains.length > 0}
      <div style="margin-top:12px;">
        <div style="display:flex; align-items:center; gap:10px; margin-bottom:14px;">
          <span class="label">Alur Dokumen</span>
          <div style="flex:1; height:1px; background:var(--border);"></div>
        </div>

        <div style="display:flex; flex-direction:column; gap:12px;">
          {#each chains as chain}
            <div style="
              display:flex; align-items:center; gap:0;
              padding:14px 20px; background:var(--surface-2);
              border:1px solid {chain.length === 1 ? 'var(--border)' : 'var(--border)'}; border-radius:10px;
              overflow-x:auto; opacity:{chain.length === 1 ? 0.5 : 1};
            ">
              {#each chain as dok, i}
                {@const warna = jenisWarna[dok.jenis] || '#94a3b8'}
                {@const icon = jenisIcon[dok.jenis] || '▸'}
                {@const st = statusStyle[dok.status] || statusStyle.draft}

                <!-- Node -->
                <button
                  onclick={() => goto('/dokumen/' + dok.id)}
                  style="
                    display:flex; flex-direction:column; align-items:center; gap:6px;
                    padding:10px 14px; border-radius:8px; cursor:pointer;
                    background:{warna}10; border:1.5px solid {warna}44;
                    min-width:120px; flex-shrink:0;
                    font-family:inherit; transition:background 0.15s;
                  "
                  onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = warna + '22'}
                  onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = warna + '10'}
                >
                  <div style="display:flex; align-items:center; gap:5px;">
                    <span style="color:{warna}; font-weight:800; font-size:12px;">{icon}</span>
                    <span style="
                      font-size:9px; font-weight:700; font-family:monospace;
                      color:{warna}; background:{warna}20; padding:1px 5px; border-radius:3px;
                    ">v{dok.versi}</span>
                  </div>
                  <div style="font-size:11px; font-weight:700; color:var(--text); text-align:center; line-height:1.3;">
                    {dok.jenis}
                  </div>
                  {#if dok.nomor_se}
                    <div style="font-size:10px; color:var(--text-3); text-align:center;">SE: {dok.nomor_se}</div>
                  {/if}
                  <span style="
                    font-size:9px; font-weight:700; padding:2px 6px; border-radius:3px;
                    background:{st.bg}; color:{st.color}; letter-spacing:0.05em;
                  ">{st.label}</span>
                </button>

                <!-- Panah antar node -->
                {#if i < chain.length - 1}
                  <div style="
                    display:flex; align-items:center; padding:0 4px;
                    color:var(--text-3); font-size:14px; flex-shrink:0;
                  ">→</div>
                {/if}
              {/each}
            </div>
          {/each}
        </div>
      </div>
    {/if}
  </div>
{/if}