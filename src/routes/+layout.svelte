<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";

  let { children } = $props();

  const navItems = [
    { href: "/", label: "Dashboard" },
    { href: "/tahun-anggaran", label: "Tahun Anggaran" },
    { href: "/dokumen", label: "Dokumen" },
    { href: "/ssh", label: "Database SHS" },
  ];
</script>

<div style="display:flex; height:100vh; overflow:hidden;">
  <aside class="sidebar" style="width:220px; display:flex; flex-direction:column; flex-shrink:0;">
    <div style="padding:20px 16px 16px; border-bottom:1px solid var(--border);">
      <div style="font-size:16px; font-weight:700; color:var(--text); letter-spacing:-0.02em;">Aerarium</div>
      <div style="font-size:11px; color:var(--text-3); margin-top:2px; font-weight:500;">Budget Workspace</div>
    </div>

    <nav style="flex:1; padding:12px 8px;">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-item {$page.url.pathname === item.href ? 'active' : ''}"
        >
          {item.label}
        </a>
      {/each}
    </nav>

    <div style="padding:12px 16px; border-top:1px solid var(--border);">
      <div style="font-size:11px; color:var(--text-3); font-weight:500;">v0.1.0-dev · AGPL-3.0</div>
    </div>
  </aside>

  <main style="flex:1; overflow:auto; padding:32px;">
    {@render children()}
  </main>
</div>
