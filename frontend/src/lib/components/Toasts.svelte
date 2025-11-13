<script lang="ts">
	import { toasts } from '$lib/toast';
	import { fly } from 'svelte/transition';

	function getBackgroundColor(type: 'success' | 'error') {
		return type === 'success' ? 'bg-green-500' : 'bg-red-500';
	}
</script>

<div aria-live="assertive" class="pointer-events-none fixed inset-0 flex items-end px-4 py-6 sm:items-start sm:p-6 z-50">
	<div class="flex w-full flex-col items-center space-y-4 sm:items-end">
		{#each $toasts as toast (toast.id)}
			<div
				in:fly={{ y: 100, duration: 300 }}
				out:fly={{ y: 100, duration: 300 }}
				class="pointer-events-auto w-full max-w-sm overflow-hidden rounded-lg {getBackgroundColor(
					toast.type
				)} shadow-lg ring-1 ring-black ring-opacity-5"
			>
				<div class="p-4">
					<div class="flex items-center">
						<div class="flex w-0 flex-1 justify-between">
							<p class="w-0 flex-1 font-medium text-white">{toast.message}</p>
						</div>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>
