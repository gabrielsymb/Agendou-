export const load = async ({
	fetch
}: {
	fetch: (input: RequestInfo, init?: RequestInit) => Promise<Response>;
}) => {
	const [resA, resC, resS] = await Promise.all([
		fetch('/api/agendamentos'),
		fetch('/api/clientes'),
		fetch('/api/servicos')
	]);
	async function safeJson(res: Response) {
		if (!res.ok) {
			// return a sensible default for each endpoint (empty array)
			return [];
		}
		try {
			return await res.json();
		} catch (_err) {
			// malformed/empty JSON
			console.error('Malformed JSON in response', _err);
			return [];
		}
	}

	const [agendamentos, clientes, servicos] = await Promise.all([
		safeJson(resA),
		safeJson(resC),
		safeJson(resS)
	]);

	return { agendamentos, clientes, servicos };
};
