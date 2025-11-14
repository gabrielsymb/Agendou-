<script lang="ts">
  import { toast } from './toast';
  import { fly } from 'svelte/transition';
  import { derived } from 'svelte/store';

  const toasts = toast.store;
  const items = derived(toasts, $t => $t);
</script>

<div class="fixed top-4 right-4 z-50 space-y-2">
  {#each $items as t (t.id)}
    <div in:fly={{ y: -6, duration: 150 }} class="rounded p-3 text-white shadow" style="background: {t.type === 'success' ? '#16a34a' : t.type === 'error' ? '#dc2626' : '#2563eb'}">
      {t.message}
    </div>
  {/each}
</div>
