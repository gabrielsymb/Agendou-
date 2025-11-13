<script lang="ts">
	import { editIcon, trashIcon } from '$lib/icons/icons.ts';
	export let nome: string;
	export let telefone: string;
	export let email: string | null;
	export let onEditar: () => void;
	export let onExcluir: () => void;
</script>

<div
	class="relative rounded-lg border bg-card text-card-foreground shadow-sm transition-all hover:shadow-md"
>
	<div class="flex flex-col space-y-1.5 p-6">
		<h3 class="text-lg font-semibold leading-none tracking-tight text-gray-800">{nome}</h3>
	</div>
	<div class="p-6 pt-0">
		<p class="text-sm text-gray-600">📞 {telefone}</p>
		<p class="text-sm text-gray-600">✉️ {email ?? 'Sem email'}</p>
	</div>
	<div class="absolute right-2 top-2 flex gap-2">
		<button on:click={onEditar} class="text-blue-600 hover:text-blue-800" title="Editar">
			{@html editIcon}
		</button>
		<button on:click={onExcluir} class="text-red-600 hover:text-red-800" title="Excluir">
			{@html trashIcon}
		</button>
	</div>
</div>