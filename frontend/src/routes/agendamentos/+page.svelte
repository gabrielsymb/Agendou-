<script lang="ts">
  import type { Agendamento, NovoAgendamento, Cliente, Servico } from '$lib/types';
  import { toast } from '$lib/toast';
  import Toast from '$lib/Toast.svelte';
  import Autocomplete from '$lib/components/Autocomplete.svelte';
  import AvailabilityPicker from '$lib/components/AvailabilityPicker.svelte';

  export let data: {
    agendamentos: Agendamento[];
    clientes: Cliente[];
    servicos: Servico[];
  };

  let agendamentos: Agendamento[] = data.agendamentos;
  let clientes: Cliente[] = data.clientes;
  let servicos: Servico[] = data.servicos;

  // Form state
  let selectedClient: number | null = null;
  let clientChosen = false;
  let selectedServices: number[] = [];
  let selectedSlot: string | null = null;

  $: totalPrice = servicos
    .filter(s => selectedServices.includes(s.id))
    .reduce((sum, s) => sum + (s.preco ?? 0), 0);

  // soma de durações dos serviços selecionados
  $: totalDuration = servicos
    .filter(s => selectedServices.includes(s.id))
    .reduce((sum, s) => sum + (s.duracao_min ?? 30), 0);

  let isSubmitting = false;
  // inline add client
  let showAddClient = false;
  let newClientName = '';
  let newClientPhone = '';
  // inline add service
  let showAddService = false;
  let showNewServiceForm = false;
  let newServiceName = '';
  let newServicePrice: number | '' = '';
  let newServiceDuration: number | '' = '';
  let showClientHint = false;

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
  selectedClient = null;
  clientChosen = false;
  selectedSlot = null;
  showAddClient = false;
  showNewServiceForm = false;
  newClientName = '';
  newClientPhone = '';
  newServiceName = '';
  newServicePrice = '';
  newServiceDuration = '';
    } finally {
      isSubmitting = false;
    }
  }

  async function excluirAgendamento(id: number) {
    await fetch(`/api/agendamentos/${id}`, { method: 'DELETE' });
  agendamentos = agendamentos.filter(a => a.id !== id);
  }

  // handle submit with validation
  async function handleSubmit() {
    if (!clientChosen || selectedClient == null) {
      toast.error('Selecione um cliente válido antes de criar o agendamento');
      return;
    }
    if (selectedServices.length === 0) {
      toast.error('Selecione pelo menos um serviço');
      return;
    }
    if (!selectedSlot) {
      toast.error('Selecione um horário');
      return;
    }

  const dataHora = new Date(selectedSlot!).toISOString();
    const novo: NovoAgendamento = {
      cliente_id: selectedClient as number,
      servicos_ids: selectedServices,
      data_hora: dataHora,
      preco: totalPrice,
      concluido: false
    };
    await criarAgendamento(novo);
  }

  // create client helper for inline form
  async function createClient() {
    try {
      const payload = { nome: newClientName, telefone: newClientPhone };
      const res = await fetch('/api/clientes', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
      if (!res.ok) { toast.error('Falha ao criar cliente'); return; }
      const json = await res.json();
      const created = json.data ?? json;
      selectedClient = created.id;
      clientChosen = true;
      showAddClient = false;
      newClientName = '';
      newClientPhone = '';
      toast.success('Cliente criado e selecionado');
    } catch (err) { console.error(err); toast.error('Erro ao criar cliente'); }
  }

  async function createService() {
    try {
      const payload:any = { nome: newServiceName };
      if (newServicePrice !== '') payload.preco = Number(newServicePrice);
      if (newServiceDuration !== '') payload.duracao_min = Number(newServiceDuration);
      const res = await fetch('/api/servicos', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(payload) });
      if (!res.ok) { toast.error('Falha ao criar serviço'); return; }
      const json = await res.json();
      const created = json.data ?? json;
      servicos = [...servicos, created];
      // auto-select the created service
      if (!selectedServices.includes(created.id)) selectedServices = [...selectedServices, created.id];
      showNewServiceForm = false;
      newServiceName = '';
      newServicePrice = '';
      newServiceDuration = '';
      toast.success('Serviço criado e selecionado');
    } catch (err) { console.error(err); toast.error('Erro ao criar serviço'); }
  }

  // helper to lookup service by id
  function getService(id: number) {
    return servicos.find(s => s.id === id);
  }

  // helper to lookup client by id
  function getClient(id: number | null | undefined) {
    return clientes.find(c => c.id === id as number);
  }

  function getServiceName(id: number) {
    return getService(id)?.nome ?? id;
  }

  function getClientName(id: number | null | undefined) {
    const nome = getClient(id)?.nome ?? id;
    if (typeof nome === 'string') return abbreviateName(nome);
    return nome;
  }

  // Abrevia nomes compostos: mantém primeiro e último, iniciais no meio
  function abbreviateName(fullName: string) {
    const parts = fullName.trim().split(/\s+/);
    if (parts.length <= 1) return fullName;
    const first = parts[0];
    const rest = parts.slice(1).map(p => p[0].toUpperCase() + '.').join(' ');
    return `${first} ${rest}`;
  }

  // show confirm card only when all required fields are filled
  $: canConfirm = clientChosen && selectedServices.length > 0 && !!selectedSlot;
