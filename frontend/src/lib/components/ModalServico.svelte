<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  const dispatch = createEventDispatcher();

  export let open = false;
  export let servico: { id?: number; nome: string; preco: number; duracao: number } = { nome: '', preco: 0, duracao: 30 };

  import { showToast } from '$lib/toast';

  function validar(): string | null {
    if (!servico.nome || servico.nome.trim().length < 2) return 'Nome deve ter ao menos 2 caracteres';
    if (servico.preco <= 0) return 'Preço deve ser maior que 0';
    if (!Number.isInteger(Number(servico.duracao)) || servico.duracao <= 0) return 'Duração inválida';
    return null;
  }

  // Generate stable ids for label associations
  const _idPrefix = servico.id ? `serv-${servico.id}-` : `serv-new-${Date.now()}-`;

  function salvar() {
    const err = validar();
    if (err) {
      showToast(err, 'error');
      return;
    }
    dispatch('salvar', { ...servico });
  }

  function fechar() {
    dispatch('fechar');
  }
</script>

{#if open}
  <div class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
    <div class="bg-white p-6 rounded shadow w-full max-w-md mx-4 sm:mx-auto">
      <h2 class="text-xl font-bold mb-4">{servico.id ? 'Editar Serviço' : 'Novo Serviço'}</h2>
  <label for={`${_idPrefix}nome`} class="block text-sm mb-1">Nome</label>
  <input id={`${_idPrefix}nome`} bind:value={servico.nome} placeholder="Nome" class="w-full mb-2 border px-3 py-2 rounded" />

  <label for={`${_idPrefix}preco`} class="block text-sm mb-1">Preço (R$)</label>
  <input id={`${_idPrefix}preco`} type="number" step="0.01" bind:value={servico.preco} placeholder="Preço" class="w-full mb-2 border px-3 py-2 rounded" />

  <label for={`${_idPrefix}duracao`} class="block text-sm mb-1">Duração (min)</label>
  <input id={`${_idPrefix}duracao`} type="number" bind:value={servico.duracao} placeholder="Duração (min)" class="w-full mb-4 border px-3 py-2 rounded" />

      <div class="flex flex-wrap justify-end gap-2">
        <button on:click={fechar} class="bg-gray-300 px-4 py-2 rounded min-w-[100px]">Cancelar</button>
        <button on:click={salvar} class="bg-blue-600 text-white px-4 py-2 rounded min-w-[100px]">Salvar</button>
      </div>
    </div>
  </div>
{/if}
