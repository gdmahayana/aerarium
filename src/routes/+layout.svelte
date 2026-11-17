<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import { onMount } from "svelte";

  let { children } = $props();

  let theme = $state<"dark"|"light">("dark");

  onMount(() => {
    const saved = localStorage.getItem("aerarium-theme") || "dark";
    theme = saved as "dark"|"light";
    document.documentElement.setAttribute("data-theme", saved);
  });

  function toggleTheme() {
    theme = theme === "dark" ? "light" : "dark";
    document.documentElement.setAttribute("data-theme", theme);
    localStorage.setItem("aerarium-theme", theme);
  }

  const navGroups = [
    {
      label: null,
      items: [
        { href: "/", label: "Dashboard" },
        { href: "/tahun-anggaran", label: "Tahun Anggaran" },
      ]
    },
    {
      label: "Persiapan",
      items: [
        { href: "/dokumen?filter=persiapan", label: "Kertas Kerja" },
        { href: "/dokumen?filter=pagu", label: "Pagu Indikatif" },
      ]
    },
    {
      label: "Siklus APBD",
      items: [
        { href: "/dokumen?filter=rka", label: "RKA-SKPD" },
        { href: "/dokumen?filter=dpa", label: "DPA-SKPD" },
        { href: "/dokumen?filter=perubahan", label: "Perubahan APBD" },
      ]
    },
    {
      label: "Data",
      items: [
        { href: "/ssh", label: "Database SHS" },
        { href: "/master-data", label: "Master Data" },
      ]
    },
  ];
</script>

<div style="display:flex; height:100vh; overflow:hidden;">
  <aside class="sidebar" style="width:220px; display:flex; flex-direction:column; flex-shrink:0;">
    <div style="padding:20px 16px 16px; border-bottom:1px solid var(--border);">
      <div style="font-size:16px; font-weight:700; color:var(--text); letter-spacing:-0.02em;">Aerarium</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:2px; font-weight:500;">Budget Workspace</div>
    </div>

    <nav style="flex:1; padding:12px 8px; overflow-y:auto;">
      {#each navGroups as group}
        {#if group.label}
          <div style="font-size:10px; font-weight:700; color:var(--text-3); letter-spacing:0.08em; text-transform:uppercase; padding:12px 10px 4px; margin-top:4px;">
            {group.label}
          </div>
        {/if}
        {#each group.items as item}
          <a
            href={item.href}
            class="nav-item {$page.url.pathname + $page.url.search === item.href || ($page.url.pathname === '/dokumen' && item.href === '/dokumen?filter=rka' && !$page.url.search) ? 'active' : ''}"
          >
            {item.label}
          </a>
        {/each}
      {/each}
    </nav>

    <div style="padding:12px 16px; border-top:1px solid var(--border); display:flex; align-items:center; justify-content:space-between;">
      <div style="font-size:11px; color:var(--text-3); font-weight:500;">v0.1.0-dev · AGPL-3.0</div>
      <button
        onclick={toggleTheme}
        style="background:none; border:1px solid var(--border); border-radius:6px; padding:4px 8px; cursor:pointer; font-size:12px; color:var(--text-3); font-family:inherit; display:flex; align-items:center; gap:4px;"
        title="Toggle tema"
      >{theme === 'dark' ? '☀' : '🌙'}</button>
    </div>
  </aside>

  <main style="flex:1; overflow:auto; padding:32px;">
    {@render children()}
  </main>
</div>
