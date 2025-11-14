import devtoolsJson from 'vite-plugin-devtools-json';
import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [tailwindcss(), sveltekit(), devtoolsJson()],
	server: {
		proxy: {
			// Encaminha chamadas para /api ao backend durante desenvolvimento
			// Usamos /api/* para evitar conflitar com rotas da SPA (ex: /servicos)
			// Removemos o prefixo /api ao encaminhar (rewrite) para que /api/servicos -> /servicos no backend
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: true,
				rewrite: (path) => path.replace(/^\/api/, '')
			}
		}
	}
});
