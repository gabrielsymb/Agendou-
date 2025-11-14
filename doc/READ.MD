# 💈 Sistema de Agendamento da Barbearia (Web App)

Este documento descreve a nova arquitetura e o roteiro de desenvolvimento do sistema, que evoluiu de um protótipo em Rust para um aplicativo web moderno, offline-first e de alta performance.

## 🎯 Objetivo Principal

Substituir a agenda de papel por um **aplicativo web funcional que roda em qualquer dispositivo, incluindo iPhones, sem depender de lojas de aplicativos**. O sistema oferece persistência de dados real no navegador, garantindo que o barbeiro possa trabalhar mesmo sem conexão com a internet.

## 🚀 Arquitetura Central (atual)

O projeto atual usa um frontend moderno em SvelteKit e um backend em Rust (Axum) com banco SQLite.

- Frontend: SvelteKit + TypeScript + Vite. Componentes reutilizáveis estão em `frontend/src/lib/components`.
- Backend: Rust + Axum; persistência em SQLite localizada no diretório `beckend/`.
- Comunicação: o frontend expõe proxies server-side em `frontend/src/routes/api/*/+server.ts` que encaminham chamadas para o backend usando a variável `PRIVATE_API_BASE`.

### Como rodar (dev)

- Backend:

```powershell
cd C:\rust\Agendou!\beckend
cargo run
```

- Frontend:

```powershell
cd C:\rust\Agendou!\frontend
npm install
npm run dev -- --host
```

### Comportamento relevante documentado

- O frontend faz `load` em `/agendamentos` buscando `/api/agendamentos`, `/api/clientes` e `/api/servicos` e faz lookup local para exibir nomes legíveis na UI.
- Na lista e no cartão de confirmação o cliente é exibido pelo primeiro nome completo e as demais palavras são abreviadas como iniciais (ex.: "João Pedro Silva" → "João P. S.").

### Endpoints úteis (proxies)

- GET /api/availability (proxy) → backend /availability
- GET /api/work_windows, POST /api/work_windows (proxy)
- Clientes/Serviços/Agendamentos têm proxies em `frontend/src/routes/api/`.

### Notas

- Alguns componentes importantes: `AvailabilityPicker.svelte`, `WheelPicker.svelte`, `Autocomplete.svelte`, `SideSheet.svelte`.
- Remova referências antigas a Flutter/OPFS; o estado atual do repositório é SvelteKit (frontend) + Rust (backend).

Fim.
