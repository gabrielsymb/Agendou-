<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import { tick } from 'svelte';

  export let slots: string[] = [];
  export let ariaLabel = 'Slot wheel picker';

  const dispatch = createEventDispatcher();

  // Looping list internal
  const LOOP_REPEAT = 5; // odd number, center copy
  let loopSlots: string[] = [];

  function buildLoop() {
    loopSlots = [];
    if (!slots || slots.length === 0) return;
    for (let i = 0; i < LOOP_REPEAT; i++) loopSlots.push(...slots);
  }

  // refs
  let col: HTMLDivElement | null = null;
  let scrollTimeout: any = null;

  // selected index in original slots
  let selectedIndex: number = 0;

  onMount(async () => {
    buildLoop();
    // center on middle copy
    await tick();
    if (col && loopSlots.length > 0) {
      const centerCopy = Math.floor(LOOP_REPEAT/2) * slots.length;
      selectedIndex = 0;
      // scroll to centerCopy + selectedIndex
      const item = col.querySelector(`[data-idx="${centerCopy}"]`) as HTMLElement | null;
      if (item) item.scrollIntoView({ block: 'center' });
    }
  });

  $: if (slots) {
    buildLoop();
    // after building loop, try to keep position near middle
    tick().then(() => {
      if (col && loopSlots.length > 0) {
        const centerCopy = Math.floor(LOOP_REPEAT/2) * slots.length + selectedIndex;
        const item = col.querySelector(`[data-idx="${centerCopy}"]`) as HTMLElement | null;
        if (item) item.scrollIntoView({ block: 'center' });
      }
    });
  }

  function findCenter(colEl: HTMLElement) {
    const rect = colEl.getBoundingClientRect();
    const centerY = rect.top + rect.height/2;
    const items = Array.from(colEl.querySelectorAll('[data-idx]')) as HTMLElement[];
    let best: HTMLElement | null = null;
    let bestDist = Infinity;
    for (const it of items) {
      const r = it.getBoundingClientRect();
      const itemCenter = r.top + r.height/2;
      const d = Math.abs(itemCenter - centerY);
      if (d < bestDist) { bestDist = d; best = it; }
    }
    return best;
  }

  function onScroll() {
    if (!col) return;
    if (scrollTimeout) clearTimeout(scrollTimeout);
    scrollTimeout = setTimeout(() => {
      const it = findCenter(col!);
      if (!it) return;
      const idx = Number(it.getAttribute('data-idx'));
      // map to original
      const orig = idx % slots.length;
      selectedIndex = orig;
      // emit slot
      const slot = slots[orig];
      dispatch('select', { slot });
      // haptic
      try { if (navigator && 'vibrate' in navigator) navigator.vibrate?.(10); } catch(_) {}
      // re-center to middle copy for infinite illusion
      const centerCopy = Math.floor(LOOP_REPEAT/2) * slots.length + orig;
      const target = col!.querySelector(`[data-idx="${centerCopy}"]`) as HTMLElement | null;
      if (target) target.scrollIntoView({ block: 'center' });
    }, 120);
  }
</script>

<style>
  .sheet-wheel { width: 100%; display:flex; justify-content:center; }
  .col { width: 10rem; height: 12rem; overflow-y:auto; scroll-snap-type: y mandatory; -webkit-overflow-scrolling: touch; border-radius: 0.6rem; background: linear-gradient(180deg,#fff,#f8fafc); box-shadow: 0 6px 18px rgba(15,23,42,0.06);} 
  .item { height: 3rem; display:flex; align-items:center; justify-content:center; scroll-snap-align:center; font-size:1.125rem; color:#374151; }
  .item.is-selected { font-weight:700; color:#0f172a; }
  .pad { height: 4rem; }
  .center-indicator { position:relative; pointer-events:none; }
  .center-line { position:absolute; left:0; right:0; top:50%; height:3rem; border-top:2px solid rgba(0,0,0,0.06); border-bottom:2px solid rgba(0,0,0,0.02); transform:translateY(-50%); }
</style>

<div class="sheet-wheel" role="group" aria-label={ariaLabel}>
  <div class="relative">
    <div bind:this={col} class="col" on:scroll={onScroll}>
      <div class="pad"></div>
      {#each loopSlots as s, i}
        <div class="item" data-idx={i} class:is-selected={i % slots.length === selectedIndex}>{new Date(s).toLocaleTimeString([], {hour:'2-digit', minute:'2-digit'})}</div>
      {/each}
      <div class="pad"></div>
    </div>
    <div class="center-indicator"><div class="center-line"></div></div>
  </div>
</div>
