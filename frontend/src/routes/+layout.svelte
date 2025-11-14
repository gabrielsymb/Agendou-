<script lang="ts">
	import { page } from '$app/stores';
	import '../app.css';
	import favicon from '$lib/assets/favicon.svg';
	import Toasts from '$lib/components/Toasts.svelte';

	let menuAberto = false;

	const navLinks = [
		{ href: '/', text: 'Início' },
		{ href: '/clientes', text: 'Clientes' },
		{ href: '/servicos', text: 'Serviços' },
		{ href: '/agendamentos', text: 'Agendamentos' }
	];
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
	<link href="/uicons/css/uicons-regular-rounded.css" rel="stylesheet" />
</svelte:head>

<div class="flex min-h-screen bg-gray-100">
	<!-- Sidebar -->
	<aside class="bg-white w-64 p-4 shadow-md hidden md:block">
		<h2 class="text-xl font-bold mb-6">Agendou!</h2>
		<nav class="space-y-2">
			{#each navLinks as link}
				{#if link.href !== $page.url.pathname}
					<a href={link.href} class="block text-gray-700 hover:text-blue-600">{link.text}</a>
				{/if}
			{/each}
		</nav>
	</aside>

	<!-- Mobile menu -->
	<div
		class="md:hidden fixed top-0 left-0 right-0 bg-white shadow z-50 flex items-center justify-between px-4 py-3"
	>
		<h2 class="text-lg font-bold">Agendou!</h2>
		<button aria-label="Abrir menu" aria-expanded={menuAberto} aria-controls="mobile-menu" on:click={() => (menuAberto = !menuAberto)}>
			<svg class="w-6 h-6 text-gray-700" fill="none" stroke="currentColor" viewBox="0 0 24 24" role="img" aria-hidden="true">
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M4 6h16M4 12h16M4 18h16"
				/>
			</svg>
		</button>
	</div>

	{#if menuAberto}
	<div id="mobile-menu" class="fixed top-16 left-0 w-full bg-white shadow z-40 p-4">
			<nav class="space-y-2">
				{#each navLinks as link}
					{#if link.href !== $page.url.pathname}
						<a
							href={link.href}
							class="block text-gray-700 hover:text-blue-600"
							on:click={() => (menuAberto = false)}>{link.text}</a
						>
					{/if}
				{/each}
			</nav>
		</div>
	{/if}

	<!-- Conteúdo principal -->
	<main class="flex-1 p-6 mt-16 md:mt-0">
		<slot />
	</main>
	<Toasts />
</div>