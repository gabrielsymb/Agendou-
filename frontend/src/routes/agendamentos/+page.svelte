<script lang="ts">
  import type { Agendamento, NovoAgendamento, Cliente, Servico } from '$lib/types';
  import { toast } from '$lib/toast';
  import Toast from '$lib/Toast.svelte';

  export let data: {
    agendamentos: Agendamento[];
    clientes: Cliente[];
    servicos: Servico[];
  };

  let agendamentos: Agendamento[] = data.agendamentos;
  let clientes: Cliente[] = data.clientes;
  let servicos: Servico[] = data.servicos;

  // Form state
  let selectedClient: number = clientes[0]?.id ?? 1;
  let selectedServices: number[] = [];

  $: totalPrice = servicos
    .filter(s => selectedServices.includes(s.id))
    .reduce((sum, s) => sum + (s.preco ?? 0), 0);

  let isSubmitting = false;

  async function criarAgendamento(novo: NovoAgendamento) {
    isSubmitting = true;
    try {
      const res = await fetch('/api/agendamentos', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(novo)
      });

      if (!res.ok) {
        const text = await res.text();
        toast.error(`Erro ao criar agendamento: ${text}`);
        return;
      }

      const criado = await res.json();
      const quadro = (criado.data ?? criado) as Agendamento;
      agendamentos = [...agendamentos, quadro];
      toast.success('Agendamento criado com sucesso!');

      // reset form
      selectedServices = [];
      selectedClient = clientes[0]?.id ?? selectedClient;
    } finally {
      isSubmitting = false;
    }
  }

  async function excluirAgendamento(id: number) {
    await fetch(`/api/agendamentos/${id}`, { method: 'DELETE' });
  agendamentos = agendamentos.filter(a => a.id !== id);
  }
</script>

<h1 class="text-xl font-bold mb-4">Agendamentos</h1>

<ul>
  {#each agendamentos as ag}
    <li class="border p-2 mb-2 flex justify-between">
      <div>
        <strong>{ag.cliente_id}</strong> — {new Date(ag.data_hora).toLocaleString()}
        <br />
        Serviços: {ag.servicos_ids?.join(', ')}
      </div>
      <button class="bg-red-500 text-white px-2 py-1 rounded"
              on:click={() => excluirAgendamento(ag.id)}>
        Excluir
      </button>
    </li>
  {/each}
</ul>

<form on:submit|preventDefault={() => {
  const novo: NovoAgendamento = {
    cliente_id: selectedClient,
    servicos_ids: selectedServices,
    data_hora: new Date().toISOString(),
    preco: totalPrice,
    concluido: false
  };
  criarAgendamento(novo);
}}>
  <Toast />
  <div class="mb-2">
    <label for="cliente-select" class="block font-semibold mb-1">Cliente</label>
    <select id="cliente-select" bind:value={selectedClient} class="border rounded px-2 py-1">
      {#each clientes as c}
        <option value={c.id}>{c.nome}</option>
      {/each}
    </select>
  </div>

  <fieldset class="mb-2">
    <legend class="block font-semibold mb-1">Serviços</legend>
    {#each servicos as s}
      <label class="flex items-center gap-2 mb-1" for={`servico-${s.id}`}>
        <input id={`servico-${s.id}`} type="checkbox" value={s.id} on:change={(e) => {
          const id = Number((e.target as HTMLInputElement).value);
          if ((e.target as HTMLInputElement).checked) {
            selectedServices = [...selectedServices, id];
          } else {
            selectedServices = selectedServices.filter(x => x !== id);
          }
        }} />
        <span>{s.nome} — R$ {s.preco.toFixed(2)}</span>
      </label>
    {/each}
  </fieldset>

  <div class="mb-4">
    <strong>Preço total: R$ {totalPrice.toFixed(2)}</strong>
  </div>

  <button class="bg-green-500 text-white px-4 py-2 rounded mt-4" aria-busy={isSubmitting} disabled={isSubmitting}>
    {#if isSubmitting}
      Criando...
    {:else}
      Novo Agendamento
    {/if}
  </button>
</form>
