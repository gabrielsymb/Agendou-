export async function load({ fetch }) {
	// Use the fetch provided by SvelteKit's load to respect adapters and proxies.
	const res = await fetch('/api/servicos');
	if (!res.ok) {
		// Forward a user-friendly error to the page
		return { servicos: [], error: `Erro ao carregar serviços: ${res.status}` };
	}
	const servicos = await res.json();
	return { servicos };
}
