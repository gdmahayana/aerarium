<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";
  import { jenisConfig, statusConfig, TRANSISI_STATUS, TAHAPAN, type StatusDokumen } from "$lib/dokumen-config";
  import { toast } from "$lib/stores/toast";

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


  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;
      if (tahunAktif) {
        dokumenList = await invoke<Dokumen[]>("get_dokumen", { tahunAnggaranId: tahunAktif.id });
      }
    } catch(e) { toast.error(String(e)); }
    finally { loading = false; }
  });

  function byJenis(jenis: string) {
    return dokumenList.filter(d => d.jenis === jenis).sort((a, b) => b.versi - a.versi);
  }



  // ── Git-graph: bangun rantai dokumen dari asal_id ──────────────────────────



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

  // Tahapan dari shared config — diperkaya warnaGlow untuk template
  const tahapan = TAHAPAN.map(g => ({
    ...g,
    items: g.items.map(item => ({
      ...item,
      ...jenisConfig(item.jenis),
      ada_se: jenisConfig(item.jenis).adaSE,
    }))
  }));

  // Tab aktif
  let activeTab = $state<'dokumen' | 'alur'>('dokumen');



  let actionLoading = $state<number | null>(null);
  let statusPopover = $state<number | null>(null);

  function handleClickOutside(e: MouseEvent) {
    if (statusPopover !== null) statusPopover = null;
  }

  async function quickJadikanDpa(rkaId: number) {
    if (!confirm('Jadikan RKA-SKPD ini sebagai DPA-SKPD?')) return;
    actionLoading = rkaId;
    try {
      const id = await invoke<number>('jadikan_dpa', { rkaId });
      toast.success('DPA-SKPD berhasil dibuat.');
      goto('/dokumen/' + id);
    } catch(e) { toast.error(String(e)); actionLoading = null; }
  }

  async function quickJadikanDppa(rkapId: number) {
    if (!confirm('Jadikan RKA-SKPD Perubahan ini sebagai DPPA-SKPD?')) return;
    actionLoading = rkapId;
    try {
      const id = await invoke<number>('jadikan_dppa', { rkapId });
      toast.success('DPPA-SKPD berhasil dibuat.');
      goto('/dokumen/' + id);
    } catch(e) { toast.error(String(e)); actionLoading = null; }
  }




  async function ubahStatus(dokId: number, statusBaru: string) {
    try {
      actionLoading = dokId;
      await invoke('ubah_status_dokumen', { id: dokId, statusBaru });
      dokumenList = await invoke('get_dokumen', { tahunAnggaranId: tahunAktif!.id });
      toast.success('Status diubah ke ' + statusConfig(statusBaru).label + '.');
    } catch(e) { toast.error(String(e)); }
    finally { actionLoading = null; }
  }

  async function quickPergeseran(dokumenId: number) {
    if (!confirm('Buat pergeseran anggaran? Dokumen ini akan dikunci dan rinciannya disalin ke versi baru.')) return;
    actionLoading = dokumenId;
    try {
      const id = await invoke<number>('buat_pergeseran', { dokumenId });
      toast.success('Pergeseran anggaran berhasil dibuat.');
      goto('/dokumen/' + id);
    } catch(e) { toast.error(String(e)); actionLoading = null; }
  }


</script>

