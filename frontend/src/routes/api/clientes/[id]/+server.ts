import type { RequestHandler } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';

const BACKEND = env.PRIVATE_API_BASE ?? 'http://localhost:3000';

export const GET: RequestHandler = async ({ params }) => {
	const res = await fetch(`${BACKEND}/clientes/${params.id}`);
	const data = await res.text();
	return new Response(data, {
		status: res.status,
		headers: { 'content-type': res.headers.get('content-type') ?? 'application/json' }
	});
};

export const PUT: RequestHandler = async ({ params, request }) => {
	const body = await request.text();
	const res = await fetch(`${BACKEND}/clientes/${params.id}`, {
		method: 'PUT',
		headers: { 'Content-Type': 'application/json' },
		body
	});
	const data = await res.text();
	return new Response(data, {
		status: res.status,
		headers: { 'content-type': res.headers.get('content-type') ?? 'application/json' }
	});
};

export const DELETE: RequestHandler = async ({ params }) => {
	const res = await fetch(`${BACKEND}/clientes/${params.id}`, { method: 'DELETE' });
	const data = await res.text();
	return new Response(data, {
		status: res.status,
		headers: { 'content-type': res.headers.get('content-type') ?? 'application/json' }
	});
};
