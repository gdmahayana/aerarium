import { writable } from 'svelte/store';

interface ConfirmOptions {
  title?: string;
  message: string;
  confirmLabel?: string;
  cancelLabel?: string;
  danger?: boolean;
  onConfirm: () => void | Promise<void>;
}

function createConfirmStore() {
  const { subscribe, set } = writable<ConfirmOptions | null>(null);
  return {
    subscribe,
    show: (opts: ConfirmOptions) => set(opts),
    hide: () => set(null),
  };
}

export const confirmDialog = createConfirmStore();

// Helper mudah dipanggil
export function confirm(message: string, onConfirm: () => void | Promise<void>, opts?: Partial<ConfirmOptions>) {
  confirmDialog.show({ message, onConfirm, danger: true, ...opts });
}
