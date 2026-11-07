<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";

  let { children } = $props();

  const navItems = [
    { href: "/", label: "Dashboard", icon: "⊞" },
    { href: "/tahun-anggaran", label: "Tahun Anggaran", icon: "📅" },
    { href: "/dokumen", label: "Dokumen", icon: "📄" },
    { href: "/ssh", label: "Database SSH", icon: "💰" },
  ];
</script>

<div class="flex h-screen bg-gray-50 text-gray-900">
  <!-- Sidebar -->
  <aside class="w-56 bg-white border-r border-gray-200 flex flex-col">
    <!-- Logo -->
    <div class="px-4 py-5 border-b border-gray-200">
      <h1 class="text-lg font-bold tracking-tight">Aerarium</h1>
      <p class="text-xs text-gray-500 mt-0.5">Budget Workspace</p>
    </div>

    <!-- Nav -->
    <nav class="flex-1 px-2 py-3 space-y-0.5">
      {#each navItems as item}
        <a
          href={item.href}
          class="flex items-center gap-2.5 px-3 py-2 rounded text-sm
            {$page.url.pathname === item.href
              ? 'bg-gray-900 text-white'
              : 'text-gray-600 hover:bg-gray-100'}"
        >
          <span>{item.icon}</span>
          <span>{item.label}</span>
        </a>
      {/each}
    </nav>

    <!-- Footer -->
    <div class="px-4 py-3 border-t border-gray-200">
      <p class="text-xs text-gray-400">v0.1.0-dev</p>
    </div>
  </aside>

  <!-- Konten -->
  <main class="flex-1 overflow-auto">
    {@render children()}
  </main>
</div>
