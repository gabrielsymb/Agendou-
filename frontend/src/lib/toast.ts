import { writable } from 'svelte/store';

export type Toast = {
	id: number;
	message: string;
	type: 'success' | 'error';
};

export const toasts = writable<Toast[]>([]);

export function showToast(message: string, type: 'success' | 'error' = 'success') {
	const id = Date.now();
	toasts.update((all) => [...all, { id, message, type }]);
	setTimeout(() => {
		toasts.update((all) => all.filter((t) => t.id !== id));
	}, 3000); // Auto-dismiss after 3 seconds
}