</script>

<div class="min-h-screen flex flex-col">
  <header class="pt-0 pb-1 px-2 border-b">
    <h1 class="text-xl font-bold text-center m-0">Agendamentos</h1>
  </header>

  <main class="flex-1 p-4 overflow-auto">
    <section class="max-w-xl mx-auto mb-4">
      <!-- CLIENT CARD -->
  <div class="bg-white rounded-lg shadow p-4 mb-4 relative card-sm-p">
        <h2 class="text-lg font-semibold mb-3">Cliente
          <button type="button" aria-label="Adicionar cliente" class="absolute right-4 top-4 w-8 h-8 rounded-full bg-blue-600 text-white flex items-center justify-center text-lg"
            on:click={() => showAddClient = !showAddClient}
            on:mouseenter={() => showClientHint = true}
            on:mouseleave={() => showClientHint = false}
          >+
          </button>
          {#if showClientHint}
            <div class="absolute right-14 top-3 bg-black text-white text-xs px-2 py-1 rounded">Cadastrar cliente</div>
          {/if}
        </h2>
        <Toast />
        <div class="mb-2">
          <label class="block font-semibold mb-1" for="cliente-input">Quem?</label>
          <div>
            <Autocomplete inputId="cliente-input" fetchUrl="/api/clientes" placeholder="Buscar cliente" on:select={(e) => { selectedClient = e.detail.id; clientChosen = true; }} />
          </div>
          <!-- descrição removida conforme solicitado -->

          {#if showAddClient}
            <div class="mt-3 border rounded p-3 bg-gray-50">
              <h3 class="font-medium mb-2">Novo Cliente</h3>
              <input class="w-full mb-2 border px-2 py-1" placeholder="Nome" bind:value={newClientName} />
              <input class="w-full mb-2 border px-2 py-1" placeholder="Telefone" bind:value={newClientPhone} />
              <div class="flex gap-2">
                  <button class="px-3 py-2 bg-green-600 text-white rounded btn-touch" on:click={createClient}>Criar</button>
                  <button class="px-3 py-2 border rounded btn-touch" on:click={() => { showAddClient = false; }}>Cancelar</button>
              </div>
            </div>
          {/if}
        </div>
      </div>
    </section>

    <section class="max-w-xl mx-auto mb-4">
      <!-- Agendamento card (serviços, horário, confirmar) -->
  <div class="bg-white rounded-lg shadow p-4 card-sm-p">
        <h2 class="text-lg font-semibold mb-3">Agendar Novo Serviço</h2>
  <Toast />
  <p class="text-sm text-gray-600 mb-3">Siga os passos: escolha serviços, escolha um horário e confirme.</p>

      <!-- CARD: Serviços -->
  <div class="bg-white rounded-lg shadow p-4 mb-4 relative card-sm-p">
        <h3 class="font-semibold">Escolha os Serviços
          <button type="button" aria-label="Novo serviço" class="absolute right-4 top-4 w-8 h-8 rounded-full bg-blue-600 text-white flex items-center justify-center text-lg" on:click={() => { showNewServiceForm = !showNewServiceForm; }} >+</button>
        </h3>

        <!-- Autocomplete estático para permitir cadastrar múltiplos serviços -->
        <div class="mt-2 mb-2 pr-12">
          <Autocomplete inputId="servico-input" fetchUrl="/api/servicos" placeholder="Buscar serviço" on:select={(e) => { const id = e.detail.id; if (!selectedServices.includes(id)) selectedServices = [...selectedServices, id]; }} />
        </div>

        {#if showNewServiceForm}
          <div class="mt-3 border rounded p-3 bg-gray-50">
            <h3 class="font-medium mb-2">Novo Serviço</h3>
            <input class="w-full mb-2 border px-2 py-1" placeholder="Nome do serviço" bind:value={newServiceName} />
            <input class="w-full mb-2 border px-2 py-1" placeholder="Preço" bind:value={newServicePrice} />
            <input class="w-full mb-2 border px-2 py-1" placeholder="Duração (min)" bind:value={newServiceDuration} />
            <div class="flex gap-2">
              <button class="px-3 py-2 bg-green-600 text-white rounded btn-touch" on:click={createService}>Criar</button>
              <button class="px-3 py-2 border rounded btn-touch" on:click={() => { showNewServiceForm = false; }}>Cancelar</button>
            </div>
          </div>
        {/if}

        <div class="mb-2 flex flex-wrap gap-2">
          {#each selectedServices as sid}
            {#if getService(sid)}
              <div class="px-3 py-1 bg-gray-100 rounded flex items-center gap-2">
                <span class="text-sm">{getService(sid)?.nome}</span>
                <button type="button" class="text-xs text-red-600" on:click={() => { selectedServices = selectedServices.filter(x => x !== sid); }}>✕</button>
              </div>
            {/if}
          {/each}
        </div>

        {#if selectedServices.length === 0}
          <div class="mt-2 text-sm text-gray-500">Nenhum serviço selecionado</div>
        {:else}
          <div class="mt-2"><strong>Duração estimada: {totalDuration} minutos</strong></div>
          <div class="mt-1"><strong>Preço total: R$ {totalPrice.toFixed(2)}</strong></div>
        {/if}
      </div>

      <!-- CARD: Quando -->
  <div class="bg-white rounded-lg shadow p-4 mb-4 card-sm-p">
        <h3 class="font-semibold mb-2">Quando?</h3>
        <!-- não passar data inicial; AvailabilityPicker inicia vazia e mostrará placeholder -->
  <AvailabilityPicker inputId="availability-date" duracao_min={totalDuration} on:select={(e) => { selectedSlot = e.detail.slot; }} />
        {#if selectedSlot}
          <div class="mt-2 text-sm">Selecionado: <strong>{new Date(selectedSlot!).toLocaleString()}</strong></div>
        {:else}
          <div class="mt-2 text-sm text-gray-500">Selecione data e hora</div>
        {/if}
      </div>

      <!-- CARD: Confirmar - aparece somente quando todos os dados estão preenchidos -->
      {#if canConfirm}
        <div class="bg-white rounded-lg shadow p-4 mb-4">
          <h3 class="font-semibold mb-2">Confirmar</h3>
          <div>
            <div><strong>Cliente:</strong> {getClientName(selectedClient)}</div>
            <div><strong>Serviços:</strong> {selectedServices.map(sid => getServiceName(sid)).join(', ')}</div>
            <div><strong>Horário:</strong> {new Date(selectedSlot!).toLocaleString()}</div>
            <div class="mt-2 flex gap-2">
              <button class="bg-green-500 text-white px-4 py-2 rounded btn-touch" aria-busy={isSubmitting} disabled={isSubmitting} on:click={handleSubmit}>
                {#if isSubmitting}Criando...{:else}Confirmar Agendamento{/if}
              </button>
              <button class="px-4 py-2 border rounded" on:click={() => { selectedClient = null; clientChosen = false; selectedServices = []; selectedSlot = null; }}>Limpar</button>
            </div>
          </div>
        </div>
      {/if}
      </div>
    </section>

    <!-- Lista de agendamentos existente abaixo -->
    <section class="max-w-xl mx-auto">
      <h2 class="font-semibold mb-2">Agendamentos</h2>
      <ul>
        {#each agendamentos as ag}
          <li class="border p-2 mb-2 flex justify-between">
            <div>
              <strong>{getClientName(ag.cliente_id)}</strong> — {new Date(ag.data_hora).toLocaleString()}
              <br />
              Serviços: {ag.servicos_ids?.map(id => getServiceName(id)).join(', ')}
            </div>
            <button class="bg-red-500 text-white px-2 py-1 rounded" on:click={() => excluirAgendamento(ag.id)}>Excluir</button>
          </li>
        {/each}
      </ul>
    </section>
  </main>
</div>

 
