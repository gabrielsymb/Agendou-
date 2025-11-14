<script lang="ts">
  import ServicoCard from '$lib/components/ServicoCard.svelte';
  import ModalServico from '$lib/components/ModalServico.svelte';
  import { showToast } from '$lib/toast';
  import Toasts from '$lib/components/Toasts.svelte';
  import ConfirmarExclusao from '$lib/components/ConfirmarExclusao.svelte';
  import { invalidateAll } from '$app/navigation';
  export let data;

  type Servico = { id?: number; nome: string; preco: number; duracao: number };

  let modalAberto = false;
  let editando: Servico | undefined = undefined;

  function abrirModalParaNovo() {
    editando = { nome: '', preco: 0, duracao: 30 };
    modalAberto = true;
  }

  function abrirModalParaEditar(id: number) {
    const s = data.servicos.find((x: any) => x.id === id);
    if (!s) return;
    editando = { id: s.id, nome: s.nome, preco: s.preco, duracao: s.duracao_min };
    modalAberto = true;
  }

  async function salvarServico(event: CustomEvent<Servico>) {
  const servico = event.detail;
    const method = servico.id ? 'PUT' : 'POST';
  const url = servico.id ? `/api/servicos/${servico.id}` : '/api/servicos';

    try {
      // map frontend field `duracao` -> backend `duracao_min`
      const payload: any = { ...servico, duracao_min: servico.duracao };
      delete payload.duracao;

      const res = await fetch(url, {
        method,
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload)
      });
      if (!res.ok) throw new Error('Erro na requisição');
  await invalidateAll();
  showToast(servico.id ? 'Serviço atualizado com sucesso' : 'Serviço salvo com sucesso', 'success');
    } catch (e) {
      console.error(e);
      showToast('Falha ao salvar serviço', 'error');
    }

  modalAberto = false;
  }

  let modalExcluirAberto = false;
  let servicoParaExcluir: Servico | null = null;

  function pedirConfirmacaoExclusao(serv: Servico) {
    servicoParaExcluir = serv;
    modalExcluirAberto = true;
  }

  async function confirmarExclusao() {
    if (!servicoParaExcluir?.id) return;
    try {
      const res = await fetch(`/api/servicos/${servicoParaExcluir.id}`, { method: 'DELETE' });
      if (!res.ok) throw new Error('Erro ao deletar');
      modalExcluirAberto = false;
      servicoParaExcluir = null;
      await invalidateAll();
      showToast('Serviço excluído', 'success');
    } catch (e) {
      console.error(e);
      showToast('Falha ao excluir serviço', 'error');
    }
  }
</script>

<div class="p-4">
  <!-- Toasts -->
  <Toasts />
  <div class="flex justify-between items-center mb-4">
    <h1 class="text-2xl font-bold">Serviços</h1>
    <button on:click={abrirModalParaNovo} class="bg-blue-600 text-white px-4 py-2 rounded">+ Adicionar Serviço</button>
  </div>

  {#if data.servicos && data.servicos.length > 0}
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
      {#each data.servicos as servico}
        <ServicoCard {...servico} onEdit={abrirModalParaEditar} onDelete={() => pedirConfirmacaoExclusao(servico)} />
      {/each}
    </div>
  {:else}
    <p class="text-gray-600">Nenhum serviço cadastrado ainda.</p>
  {/if}

  <ModalServico bind:open={modalAberto} bind:servico={editando} on:salvar={salvarServico} on:fechar={() => (modalAberto = false)} />
  <ConfirmarExclusao bind:open={modalExcluirAberto} nome={servicoParaExcluir?.nome} on:confirmar={confirmarExclusao} on:cancelar={() => (modalExcluirAberto = false)} />
</div>
