import { writable } from 'svelte/store';

export type ToastType = 'success' | 'error' | 'warning' | 'info';

export interface Toast {
  id: number;
  type: ToastType;
  message: string;
}

function createToastStore() {
  const { subscribe, update } = writable<Toast[]>([]);
  let counter = 0;

  function show(message: string, type: ToastType = 'info', duration = 3000) {
    const id = ++counter;
    update(t => [...t, { id, type, message }]);
    if (duration > 0) setTimeout(() => dismiss(id), duration);
  }

  function dismiss(id: number) {
    update(t => t.filter(x => x.id !== id));
  }

  return { subscribe, show, dismiss,
    success: (msg: string, duration = 3000) => show(msg, 'success', duration),
    error:   (msg: string, duration = 5000) => show(msg, 'error', duration),
    warning: (msg: string, duration = 4000) => show(msg, 'warning', duration),
    info:    (msg: string, duration = 3000) => show(msg, 'info', duration),
  };
}

export const toast = createToastStore();
