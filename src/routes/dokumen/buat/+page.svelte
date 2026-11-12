<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { goto } from "$app/navigation";

  interface TahunAnggaran { id: number; tahun: number; aktif: boolean; }
  interface DokumenAsal { id: number; jenis: string; versi: number; nomor_se: string | null; status: string; }

  let tahunAktif = $state<TahunAnggaran | null>(null);
  let loading = $state(true);
  let submitting = $state(false);
  let error = $state("");

  let jenis = $state("Kertas Kerja");
  let nomor_se = $state("");
  let tanggal_se = $state("");
  let catatan = $state("");
  let asal_id = $state<number | null>(null);
  let asal_mode = $state<"aerarium"|"sipd">("aerarium");
  let asal_keterangan = $state("");
  let dokumenAsalList = $state<DokumenAsal[]>([]);

  const butuhAsal = $derived(jenis === "DPA-SKPD" || jenis === "DPPA-SKPD");
  const jenisDibutuhkan = $derived(jenis === "DPA-SKPD" ? "RKA-SKPD" : "RKA-SKPD Perubahan");
  const adaSE = $derived(!["Kertas Kerja", "Pagu Indikatif"].includes(jenis));

  const jenisList = [
    {
      value: "Kertas Kerja",
      icon: "✦",
      warna: "#6366f1",
      desc: "Survei kebutuhan & estimasi harga awal",
    },
    {
      value: "Pagu Indikatif",
      icon: "◈",
      warna: "#0ea5e9",
      desc: "Alokasi dari Bappeda, gunakan SHS Resmi",
    },
    {
      value: "RKA-SKPD",
      icon: "▸",
      warna: "#10b981",
      desc: "Rencana Kerja & Anggaran berdasarkan SE",
    },
    {
      value: "DPA-SKPD",
      icon: "▸",
      warna: "#f59e0b",
      desc: "Dokumen Pelaksanaan Anggaran dari RKA",
    },
    {
      value: "RKA-SKPD Perubahan",
      icon: "▸",
      warna: "#10b981",
      desc: "RKA Perubahan berdasarkan SE perubahan",
    },
    {
      value: "DPPA-SKPD",
      icon: "▸",
      warna: "#f59e0b",
      desc: "Dokumen Pelaksanaan Perubahan Anggaran",
    },
  ];

  onMount(async () => {
    try {
      const list = await invoke<TahunAnggaran[]>("get_tahun_anggaran");
      tahunAktif = list.find(t => t.aktif) || null;
      if (tahunAktif) {
        const semua = await invoke<DokumenAsal[]>("get_dokumen", { tahunAnggaranId: tahunAktif.id });
        dokumenAsalList = semua.filter(d => d.status !== "diarsipkan");
      }
    } catch(e) { error = String(e); }
    finally { loading = false; }
  });

  async function simpan() {
    if (!tahunAktif) return;
    submitting = true;
    try {
      const id = await invoke<number>("buat_dokumen", {
        input: {
          tahun_anggaran_id: tahunAktif.id,
          jenis,
          nomor_se: adaSE && nomor_se ? nomor_se : null,
          tanggal_se: adaSE && tanggal_se ? tanggal_se : null,
          asal_id: butuhAsal && asal_mode === "aerarium" ? asal_id : null,
          asal_keterangan: butuhAsal && asal_mode === "sipd" ? asal_keterangan || null : null,
          catatan: catatan || null,
        }
      });
      goto("/dokumen/" + id);
    } catch(e) { error = String(e); }
    finally { submitting = false; }
  }

  $effect(() => {
    // Reset SE saat ganti jenis
    if (!adaSE) { nomor_se = ""; tanggal_se = ""; }
  });
</script>

<div class="page-header" style="display:flex; align-items:start; justify-content:space-between;">
  <div>
    <h2>Buat Dokumen</h2>
    <p>Tambah dokumen anggaran baru untuk TA {tahunAktif?.tahun || "..."}</p>
  </div>
  <a href="/dokumen" class="btn btn-ghost" style="font-size:12px;">← Kembali</a>
</div>