<div onclick={handleClickOutside}>
<div class="page-header" style="display:flex; align-items:start; justify-content:space-between;">
  <div>
    <h2>Dokumen Anggaran</h2>
    {#if tahunAktif}
      <p>TA <strong style="color:var(--text);">{tahunAktif.tahun}</strong></p>
    {/if}
  </div>
  <a href="/dokumen/buat" class="btn btn-primary" style="font-size:12px;">+ Buat Dokumen</a>
</div>

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !tahunAktif}
  <div class="card" style="padding:48px; text-align:center; max-width:380px; margin:60px auto;">
    <p style="font-size:13px; color:var(--text-2); margin-bottom:16px;">Belum ada tahun anggaran aktif.</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Atur Tahun Anggaran</a>
  </div>
{:else}
  <!-- Tab switcher -->
  <div style="display:flex; gap:0; border:1px solid var(--border); border-radius:8px; overflow:hidden; margin-bottom:24px; width:fit-content;">
    <button
      style="padding:8px 20px; font-size:12px; font-weight:600; border:none; cursor:pointer; background:{activeTab === 'dokumen' ? 'var(--accent)' : 'transparent'}; color:{activeTab === 'dokumen' ? 'white' : 'var(--text-3)'}; font-family:inherit;"
      onclick={() => activeTab = 'dokumen'}
    >📋 Dokumen</button>
    <button
      style="padding:8px 20px; font-size:12px; font-weight:600; border:none; cursor:pointer; background:{activeTab === 'alur' ? 'var(--accent)' : 'transparent'}; color:{activeTab === 'alur' ? 'white' : 'var(--text-3)'}; font-family:inherit; border-left:1px solid var(--border);"
      onclick={() => activeTab = 'alur'}
    >◈ Alur Dokumen {chains.filter(c => c.length > 1).length > 0 ? '(' + chains.filter(c => c.length > 1).length + ')' : ''}</button>
  </div>

  {#if activeTab === 'dokumen'}
  <div style="display:flex; flex-direction:column; gap:28px;">
    {#each tahapan as group}
      <div>
        <!-- Group label -->
        <div style="display:flex; align-items:center; gap:10px; margin-bottom:14px;">
          <span class="label">{group.group}</span>
          <div style="flex:1; height:1px; background:var(--border);"></div>
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
                    href="/dokumen/buat?jenis={encodeURIComponent(tahap.jenis)}"
                    style="font-size:11px; color:var(--text-3); padding:4px 10px; border:1px dashed var(--border); border-radius:5px; text-decoration:none;"
                  >+ Buat</a>
                {/if}
              </div>

              <!-- Daftar dokumen -->
              {#if ada}
                <div style="display:flex; flex-direction:column; gap:0;">
                  {#each doks as dok, i}
                    {@const sc = statusConfig(dok.status)}
                    {@const transisi = TRANSISI_STATUS[dok.status as StatusDokumen] || []}
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
                        <div style="position:relative;">
                          <button
                            onclick={(e) => { e.stopPropagation(); statusPopover = statusPopover === dok.id ? null : dok.id; }}
                            disabled={actionLoading === dok.id}
                            style="font-size:10px; font-weight:700; padding:3px 8px; border-radius:4px; cursor:{transisi.length > 0 ? 'pointer' : 'default'}; border:1px solid {sc.color}44; background:{sc.bg}; color:{sc.color}; font-family:inherit; letter-spacing:0.05em; text-transform:uppercase; display:flex; align-items:center; gap:4px;"
                          >
                            {sc.label}{transisi.length > 0 ? ' ▾' : ''}
                          </button>
                          {#if statusPopover === dok.id && transisi.length > 0}
                            <div
                              style="position:absolute; right:0; top:calc(100% + 4px); background:var(--surface); border:1px solid var(--border); border-radius:6px; overflow:hidden; z-index:100; min-width:130px; box-shadow:0 4px 12px #0004;"
                              onclick={(e) => e.stopPropagation()}
                            >
                              {#each transisi as s}
                                {@const sc2 = statusConfig(s)}
                                <button
                                  onclick={(e) => { e.stopPropagation(); statusPopover = null; ubahStatus(dok.id, s); }}
                                  style="width:100%; text-align:left; padding:8px 12px; font-size:11px; font-weight:600; border:none; cursor:pointer; background:transparent; color:{sc2.color}; font-family:inherit; display:flex; align-items:center; gap:6px;"
                                  onmouseenter={(e) => (e.currentTarget as HTMLElement).style.background = 'var(--surface-2)'}
                                  onmouseleave={(e) => (e.currentTarget as HTMLElement).style.background = 'transparent'}
                                >
                                  <span style="font-size:9px; background:{sc2.bg}; color:{sc2.color}; padding:1px 5px; border-radius:3px; letter-spacing:0.05em; text-transform:uppercase;">{sc2.label}</span>
                                </button>
                              {/each}
                            </div>
                          {/if}
                        </div>
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

                      </div>
                    </div>
                  {/each}
                </div>
              {/if}
            </div>

            <!-- Panah koneksi antar tahap dalam group -->
            {#if tahap.next && byJenis(tahap.jenis).length > 0 && byJenis(tahap.next).length === 0}
              <div style="display:flex; align-items:center; gap:8px; padding:6px 0 2px 16px;">
                <div style="width:1px; height:20px; background:var(--border); margin-left:4px;"></div>
                <a
                  href="/dokumen/buat?jenis={encodeURIComponent(tahap.next)}"
                  style="font-size:11px; color:var(--text-3); padding:3px 10px; border:1px dashed var(--border); border-radius:5px; text-decoration:none; display:flex; align-items:center; gap:4px;"
                >+ Buat {tahap.next}</a>
              </div>
            {/if}
          {/each}
        </div>
      </div>
    {/each}
  </div>
  {/if}

  {#if activeTab === 'alur'}
  <div>
    {#if chains.length === 0}
      <div class="card" style="padding:48px; text-align:center;">
        <div style="font-size:28px; margin-bottom:12px;">◈</div>
        <p style="font-size:13px; color:var(--text-3);">Belum ada rantai dokumen. Buat dokumen dan jadikan DPA/DPPA untuk melihat alurnya.</p>
      </div>
    {:else}
      <div style="display:flex; align-items:center; gap:10px; margin-bottom:14px;">
        <span class="label">Alur Dokumen</span>
        <div style="flex:1; height:1px; background:var(--border);"></div>
        <span style="font-size:11px; color:var(--text-3);">{chains.filter(c => c.length > 1).length} rantai · {chains.filter(c => c.length === 1).length} tunggal</span>
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
                {@const warna = jenisConfig(dok.jenis).warna}
                {@const icon = jenisConfig(dok.jenis).icon}
                {@const st = statusConfig(dok.status)}

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
    {/if}
  </div>
  {/if}
{/if}
</div>