import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND = env.PRIVATE_API_BASE ?? 'http://localhost:3000';

export const GET: RequestHandler = async ({ url }) => {
	const query = url.search ? `?${url.searchParams.toString()}` : '';
	const res = await fetch(`${BACKEND}/availability${query}`);
	const data = await res.text();
	return new Response(data, {
		status: res.status,
		headers: { 'content-type': res.headers.get('content-type') ?? 'application/json' }
	});
};
