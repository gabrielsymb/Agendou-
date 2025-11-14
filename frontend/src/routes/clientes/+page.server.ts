import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, depends }) => {
	depends('app:clientes'); // Informa ao SvelteKit sobre a dependência externa
	try {
		// Usar endpoint relativo que será tratado pelo proxy server-side (/api/clientes)
		const response = await fetch('/api/clientes');
		if (!response.ok) {
			throw new Error(`HTTP error! status: ${response.status}`);
		}
		const clientes = await response.json();
		return { clientes };
	} catch (error) {
		console.error('Failed to fetch clientes:', error);
		return { clientes: [] };
	}
};
