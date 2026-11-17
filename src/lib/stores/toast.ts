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
    success: (msg: string) => show(msg, 'success'),
    error:   (msg: string) => show(msg, 'error', 5000),
    warning: (msg: string) => show(msg, 'warning'),
    info:    (msg: string) => show(msg, 'info'),
  };
}

export const toast = createToastStore();
