<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let open = false;
	export let cliente: { id?: number | null; nome: string; telefone: string; email: string | null } = { id: null, nome: '', telefone: '', email: '' };

	function salvar() {
		dispatch('salvar', cliente);
	}

	function fechar() {
		dispatch('fechar');
	}
</script>

{#if open}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
		<div class="w-full max-w-md mx-4 sm:mx-auto rounded bg-white p-6 shadow-lg">
			<h2 class="mb-4 text-xl font-bold">{cliente.id ? 'Editar' : 'Cadastrar'} Cliente</h2>

			<div class="space-y-3">
				<input type="text" bind:value={cliente.nome} placeholder="Nome" class="w-full rounded border px-3 py-2" />
				<input type="text" bind:value={cliente.telefone} placeholder="Telefone" class="w-full rounded border px-3 py-2" />
				<input type="email" bind:value={cliente.email} placeholder="Email" class="w-full rounded border px-3 py-2" />
			</div>

			<div class="mt-6 flex flex-wrap justify-end gap-2">
				<button on:click={fechar} class="rounded bg-gray-300 px-4 py-2 hover:bg-gray-400 min-w-[110px]">Cancelar</button>
				<button on:click={salvar} class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700 min-w-[110px]">Salvar</button>
			</div>
		</div>
	</div>
{/if}