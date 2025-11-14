<script lang="ts">
    import { createEventDispatcher } from 'svelte';
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';
    import WheelPicker from './WheelPicker.svelte';

    // start empty so UI shows placeholder
    export let date: string = ''; // YYYY-MM-DD
    export let duracao_min: number = 30;
    export let buffer_min: number = 15;
    export let granularity_min: number = 15;
    export let fetchUrl: string = '/api/availability';
    export let inputId: string = '';

    const dispatch = createEventDispatcher();

    let slots: string[] = [];
    let loading = false;
    let selected: string | null = null;
    let opened = false;
    let expanded = false;
    let attemptedOpenWithoutServices = false;
    let attemptedTimeout: any = null;
    let showSetup = false;
    let setupOpen: string = '08:00';
    let setupClose: string = '18:00';
    const SHOW_LIMIT = 12;

    async function load() {
        loading = true;
        try {
            if (!date) {
                slots = [];
                return;
            }
            if (!browser) {
                slots = [];
                return;
            }
            const url = `${fetchUrl}?date=${encodeURIComponent(date)}&duracao_min=${duracao_min}&buffer_min=${buffer_min}&granularity_min=${granularity_min}`;
            const res = await fetch(url);
            if (!res.ok) {
                slots = [];
                return;
            }
            const json = await res.json();
            slots = json.slots ?? [];
            // auto-select first slot if available
            if (slots.length > 0) {
                selected = slots[0];
                dispatch('select', { slot: selected });
            } else {
                selected = null;
            }
        } finally {
            loading = false;
        }
    }

    function choose(s: string) {
        selected = s;
        dispatch('select', { slot: s });
    }

    function onTimePickerSelect(e: CustomEvent) {
        const s = e.detail?.slot as string | undefined;
        if (s) choose(s);
    }

    // Toggle open - keep wheel picker type; if duration missing, block and inform parent.
    function toggleOpen() {
        if (duracao_min <= 0) {
            // do not open if duration not specified
            opened = false;
            attemptedOpenWithoutServices = true;
            // clear previous timeout if exists
            if (attemptedTimeout) clearTimeout(attemptedTimeout);
            attemptedTimeout = setTimeout(() => { attemptedOpenWithoutServices = false; attemptedTimeout = null; }, 3000);
            // dispatch an event so parent can show UI feedback if desired
            dispatch('need-services');
            return;
        }
        opened = !opened;
        if (opened) {
            // load only on client. If slots already present we still may refresh to keep consistency
            if (browser) load();
        }
    }

    // allow reloading when date changes while open
    // reload when date changes; also auto-load when user changes date even if not opened
    $: if (browser) {
        // Only attempt to refresh when date has a value
        if (date) load();
    }

    function minutesBetween(a: string, b: string, step: number) {
        const [ah, am] = a.split(':').map(Number);
        const [bh, bm] = b.split(':').map(Number);
        const start = new Date(); start.setHours(ah, am, 0, 0);
        const end = new Date(); end.setHours(bh, bm, 0, 0);
        const res: string[] = [];
        for (let t = start.getTime(); t <= end.getTime(); t += step*60000) {
            const d = new Date(t);
            res.push(d.toISOString().slice(0,16)+':00');
        }
        return res;
    }

    async function saveSetup() {
        // try persist to backend via frontend proxy
        const payload = {
            // weekday will be determined from the currently selected date
            weekday: 0,
            start_time: setupOpen,
            end_time: setupClose,
        } as any;

        // Adjust weekday: backend uses num_days_from_monday (0 = Monday).
        // JS getDay(): 0 = Sunday => map to 6, otherwise subtract 1.
        if (date) {
            const jsDay = new Date(date).getDay();
            payload.weekday = jsDay === 0 ? 6 : jsDay - 1;
        }

        let posted = false;
        let res: Response | null = null; // Definido fora do try/catch para melhor escopo

        try {
            if (browser) {
                res = await fetch('/api/work_windows', { 
                    method: 'POST', 
                    headers: { 'Content-Type': 'application/json' }, 
                    body: JSON.stringify(payload) 
                });
            }
            
            // CORREÇÃO: Verificação de 'res' antes de acessar 'res.ok'
            if (res && res.ok) {
                posted = true;
            } else if (res) {
                // Se 'res' existe mas 'res.ok' é falso
                console.warn('Falha ao salvar work_window no backend', res.status);
            }
        } catch (err) {
            console.warn('Erro conectando ao backend para salvar work_window, usando fallback local', err);
        }

        // If backend persisted successfully, reload slots from availability endpoint
        if (posted) {
            showSetup = false;
            // ensure we reload availability for current date
            await load();
            opened = true;
            return;
        }

        // fallback: persist locally and generate simple slots with granularity
        const key = 'work_windows_local';
        try { localStorage.setItem(key, JSON.stringify({ open: setupOpen, close: setupClose })); } catch(e) { /* ignore */ }
        const step = granularity_min || 10;
        const [oh, om] = setupOpen.split(':').map(Number);
        const [ch, cm] = setupClose.split(':').map(Number);
        const start = new Date(); start.setHours(oh, om, 0, 0);
        const end = new Date(); end.setHours(ch, cm, 0, 0);
        const gen: string[] = [];
        for (let t = start.getTime(); t <= end.getTime(); t += step*60000) {
            const d = new Date(t);
            gen.push(d.toISOString());
        }
        slots = gen; // override slots locally
        showSetup = false;
        opened = true;
    }
