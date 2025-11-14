import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND = env.PRIVATE_API_BASE ?? 'http://localhost:3000';

export const GET: RequestHandler = async () => {
	const res = await fetch(`${BACKEND}/agendamentos`);
	const data = await res.text();
	return new Response(data, {
		status: res.status,
		headers: { 'content-type': res.headers.get('content-type') ?? 'application/json' }
	});
};

export const POST: RequestHandler = async ({ request }) => {
	const body = await request.text();
	const res = await fetch(`${BACKEND}/agendamentos`, {
		method: 'POST',
		headers: { 'Content-Type': 'application/json' },
		body
	});
	const data = await res.text();
	return new Response(data, {
		status: res.status,
		headers: { 'content-type': res.headers.get('content-type') ?? 'application/json' }
	});
};
