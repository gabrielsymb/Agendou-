<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from 'svelte';
  const dispatch = createEventDispatcher();
  export let open = false;
  export let side: 'left' | 'right' = 'left';
  export let width = '80vw'; // default width

  let sheetEl: HTMLElement | null = null;

  // touch drag state
  let dragging = false;
  let startX = 0;
  let currentX = 0;
  let translate = 0; // px
  let sheetWidth = 0; // px

  function close() {
    dispatch('close');
    // reset drag state
    dragging = false;
    translate = 0;
  }

  function onKey(e: KeyboardEvent) {
    if (e.key === 'Escape') close();
  }

  function updateWidth() {
    if (sheetEl) sheetWidth = sheetEl.clientWidth || 0;
  }

  function onTouchStart(e: TouchEvent) {
    if (!open) return;
    const t = e.touches[0];
    startX = t.clientX;
    currentX = startX;
    dragging = true;
    updateWidth();
  }

  function onTouchMove(e: TouchEvent) {
    if (!dragging) return;
    const t = e.touches[0];
    currentX = t.clientX;
    const delta = currentX - startX;
    if (side === 'left') {
      // left sheet: dragging to left (negative delta) should move it off-screen
      translate = Math.min(0, delta);
    } else {
      // right sheet: dragging to left (negative delta) closes, so translate negative
      translate = Math.max(0, delta);
    }
  // prevent vertical scroll while dragging horizontally if possible
  if (Math.abs(delta) > 6 && e.cancelable) e.preventDefault();
  e.stopPropagation();
  // dispatch overlay opacity
  const opacity = Math.max(0, 1 - (Math.abs(translate) / Math.max(1, sheetWidth)));
  dispatch('overlay', { opacity });
  }

  function onTouchEnd() {
    if (!dragging) return;
    const abs = Math.abs(translate);
    const threshold = Math.max(56, sheetWidth * 0.3); // at least 56px or 30%
    if (abs > threshold) {
      // consider it a swipe to close
      close();
    } else {
      // snap back
      translate = 0;
      dragging = false;
    }
    if (typeof window !== 'undefined') {
      window.removeEventListener('pointermove', onPointerMove as any);
      window.removeEventListener('pointerup', onPointerUp as any);
    }
  }

  // Pointer events (more reliable across devices and during SSR hydration)
  function onPointerDown(e: PointerEvent) {
    if (!open) return;
    // only start on primary button/touch
    if ((e as PointerEvent).isPrimary === false) return;
    startX = e.clientX;
    currentX = startX;
    dragging = true;
    updateWidth();
    try {
      sheetEl?.setPointerCapture((e as PointerEvent).pointerId);
    } catch {}
    // add global listeners to ensure we capture move/up even if pointer leaves element
    if (typeof window !== 'undefined') {
      window.addEventListener('pointermove', onPointerMove as any, { passive: false });
      window.addEventListener('pointerup', onPointerUp as any);
    }
  }

  function onPointerMove(e: PointerEvent) {
    if (!dragging) return;
    currentX = e.clientX;
    const delta = currentX - startX;
    if (side === 'left') {
      translate = Math.min(0, delta);
    } else {
      translate = Math.max(0, delta);
    }
  if (Math.abs(delta) > 6 && e.cancelable) e.preventDefault();
  e.stopPropagation();
  const opacity = Math.max(0, 1 - (Math.abs(translate) / Math.max(1, sheetWidth)));
  dispatch('overlay', { opacity });
  }

  function onPointerUp(e: PointerEvent) {
    if (!dragging) return;
    try {
      sheetEl?.releasePointerCapture((e as PointerEvent).pointerId);
    } catch {}
    const abs = Math.abs(translate);
    const threshold = Math.max(56, sheetWidth * 0.3);
    if (abs > threshold) {
      close();
    } else {
      translate = 0;
      dragging = false;
    }
    if (typeof window !== 'undefined') {
      window.removeEventListener('pointermove', onPointerMove as any);
      window.removeEventListener('pointerup', onPointerUp as any);
    }
  }

  onMount(() => {
    if (typeof document !== 'undefined') {
      document.addEventListener('keydown', onKey);
    }
    if (typeof window !== 'undefined') {
      window.addEventListener('resize', updateWidth);
      updateWidth();
    }
  });
  onDestroy(() => {
    if (typeof document !== 'undefined') {
      document.removeEventListener('keydown', onKey);
    }
    if (typeof window !== 'undefined') {
      window.removeEventListener('resize', updateWidth);
    }
  });
</script>

{#if open}
  <div class="ss-backdrop" on:click={close} aria-hidden="true" style="opacity:{Math.max(0, 1 - (Math.abs(translate) / Math.max(1, sheetWidth)))}"></div>
{/if}

<div
  bind:this={sheetEl}
  class:open={open}
  class:dragging={dragging}
  class="ss-sheet {side}"
  on:touchstart|capture={onTouchStart}
  on:touchmove|capture={onTouchMove}
  on:touchend|capture={onTouchEnd}
  on:pointerdown|capture={onPointerDown}
  on:pointermove|capture={onPointerMove}
  on:pointerup|capture={onPointerUp}
  role="dialog"
  aria-modal="true"
  style="width:{width}; transform: {dragging ? (side === 'left' ? `translateX(${translate}px)` : `translateX(${translate}px)`) : ''};"
>
  <div class="ss-handle" on:pointerdown|capture={onPointerDown} on:touchstart|capture={onTouchStart} aria-hidden="true"></div>
  <div class="ss-content"><slot /></div>
</div>

<style>
  :global(.ss-backdrop) {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.32);
    z-index: 60;
    opacity: 1;
    transition: opacity 220ms ease-out;
  }

  .ss-sheet {
    position: fixed;
    top: 0;
    bottom: 0;
    z-index: 70;
  background: white;
  /* shadow removed as requested */
  box-shadow: none;
    transition: transform 320ms cubic-bezier(.1,.9,.2,1), box-shadow 320ms ease;
  display: flex;
    flex-direction: column;
    padding: 16px;
    border-top-right-radius: 12px;
    border-bottom-right-radius: 12px;
    overflow-y: auto;
  touch-action: pan-y;
  }

  .ss-sheet.left { left: 0; --tx: -100%; transform: translateX(var(--tx)); border-top-right-radius: 12px; border-bottom-right-radius: 12px; }
  .ss-sheet.right { right: 0; --tx: 100%; transform: translateX(var(--tx)); border-top-left-radius: 12px; border-bottom-left-radius: 12px; }

  .ss-sheet.open.left { --tx: 0; }
  .ss-sheet.open.right { --tx: 0; }

  /* while dragging, remove transition for immediate follow */
  .ss-sheet.dragging { transition: none; }

  /* invisible handle area to capture drag gestures reliably */
  .ss-handle {
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 28px;
    z-index: 80;
    touch-action: none;
    cursor: grab;
  }

  .ss-content { width: 100%; }

  /* largura fixa para desktops */
  @media(min-width: 768px) {
    .ss-sheet { width: 360px; }
  }

  @media (prefers-reduced-motion: reduce) {
    .ss-backdrop, .ss-sheet { transition: none; }
  }
</style>
