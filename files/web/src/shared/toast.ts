/** Toast notifications: `toast.success(t('saved'))`, `toast.error(errorMessage(e))`. Rendered by app/ToastHost.vue. */
import { reactive } from 'vue';

export interface Toast {
  id: number;
  kind: 'success' | 'error' | 'info';
  message: string;
}

export const toasts = reactive<Toast[]>([]);
let nextId = 1;

function push(kind: Toast['kind'], message: string, timeoutMs = 4000): void {
  const id = nextId++;
  toasts.push({ id, kind, message });
  setTimeout(() => dismiss(id), timeoutMs);
}

export function dismiss(id: number): void {
  const index = toasts.findIndex((t) => t.id === id);
  if (index >= 0) toasts.splice(index, 1);
}

export const toast = {
  success: (message: string) => push('success', message),
  error: (message: string) => push('error', message, 7000),
  info: (message: string) => push('info', message),
};
