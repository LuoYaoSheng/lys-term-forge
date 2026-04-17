/**
 * Simple toast notification system with fade-out animation.
 *
 * Usage:
 *   import { showToast } from '@/lib/toast';
 *   showToast('Connection saved', 'success');
 *   showToast('Failed to connect', 'error');
 */

export type ToastType = 'success' | 'error' | 'info';

export interface Toast {
  id: number;
  message: string;
  type: ToastType;
  /** True when toast is fading out (CSS transition in progress) */
  leaving: boolean;
}

let nextId = 0;

const subscribers: Set<(toasts: Toast[]) => void> = new Set();
let toasts: Toast[] = [];

function notify() {
  subscribers.forEach((cb) => cb([...toasts]));
}

export function subscribe(cb: (toasts: Toast[]) => void): () => void {
  subscribers.add(cb);
  cb([...toasts]);
  return () => subscribers.delete(cb);
}

export function showToast(message: string, type: ToastType = 'info', duration = 3000): void {
  const id = nextId++;
  const toast: Toast = { id, message, type, leaving: false };
  toasts = [...toasts, toast];
  notify();

  // After `duration`, mark as leaving (triggers CSS fade-out)
  setTimeout(() => {
    toasts = toasts.map((t) => t.id === id ? { ...t, leaving: true } : t);
    notify();

    // After fade-out animation completes, remove from list
    setTimeout(() => {
      toasts = toasts.filter((t) => t.id !== id);
      notify();
    }, 250);
  }, duration);
}

export function dismissToast(id: number): void {
  toasts = toasts.map((t) => t.id === id ? { ...t, leaving: true } : t);
  notify();

  setTimeout(() => {
    toasts = toasts.filter((t) => t.id !== id);
    notify();
  }, 250);
}
