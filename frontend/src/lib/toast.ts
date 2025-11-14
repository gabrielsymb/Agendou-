import { writable } from 'svelte/store';

export type Toast = { id: number; type: 'success' | 'error' | 'info'; message: string };

const toasts = writable<Toast[]>([]);

let nextId = 1;

function pushToast(type: Toast['type'], message: string, ttl = 3000) {
	const id = nextId++;
	toasts.update((t) => [{ id, type, message }, ...t]);
	setTimeout(() => toasts.update((t) => t.filter((x) => x.id !== id)), ttl);
}

// named helper used across the codebase
export function showToast(message: string, type: Toast['type'] = 'info', ttl?: number) {
	pushToast(type, message, ttl);
}

export const toast = {
	success: (msg: string, ttl?: number) => pushToast('success', msg, ttl),
	error: (msg: string, ttl?: number) => pushToast('error', msg, ttl),
	info: (msg: string, ttl?: number) => pushToast('info', msg, ttl),
	store: toasts
};

export { toasts };
export default toast;
