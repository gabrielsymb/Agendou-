# Agendamento — UX Guidelines

Objetivo

Padronizar a experiência de agendamento (mobile-first) do aplicativo: fluxo, componentes, contratos e regras visuais/funcionais. Essas diretrizes documentam as mudanças e decisões implementadas no fluxo "Agendar Novo Serviço".

Resumo das mudanças (estado atual)

- Substituição de grandes dropdowns por Autocomplete incremental para clientes e serviços. (implementado)
- Fluxo visual: Quem → O quê → Quando → Confirmar. (implementado no `frontend/src/routes/agendamentos/+page.svelte`)
- UI Mobile-first com cards: Client Card, Service Card, When Card, Confirm Card. (implementado)
- WheelPicker: rolagem com snap que exibe apenas slots discretos retornados pelo backend; usado dentro de `AvailabilityPicker`.

Fluxo e Hierarquia

1. Quem? (Client Card)

   - Autocomplete com `inputId` para associação de labels (a11y).
   - Botão "Adicionar" abre formulário inline para criar novo cliente; ao criar, seleciona o cliente automaticamente.

2. O quê? (Service Card)

   - Autocomplete de serviços com chips representando serviços selecionados.
   - Duração estimada e Preço total aparecem quando houver serviços selecionados.

3. Quando? (When Card)

   - `AvailabilityPicker` solicita slots ao backend (GET `/api/availability` via proxy) passando `date`, `duracao_min`, `buffer_min` e `granularity_min`.
   - Se `work_windows` não existirem no backend, o componente exibe um mini-setup (fallback local) — objetivo: migrar para persistência via `/api/work_windows`.

4. Confirmar (Confirm Card)

   - Resumo: Cliente | Serviços | Horário | Preço total.
   - Botão Confirmar desabilitado até haver cliente, serviços e horário.
   - Exibição legível: a página carrega listas de `clientes` e `servicos` e faz lookup local para mostrar nomes legíveis. O cliente é exibido com o primeiro nome por extenso e todas as demais palavras abreviadas como iniciais (ex.: "João Pedro Silva" → "João P. S.").

Componentes (contratos curtos e status)

- Autocomplete

  - Props: `inputId`, `fetchUrl`, `placeholder`
  - Events: `select` (payload: item object { id, nome, ... })
  - Status: implementado e usado para clientes e serviços.

- WheelPicker / AvailabilityPicker
  - Props: `slots: string[]` ou `fetchUrl` + query
  - Events: `select` (payload: { slot: ISOString })
  - Contrato: exibe apenas `slots` discretos retornados pelo backend; o front não gera slots localmente (exceto fallback temporário).
  - Status: implementado; `AvailabilityPicker` consome `/api/availability`.

Nota: o frontend usa proxies em `frontend/src/routes/api/*/+server.ts` que encaminham para o backend. Isso permite que o frontend carregue `clientes` e `servicos` no `load` da rota `/agendamentos` e faça lookup local para exibição.

Regras e validações

- Não permitir abrir o picker de disponibilidade se `totalDuration <= 0`.
- Botão Confirmar fica `disabled` enquanto faltar cliente, serviços ou slot.
- Ao criar novo cliente, frontend seleciona o cliente e emite um Toast.

Acessibilidade (a11y)

- Inputs possuem `id` e são associados a `<label>`.
- Botões usam `<button>` com `aria-label` quando necessário.
- WheelPicker usa `scroll-snap` com alternativa por teclado.

API / Backend (contratos)

- GET `/api/clientes?search=...` — Autocomplete de clientes.
- POST `/api/clientes` — criar cliente.
- GET `/api/servicos?search=...` — Autocomplete de serviços.
- GET `/api/availability?date=YYYY-MM-DD&duracao_min=&buffer_min=&granularity_min=` — retorna `{ slots: string[] }`.
- GET `/api/work_windows` e POST `/api/work_windows` — listar/criar janelas de trabalho; usar para persistir schedule da barbearia.

Teste & QA

- Testes manuais recomendados:
  - Fluxo completo: criar cliente inline → adicionar serviços → abrir disponibilidade → selecionar slot → confirmar.
  - Verificar bloqueio do picker quando não há serviços.
  - CRUD de work_windows: criar via UI e verificar disponibilidade atualizada.

Estilo visual

- Mobile-first: cards, espaçamento generoso e botões touch-friendly.
- Chips: `bg-gray-100`, remoção discreta (✕).

Próximos passos

- Remover fallback localStorage em `AvailabilityPicker` quando `work_windows` estiverével no backend.
- Ajustar thresholds de swipe e gestos conforme testes em dispositivos reais.
- Implementar testes unitários para helpers (duração, preço) e um fluxo de integração com Playwright.

Arquivos relevantes (referência atualizada)

- `frontend/src/routes/agendamentos/+page.svelte`
- `frontend/src/lib/components/AvailabilityPicker.svelte`
- `frontend/src/lib/components/WheelPicker.svelte`
- `frontend/src/lib/components/Autocomplete.svelte`
- `frontend/src/lib/components/SideSheet.svelte`
- `beckend/src/db/mod.rs`, `beckend/src/agendamentos.rs`, `beckend/src/main.rs`

---

Document updated to reflect current implementation.
