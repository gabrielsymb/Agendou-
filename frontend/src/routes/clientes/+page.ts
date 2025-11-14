export async function load({
	fetch
}: {
	fetch: (input: RequestInfo, init?: RequestInit) => Promise<Response>;
}) {
	const res = await fetch('/api/clientes');
	if (!res.ok) return { clientes: [] };
	try {
		const clientes = await res.json();
		return { clientes };
	} catch (_err) {
		console.error('Malformed JSON in /api/clientes response', _err);
		return { clientes: [] };
	}
}
