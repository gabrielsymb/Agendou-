import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, depends }) => {
	depends('app:clientes'); // Informa ao SvelteKit sobre a dependência externa
	try {
		const response = await fetch('http://localhost:3000/clientes');
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