</script>

<div>
    <div class="mb-2">
        <div class="flex gap-2 items-start">
            <div class="relative flex-1">
                <input id={inputId} type="date" bind:value={date} class="w-full border px-2 py-1 bg-white" />
                {#if !date}
                    <div class="absolute left-3 top-1/2 -translate-y-1/2 pointer-events-none text-gray-500">Data</div>
                {/if}
            </div>
            <div class="w-36">
                <button type="button" class="w-full h-9 text-sm px-2 bg-blue-600 text-white rounded flex items-center justify-center" on:click={toggleOpen}>{opened ? 'Fechar horários' : 'Ver horários'}</button>
            </div>
        </div>

        {#if attemptedOpenWithoutServices}
            <div class="text-sm text-red-600 mt-2">Selecione pelo menos um serviço antes de ver horários.</div>
        {/if}
    </div>

    {#if loading}
        <div>Carregando...</div>
    {:else}
        {#if !opened}
            <!-- not opened yet -->
        {:else}
            {#if slots.length === 0}
                <!-- if no slots, show inline setup for work hours with detailed feedback -->
                <div class="p-3">
                    {#if date}
                        <div class="mb-2 text-sm text-red-600">Nenhum horário disponível para <strong>{new Date(date).toLocaleDateString()}</strong> com <strong>{duracao_min} min</strong>.</div>
                    {:else}
                        <div class="mb-2 text-sm text-red-600">Nenhum horário disponível.</div>
                    {/if}
                    <div class="text-sm mb-2">Defina horário de funcionamento rápido (salvo localmente)</div>
                    {#if attemptedOpenWithoutServices}
                        <div class="text-xs text-amber-600 mt-2">Selecione um serviço para ver horários.</div>
                    {/if}
                </div>
            {:else}
                <!-- Bottom-sheet style wheel picker -->
                <div class="fixed inset-0 z-40">
                    <button type="button" class="absolute inset-0 bg-black/30" aria-label="Fechar painel de horários" on:click={() => { opened = false; }}></button>
                    <div class="absolute left-0 right-0 bottom-0 z-50">
                        <div class="max-w-full mx-auto bg-white rounded-t-xl shadow-xl p-3">
                            <div class="flex justify-between items-center mb-2">
                                <div class="text-sm font-semibold">Escolha horário</div>
                                <button class="px-3 py-1 text-sm text-gray-700" on:click={() => { opened = false; }}>Fechar</button>
                            </div>
                            <WheelPicker {slots} on:select={onTimePickerSelect} ariaLabel="Seletor de horário" />
                        </div>
                    </div>
                </div>
            {/if}

            {#if showSetup}
                <div class="fixed inset-0 z-40">
                    <button type="button" class="absolute inset-0 bg-black/30" aria-label="Fechar setup" on:click={() => { showSetup = false; }}></button>
                    <div class="absolute left-0 right-0 bottom-0 z-50">
                        <div class="max-w-full mx-auto bg-white rounded-t-xl shadow-xl p-4">
                            <div class="text-sm font-semibold mb-2">Defina horário de funcionamento</div>
                            <div class="grid grid-cols-2 gap-2 mb-3">
                                <label for="setup-open" class="text-xs">Abertura</label>
                                <label for="setup-close" class="text-xs">Fechamento</label>
                                <input id="setup-open" type="time" bind:value={setupOpen} class="border px-2 py-1" />
                                <input id="setup-close" type="time" bind:value={setupClose} class="border px-2 py-1" />
                            </div>
                            <div class="flex gap-2">
                                <button class="px-3 py-2 bg-indigo-600 text-white rounded" on:click={saveSetup}>Salvar e continuar</button>
                                <button class="px-3 py-2 border rounded" on:click={() => { showSetup = false; }}>Cancelar</button>
                            </div>
                        </div>
                    </div>
                </div>
            {/if}
        {/if}
    {/if}
</div>