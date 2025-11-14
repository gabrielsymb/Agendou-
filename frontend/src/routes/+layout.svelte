<script lang="ts">
    import { page } from '$app/stores';
    import '../app.css';
    import favicon from '$lib/assets/favicon.svg';
    import Toasts from '$lib/components/Toasts.svelte';
    import SideSheet from '$lib/components/SideSheet.svelte';
    import { IconHome, IconUsers, IconTools, IconCalendar, IconSettings } from '$lib/icons';
    // A importação 'goto' não é mais necessária, mas vamos mantê-la por segurança caso seja usada em outro lugar
    import { goto } from '$app/navigation'; 

    let menuAberto = false;
    const navLinks = [
        { href: '/', text: 'Início', icon: IconHome },
        { href: '/clientes', text: 'Clientes', icon: IconUsers },
        { href: '/servicos', text: 'Serviços', icon: IconTools },
        { href: '/agendamentos', text: 'Agendamentos', icon: IconCalendar },
        { href: '/configuracoes', text: 'Configurações', icon: IconSettings }
    ];

    // overlay handler
    let overlayOpacity = 1;
    function applyOverlay(op: number) {
        overlayOpacity = op;
        if (typeof document !== 'undefined') {
            const main = document.querySelector('main');
            // Nota: Este código faz o main ficar opaco quando o SideSheet está aberto.
            if (main) (main as HTMLElement).style.opacity = String(op);
        }
    }

    // Código relacionado ao swipe (swipeStartX, onRootPointerDown, onRootPointerUp) foi removido.
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
    <link href="/uicons/css/uicons-regular-rounded.css" rel="stylesheet" />
</svelte:head>

<div class="flex min-h-screen bg-gray-100">
    <!-- Sidebar Desktop -->
    <aside class="bg-white w-64 p-4 shadow-md hidden md:block">
        <div class="flex items-center gap-3 mb-4">
            <div class="w-10 h-10 bg-blue-600 text-white rounded flex items-center justify-center font-bold">A!</div>
            <div>
                <div class="text-lg font-bold">Agendou!</div>
                <div class="text-xs text-gray-500">Painel</div>
            </div>
        </div>
        <hr class="border-gray-200 my-3" />
        <nav class="space-y-2 mt-3">
            {#each navLinks as link}
                <!-- Aplica estilo 'ativo' se for a página atual, senão o padrão -->
                <a 
                    href={link.href} 
                    class={`flex items-center gap-3 p-2 rounded transition-colors ${
                        $page.url.pathname === link.href 
                            ? 'bg-blue-100 text-blue-700 font-semibold' 
                            : 'text-gray-700 hover:bg-gray-50 hover:text-blue-600'
                    }`}
                >
                    <span class="w-5 h-5" aria-hidden="true">{@html link.icon}</span>
                    <span>{link.text}</span>
                </a>
            {/each}
        </nav>
    </aside>

    <!-- Header Mobile -->
    <div class="md:hidden fixed top-0 left-0 right-0 bg-white z-50 flex items-center justify-between px-4 py-3 shadow-sm">
        <h2 class="text-lg font-bold">Agendou!</h2>
        <button aria-label="Abrir menu" aria-expanded={menuAberto} aria-controls="mobile-menu" on:click={() => (menuAberto = !menuAberto)} class="p-2 rounded-full hover:bg-gray-100 transition-colors">
            <svg class="w-6 h-6 text-gray-700" fill="none" stroke="currentColor" viewBox="0 0 24 24" role="img" aria-hidden="true">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
            </svg>
        </button>
    </div>

    <!-- SideSheet para Mobile -->
    <SideSheet open={menuAberto} on:close={() => (menuAberto = false)} on:overlay={(e) => applyOverlay(e.detail.opacity)} width="80vw" side="left">
        <div class="flex items-center gap-3 p-4 border-b border-gray-100 mb-3">
            <div class="w-10 h-10 bg-blue-600 text-white rounded flex items-center justify-center font-bold">A!</div>
            <div>
                <div class="text-lg font-bold">Agendou!</div>
                <div class="text-xs text-gray-500">Menu de Navegação</div>
            </div>
        </div>
        <nav class="p-4 space-y-2">
            {#each navLinks as link}
                <a 
                    href={link.href} 
                    class={`flex items-center gap-3 p-2 rounded transition-colors ${
                        $page.url.pathname === link.href 
                            ? 'bg-blue-100 text-blue-700 font-semibold' 
                            : 'text-gray-800 hover:bg-gray-50 hover:text-blue-600'
                    }`}
                    on:click={() => (menuAberto = false)}
                >
                    <span class="w-6 h-6 text-gray-600" aria-hidden="true">{@html link.icon}</span>
                    <span>{link.text}</span>
                </a>
            {/each}
        </nav>
    </SideSheet>

    <!-- Conteúdo principal -->
    <main 
        class="flex-1 px-4 py-4 pt-20 md:p-6 md:pt-6 overflow-x-hidden transition-opacity duration-300"
        style={`opacity: ${overlayOpacity};`}
    >
        <slot />
    </main>

    <Toasts />
</div>