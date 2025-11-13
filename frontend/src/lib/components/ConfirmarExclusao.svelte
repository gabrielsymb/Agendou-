<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	const dispatch = createEventDispatcher();

	export let open = false;
	export let nome = '';

	function confirmar() {
		dispatch('confirmar');
	}

	function cancelar() {
		dispatch('cancelar');
	}
</script>

{#if open}
	<div class="fixed inset-0 z-50 flex items-center justify-center bg-black bg-opacity-50">
		<div class="w-full max-w-sm rounded bg-white p-6 shadow-lg">
			<h2 class="mb-4 text-lg font-bold">Confirmar exclusão</h2>
			<p class="mb-6 text-gray-700">Deseja realmente excluir <strong>{nome}</strong>?</p>

			<div class="mt-6 flex justify-end gap-2">
				<button on:click={cancelar} class="rounded bg-gray-300 px-4 py-2 hover:bg-gray-400">Cancelar</button>
				<button on:click={confirmar} class="rounded bg-red-600 px-4 py-2 text-white hover:bg-red-700">Excluir</button>
			</div>
		</div>
	</div>
{/if}