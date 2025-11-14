<script lang="ts">
  export let id: number | undefined;
  export let nome: string;
  export let preco: number;
  export let duracao: number;
  export let onEdit: ((id: number) => void) | null = null;
  export let onDelete: ((id: number) => void) | null = null;
</script>

<div class="bg-white rounded shadow p-4 flex flex-col justify-between h-full">
  <div>
    <h3 class="text-lg font-semibold mb-1">{nome}</h3>
    <p class="text-sm text-gray-600">💰 R$ {preco.toFixed(2)}</p>
    <p class="text-sm text-gray-600">⏱️ {duracao} min</p>
  </div>
  <div class="mt-4 flex gap-2 justify-end">
    {#if id !== undefined && onEdit}
      <button class="px-3 py-1 bg-yellow-400 text-white rounded" on:click={() => onEdit && onEdit(id!)}>Editar</button>
    {/if}
    {#if id !== undefined && onDelete}
      <button class="px-3 py-1 bg-red-500 text-white rounded" on:click={() => onDelete && onDelete(id!)}>Excluir</button>
    {/if}
  </div>
</div>