{#if error}<div class="alert-error">{error}<button onclick={() => error=""} style="margin-left:8px; background:none; border:none; color:inherit; cursor:pointer; text-decoration:underline;">Tutup</button></div>{/if}

{#if loading}
  <p style="font-size:13px; color:var(--text-3);">Memuat...</p>
{:else if !tahunAktif}
  <div class="card" style="padding:40px; text-align:center;">
    <p style="font-size:13px; color:var(--text-3); margin-bottom:12px;">Belum ada tahun anggaran aktif.</p>
    <a href="/tahun-anggaran" class="btn btn-primary">Atur Tahun Anggaran</a>
  </div>
{:else}
  <div style="max-width:580px; display:flex; flex-direction:column; gap:24px;">

    <!-- Pilih jenis -->
    <div>
      <div class="label" style="margin-bottom:12px;">Jenis Dokumen</div>
      <div style="display:grid; grid-template-columns:1fr 1fr; gap:8px;">
        {#each jenisList as item}
          {@const dipilih = jenis === item.value}
          <button
            onclick={() => jenis = item.value}
            style="
              text-align:left; padding:12px 14px; border-radius:8px; cursor:pointer;
              background:{dipilih ? item.warna + '18' : 'var(--surface-2)'};
              border:1.5px solid {dipilih ? item.warna : 'var(--border)'};
              transition: all 0.15s;
              font-family: inherit;
            "
          >
            <div style="display:flex; align-items:center; gap:8px; margin-bottom:4px;">
              <span style="color:{item.warna}; font-weight:800; font-size:13px;">{item.icon}</span>
              <span style="font-size:12px; font-weight:700; color:{dipilih ? 'var(--text)' : 'var(--text-2)'};">
                {item.value}
              </span>
            </div>
            <div style="font-size:11px; color:var(--text-3); padding-left:21px;">{item.desc}</div>
          </button>
        {/each}
      </div>
    </div>

    <!-- SE — hanya untuk jenis yang butuh -->
    {#if adaSE}
      <div style="display:flex; gap:12px;">
        <div style="flex:2;">
          <div class="label" style="margin-bottom:6px;">Nomor Surat Edaran</div>
          <input class="input" placeholder="900/123/BPKPD/2025" bind:value={nomor_se} />
        </div>
        <div style="flex:1;">
          <div class="label" style="margin-bottom:6px;">Tanggal SE</div>
          <input class="input" type="date" bind:value={tanggal_se} />
        </div>
      </div>
    {:else}
      <div style="
        padding:12px 14px; border-radius:8px;
        background:var(--surface-2); border:1px solid var(--border);
        font-size:12px; color:var(--text-3);
      ">
        💡 {jenis} tidak memerlukan Surat Edaran — langsung isi sub kegiatan setelah dibuat.
      </div>
    {/if}

    <!-- Asal dokumen -->
    {#if butuhAsal}
      <div>
        <div class="label" style="margin-bottom:8px;">Asal Dokumen <span style="color:{jenisDibutuhkan === 'RKA-SKPD' ? '#10b981' : '#10b981'};">({jenisDibutuhkan})</span></div>
        <div style="display:flex; gap:6px; margin-bottom:10px;">
          <button onclick={() => asal_mode = "aerarium"} class="btn {asal_mode === 'aerarium' ? 'btn-primary' : 'btn-ghost'}" style="font-size:12px;">Di Aerarium</button>
          <button onclick={() => asal_mode = "sipd"} class="btn {asal_mode === 'sipd' ? 'btn-primary' : 'btn-ghost'}" style="font-size:12px;">Di SIPD</button>
        </div>
        {#if asal_mode === "aerarium"}
          <select bind:value={asal_id} class="input">
            <option value={null}>-- Pilih {jenisDibutuhkan} --</option>
            {#each dokumenAsalList.filter(d => d.jenis === jenisDibutuhkan) as dok}
              <option value={dok.id}>{dok.jenis} v{dok.versi}{dok.nomor_se ? ' — ' + dok.nomor_se : ''} ({dok.status})</option>
            {/each}
          </select>
        {:else}
          <input class="input" placeholder="Nomor DPA di SIPD..." bind:value={asal_keterangan} />
        {/if}
      </div>
    {/if}

    <!-- Catatan -->
    <div>
      <div class="label" style="margin-bottom:6px;">Catatan <span style="color:var(--text-3); font-weight:400; text-transform:none; letter-spacing:0;">(opsional)</span></div>
      <textarea class="input" placeholder="Keterangan tambahan..." bind:value={catatan} style="height:72px; resize:vertical;"></textarea>
    </div>

    <!-- Actions -->
    <div style="display:flex; gap:10px;">
      <button onclick={simpan} disabled={submitting} class="btn btn-primary">
        {submitting ? "Menyimpan..." : "Buat Dokumen"}
      </button>
      <a href="/dokumen" class="btn btn-ghost">Batal</a>
    </div>
  </div>
{/if}