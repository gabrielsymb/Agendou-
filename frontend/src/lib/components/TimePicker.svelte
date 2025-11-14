<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  export let slots: string[] = [];
  export let selected: string | null = null;
  export let ariaLabel = 'Time picker';

  const dispatch = createEventDispatcher();

  let trackEl: HTMLDivElement | null = null;
  let handleEl: HTMLDivElement | null = null;
  let width = 0;

  let positions: number[] = [];

  function updatePositions() {
    positions = [];
    if (!trackEl) return;
    width = trackEl.clientWidth;
    const n = slots.length;
    if (n === 0) return;
    for (let i = 0; i < n; i++) {
      const pos = (i / Math.max(1, n - 1)) * width;
      positions.push(pos);
    }
  }

  onMount(() => {
    updatePositions();
    window.addEventListener('resize', updatePositions);
    return () => window.removeEventListener('resize', updatePositions);
  });

  // helpers
  function indexOfSlot(slot: string | null) {
    if (!slot) return -1;
    return slots.indexOf(slot);
  }

  function setSelectedByIndex(i: number) {
    if (i < 0 || i >= slots.length) return;
    selected = slots[i];
    dispatch('select', { slot: selected });
  }

  function nearestIndexFromX(x: number) {
    if (positions.length === 0) return -1;
    let best = 0;
    let bestDist = Math.abs(positions[0] - x);
    for (let i = 1; i < positions.length; i++) {
      const d = Math.abs(positions[i] - x);
      if (d < bestDist) { best = i; bestDist = d; }
    }
    return best;
  }

  let dragging = false;

  function onPointerDown(e: PointerEvent) {
    if (!trackEl) return;
    dragging = true;
    (e.target as Element).setPointerCapture(e.pointerId);
    handleMove(e.clientX);
  }

  function onPointerMove(e: PointerEvent) {
    if (!dragging) return;
    handleMove(e.clientX);
  }

  function onPointerUp(e: PointerEvent) {
    if (!dragging) return;
    dragging = false;
    handleRelease(e.clientX);
  }

  function handleMove(clientX: number) {
    if (!trackEl || !handleEl) return;
    const rect = trackEl.getBoundingClientRect();
    const x = Math.max(0, Math.min(clientX - rect.left, rect.width));
    // move handle to x (visually)
    handleEl.style.transform = `translateX(${x - 8}px)`; // center handle (assuming 16px)
  }

  function handleRelease(clientX: number) {
    if (!trackEl || !handleEl) return;
    const rect = trackEl.getBoundingClientRect();
    const x = Math.max(0, Math.min(clientX - rect.left, rect.width));
    const idx = nearestIndexFromX(x);
    setSelectedByIndex(idx);
    // snap handle
    const snap = positions[idx] ?? 0;
    handleEl.style.transform = `translateX(${snap - 8}px)`;
  }

  function onClickMarker(i: number) {
    setSelectedByIndex(i);
    // snap
    if (handleEl) {
      const snap = positions[i] ?? 0;
      handleEl.style.transform = `translateX(${snap - 8}px)`;
    }
  }

  // when slots change or selected changes, recalc
  $: if (slots) {
    updatePositions();
    const idx = indexOfSlot(selected);
    if (idx >= 0 && handleEl) {
      const snap = positions[idx] ?? 0;
      handleEl.style.transform = `translateX(${snap - 8}px)`;
    }
  }

</script>

<style>
  .track { height: 36px; background: #f3f4f6; border-radius: 8px; position: relative; }
  .marker { position: absolute; top: 50%; transform: translateY(-50%); padding: 2px 6px; background: white; border-radius: 6px; border: 1px solid #e5e7eb; font-size: 12px; }
  .handle { position: absolute; top: 50%; transform: translateY(-50%); width: 16px; height: 16px; background: #2563eb; border-radius: 50%; box-shadow: 0 1px 3px rgba(0,0,0,0.2); cursor: grab; }
  .marker-button { background: transparent; border: none; cursor: pointer; }
  .marker-selected { background: #dcfce7; border-color: #86efac; }
  .container { padding: 8px 0; }
</style>

<div class="container" aria-label={ariaLabel} role="group">
  <div bind:this={trackEl} class="track" on:pointermove={onPointerMove} on:pointerup={onPointerUp} on:pointercancel={onPointerUp} on:lostpointercapture={onPointerUp}>
    {#if positions.length === 0}
      <div class="p-2 text-sm">Nenhum slot</div>
    {:else}
      {#each slots as s, i}
        <div
          class="marker"
          style="left: {positions[i]}px;"
        >
          <button class="marker-button {selected === s ? 'marker-selected' : ''}" on:click={() => onClickMarker(i)}>
            {new Date(s).toLocaleTimeString([], {hour: '2-digit', minute: '2-digit'})}
          </button>
        </div>
      {/each}
  <div bind:this={handleEl} class="handle" on:pointerdown={onPointerDown}></div>
    {/if}
  </div>
</div>
