<script lang="ts">
  import { createEventDispatcher, onDestroy } from 'svelte';
  import { browser } from '$app/environment';
  import { writable } from 'svelte/store';

  export let placeholder = 'Buscar...';
  export let fetchUrl = '/api/clientes'; // endpoint que aceita ?search=&limit=
  // opcional: id para associar <label for="..."> ao input
  export let inputId: string = '';
  export let minChars = 2;
  export let limit = 15;

  const dispatch = createEventDispatcher();
  let input = '';
  let results: Array<any> = [];
  let open = false;
  let selectedIndex = -1;
  let abortController: AbortController | null = null;

  const cache = new Map<string, Array<any>>();

  const debounceMs = 300;
  let timer: any = null;

  function clearTimer() {
    if (timer) clearTimeout(timer);
    timer = null;
  }

  function onInput(e: Event) {
    // usar Event para compatibilidade com o tipo esperado pelo Svelte/TS
    input = (e.currentTarget as HTMLInputElement).value;
    clearTimer();
    if (input.length < minChars) {
      results = [];
      open = false;
      return;
    }
    timer = setTimeout(fetchResults, debounceMs);
  }

  async function fetchResults() {
  if (!browser) return;
    const q = input.trim();
    if (cache.has(q)) {
      results = cache.get(q) ?? [];
      open = results.length > 0;
      return;
    }

    if (abortController) abortController.abort();
    abortController = new AbortController();
    try {
      const url = `${fetchUrl}?search=${encodeURIComponent(q)}&limit=${limit}`;
      const res = await fetch(url, { signal: abortController.signal });
      if (!res.ok) {
        results = [];
        open = false;
        return;
      }
      const json = await res.json();
      results = json;
      cache.set(q, results);
      open = results.length > 0;
      selectedIndex = -1;
    } catch (err) {
      if ((err as any).name === 'AbortError') return;
      console.error('Autocomplete fetch error', err);
      results = [];
      open = false;
    }
  }

  function choose(item: any) {
    dispatch('select', item);
    input = item.nome ?? item.name ?? '';
    open = false;
  }

  function onKeyDown(e: KeyboardEvent) {
    if (!open) return;
    if (e.key === 'ArrowDown') {
      selectedIndex = Math.min(selectedIndex + 1, results.length - 1);
      e.preventDefault();
    } else if (e.key === 'ArrowUp') {
      selectedIndex = Math.max(selectedIndex - 1, 0);
      e.preventDefault();
    } else if (e.key === 'Enter') {
      if (selectedIndex >= 0 && selectedIndex < results.length) {
        choose(results[selectedIndex]);
        e.preventDefault();
      }
    } else if (e.key === 'Escape') {
      open = false;
    }
  }

  onDestroy(() => {
    clearTimer();
    if (abortController) abortController.abort();
  });
</script>

  <div class="relative">
    <input
      id={inputId}
      class="border rounded px-2 py-1 w-full"
      type="text"
      bind:value={input}
      on:input={onInput}
      on:keydown={onKeyDown}
      placeholder={placeholder}
      aria-autocomplete="list"
      aria-controls={inputId ? `${inputId}-listbox` : undefined}
    />
    {#if open}
      <ul id={inputId ? `${inputId}-listbox` : undefined} role="listbox" class="absolute z-50 w-full bg-white border mt-1 max-h-56 overflow-auto">
        {#each results as r, i}
          <li>
            <button
              type="button"
              role="option"
              aria-selected={i === selectedIndex}
              class="w-full text-left px-2 py-1 hover:bg-gray-100 {i === selectedIndex ? 'bg-gray-100' : ''}"
              on:click={() => choose(r)}
            >
              {r.nome ?? r.name}
            </button>
          </li>
        {/each}
      </ul>
    {/if}
  </div>

<style>
  /* minimal styles; project may override */
</style>
