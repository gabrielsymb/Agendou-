<script lang="ts">
	import ClienteCard from '$lib/components/ClienteCard.svelte';
	import Modal from '$lib/components/Modal.svelte';
	import ConfirmarExclusao from '$lib/components/ConfirmarExclusao.svelte';
	import { invalidateAll } from '$app/navigation';
	import { toast } from '$lib/toast';

	export let data;

	// Definindo uma interface para o Cliente para reutilização e clareza
	type Cliente = { id?: number | null; nome: string; telefone: string; email: string | null };

	// Estado para o modal de criação/edição
	let modalAberto = false;
	let clienteAtual: Cliente = { id: null, nome: '', telefone: '', email: '' };

	// Estado para o modal de exclusão
	let modalExcluirAberto = false;
	let clienteParaExcluir: Cliente | null = null;

	function abrirModal(cliente: Cliente | null = null) {
		// Se um cliente for passado, cria uma cópia para edição. Senão, usa um objeto vazio para criação.
		clienteAtual = cliente ? { ...cliente } : { id: null, nome: '', telefone: '', email: '' };
		modalAberto = true;
	}

	async function salvarCliente(event: CustomEvent<Cliente>) {
		const cliente = event.detail;
		const url = cliente.id ? `/api/clientes/${cliente.id}` : '/api/clientes';
		const method = cliente.id ? 'PUT' : 'POST';

		try {
			const response = await fetch(url, {
				method: method,
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify(cliente)
			});

			if (!response.ok) {
				throw new Error('Falha ao salvar cliente');
			}

			modalAberto = false;
			await invalidateAll(); // Atualiza os dados da página
			toast.success(cliente.id ? 'Edição concluída com sucesso!' : 'Cliente salvo com sucesso!');
		} catch (error) {
			toast.error('Erro ao salvar cliente.');
		}
	}

	function pedirConfirmacaoExclusao(cliente: Cliente) {
		clienteParaExcluir = cliente;
		modalExcluirAberto = true;
	}

	async function confirmarExclusao() {
		if (!clienteParaExcluir?.id) return;

		try {
			const response = await fetch(`/api/clientes/${clienteParaExcluir.id}`, {
				method: 'DELETE'
			});

			if (!response.ok) {
				throw new Error('Falha ao excluir cliente');
			}

			modalExcluirAberto = false;
			clienteParaExcluir = null;
			await invalidateAll(); // Atualiza os dados
			toast.success('Cliente excluído com sucesso!');
		} catch (error) {
			toast.error('Erro ao excluir cliente.');
		}
	}
</script>

<div class="container mx-auto p-4">
	<div class="mb-4 flex justify-end">
		<button on:click={() => abrirModal()} class="rounded bg-blue-600 px-4 py-2 text-white hover:bg-blue-700"> + Adicionar Cliente </button>
	</div>
	{#if data.clientes.length === 0}
		<p>Nenhum cliente encontrado.</p>
	{:else}
		<div class="grid grid-cols-1 gap-4 md:grid-cols-2 lg:grid-cols-3">
			{#each data.clientes as cliente (cliente.id)}
				<ClienteCard {...cliente} onEditar={() => abrirModal(cliente)} onExcluir={() => pedirConfirmacaoExclusao(cliente)} />
			{/each}
		</div>
	{/if}
</div>

<Modal bind:open={modalAberto} bind:cliente={clienteAtual} on:salvar={salvarCliente} on:fechar={() => (modalAberto = false)} />

<ConfirmarExclusao
	bind:open={modalExcluirAberto}
	nome={clienteParaExcluir?.nome}
	on:confirmar={confirmarExclusao}
	on:cancelar={() => (modalExcluirAberto = false)}
/>