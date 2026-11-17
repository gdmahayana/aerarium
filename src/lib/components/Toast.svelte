<script lang="ts">
  import { toast, type ToastType } from "$lib/stores/toast";

  const icons: Record<ToastType, string> = {
    success: '✓', error: '✕', warning: '⚠', info: 'ℹ',
  };

  const styles: Record<ToastType, { bg: string; border: string; color: string }> = {
    success: { bg: '#052e16', border: '#14532d', color: '#4ade80' },
    error:   { bg: '#2d1515', border: '#7f1d1d', color: '#f87171' },
    warning: { bg: '#2d1f00', border: '#4a3200', color: '#fbbf24' },
    info:    { bg: '#0c1a2e', border: '#1e3a5f', color: '#7dd3fc' },
  };
</script>

{#if $toast.length > 0}
  <div style="
    position:fixed; bottom:24px; right:24px;
    display:flex; flex-direction:column; gap:8px;
    z-index:9999; pointer-events:none;
  ">
    {#each $toast as t (t.id)}
      {@const s = styles[t.type]}
      <div style="
        display:flex; align-items:center; gap:10px;
        padding:10px 14px; border-radius:8px;
        background:{s.bg}; border:1px solid {s.border};
        pointer-events:all; min-width:240px; max-width:380px;
        animation:slideIn 0.15s ease;
      ">
        <span style="color:{s.color}; font-size:13px; font-weight:700; flex-shrink:0;">{icons[t.type]}</span>
        <span style="font-size:12px; color:var(--text); flex:1; line-height:1.4;">{t.message}</span>
        <button
          onclick={() => toast.dismiss(t.id)}
          style="background:none; border:none; color:var(--text-3); cursor:pointer; font-size:14px; flex-shrink:0; padding:0 2px; font-family:inherit;"
        >✕</button>
      </div>
    {/each}
  </div>
{/if}

<style>
  @keyframes slideIn {
    from { opacity: 0; transform: translateX(16px); }
    to   { opacity: 1; transform: translateX(0); }
  }
</style>
