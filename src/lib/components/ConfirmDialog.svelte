<script lang="ts">
  import { confirmDialog } from "$lib/stores/confirm";
  import { toast } from "$lib/stores/toast";

  let loading = $state(false);

  async function handleConfirm() {
    if (!$confirmDialog) return;
    loading = true;
    try {
      await $confirmDialog.onConfirm();
      confirmDialog.hide();
    } catch(e) {
      toast.error(String(e));
    } finally {
      loading = false;
    }
  }
</script>

{#if $confirmDialog}
  {@const opts = $confirmDialog}
  <div
    style="position:fixed; inset:0; background:#0008; z-index:9998; display:flex; align-items:center; justify-content:center;"
    onclick={() => !loading && confirmDialog.hide()}
  >
    <div
      style="background:var(--surface); border:1px solid var(--border); border-radius:12px; padding:24px; max-width:380px; width:90%; box-shadow:0 8px 32px #0006;"
      onclick={(e) => e.stopPropagation()}
    >
      <div style="font-size:14px; font-weight:700; color:var(--text); margin-bottom:8px;">
        {opts.title || 'Konfirmasi'}
      </div>
      <div style="font-size:13px; color:var(--text-2); line-height:1.5; margin-bottom:20px;">
        {opts.message}
      </div>
      <div style="display:flex; gap:8px; justify-content:flex-end;">
        <button
          onclick={() => confirmDialog.hide()}
          disabled={loading}
          class="btn btn-ghost"
          style="font-size:12px;"
        >{opts.cancelLabel || 'Batal'}</button>
        <button
          onclick={handleConfirm}
          disabled={loading}
          class="btn btn-primary"
          style="font-size:12px; {opts.danger ? 'background:#dc2626; border-color:#dc2626;' : ''}"
        >{loading ? '...' : (opts.confirmLabel || 'Ya, lanjutkan')}</button>
      </div>
    </div>
  </div>
{/if}
