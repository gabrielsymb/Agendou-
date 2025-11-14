# 🌳 Estrutura do Projeto (Agendou!)

Resumo conciso da estrutura atual do repositório e responsabilidades principais.

Observações rápidas:

- Frontend: SvelteKit + TypeScript. Componentes reutilizáveis em `frontend/src/lib/components`.
- Backend: Rust (Axum) com persistência em SQLite. Helpers de disponibilidade e work_windows em `beckend/src/db/mod.rs`.

Árvore resumida:

/
├── beckend/
│ ├── Cargo.toml
│ └── src/
│ ├── db/ # Conexão SQLite, criação de tabelas, calcular_disponibilidade, work_windows
│ ├── models/ # Structs: Cliente, Servico, Agendamento
│ ├── agendamentos.rs # Handlers: availability, work_windows, criar/agendar
│ └── main.rs # Registro de rotas e inicialização
├── frontend/
│ ├── package.json
│ └── src/
│ ├── lib/components/ # AvailabilityPicker, WheelPicker, Autocomplete, SideSheet, Toasts, etc.
│ ├── lib/icons.ts # SVG icons
│ └── routes/ # +layout.svelte, agendamentos, clientes, servicos, api proxies
├── doc/ # Documentação: PROJECT_STRUCTURE.MD, contexto_projeto.txt, UX guidelines
└── .gitignore

Principais endpoints (resumido):

- GET /availability -> retorna { slots: string[] } (consumido por AvailabilityPicker)
- GET /work_windows, POST /work_windows -> listar/criar janelas de trabalho
- Proxies em frontend: `frontend/src/routes/api/*/+server.ts` encaminham requisições para o backend usando `PRIVATE_API_BASE`.

Observação: a rota `/agendamentos` no frontend realiza um `load` que busca `/api/agendamentos`, `/api/clientes` e `/api/servicos` e usa esses arrays para renderizar nomes legíveis (fazer lookup local). O frontend também aplica uma regra de exibição de nomes: mantém o primeiro nome por extenso e abrevia as demais palavras como iniciais.

Como rodar (dev):

- Backend: `cd beckend && cargo run`
- Frontend: `cd frontend && npm install && npm run dev -- --host`

Use este arquivo como visão geral; detalhes sobre UX e lógica estão em `AGENDAMENTO_UX_GUIDELINES.md` e `logic_agendamento.md`.
