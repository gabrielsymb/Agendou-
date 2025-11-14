# 🌳 Estrutura do Projeto

Esta é a árvore de diretórios do projeto `barber_agenda`.

> **Nota sobre a Monetização:** O escopo do projeto foi alterado. A ideia inicial de um sistema de licença paga (`licenca.rs`) foi abandonada em favor de um modelo baseado em doações (donations). A arquitetura primariamente offline torna um sistema de assinatura facilmente contornável, sendo o modelo de apoio voluntário mais sustentável e honesto para com os usuários.

> **Nota sobre a Arquitetura:** A abordagem de frontend foi alterada. O desenvolvimento com Flutter (`agendou_web`) foi descontinuado. O projeto agora utilizará **Svelte** para o frontend, mantendo o backend em **Rust** intacto. A aplicação completa (backend + frontend) será hospedada na plataforma **Fly.io**.

```markdown
# 🌳 Estrutura do Projeto (atualizada)

Esta é a árvore de diretórios do projeto `Agendou!` (backend Rust + frontend Svelte).

> **Nota sobre a Monetização:** O sistema de licença (`licenca.rs`) foi originalmente planejado, mas o projeto migrou para um modelo baseado em doações; o código ainda mantém campos e tabelas relacionados à validade de licença.

> **Nota sobre a Arquitetura:** Frontend em Svelte (SvelteKit) e backend em Rust (Axum). A aplicação pode rodar em modo CLI ou como servidor HTTP.
```

/
├── beckend/
│ ├── Cargo.toml
│ ├── Cargo.lock
│ └── src/
│ ├── db/
│ │ ├── mod.rs # Acesso SQLite: conexão, criação de tabelas, CRUD, migrações leves (ex.: adiciona `duracao_min` em `servicos` se ausente)
│ │ └── Banco.db # Arquivo de banco (padrão em src/bd/Banco.db em runtime; pode ser sobrescrito por APP_DB_PATH)
│ ├── models/
│ │ └── mod.rs # Structs: Cliente, Agendamento, Servico, UsuarioSistema
│ ├── calc_preco.rs # Relatórios e cálculos de lucro
│ ├── licenca.rs # Lógica de licença/validação (campo de validade mantido, monetização migrada para doações)
│ ├── menu.rs # Menu CLI (opera sobre as mesmas funções de persistência do backend)
│ ├── servicos.rs # Handlers/funcionalidades de serviços (listar/criar/atualizar/excluir)
│ ├── agendamentos.rs # Handlers HTTP para agendamentos + adaptador para payloads flexíveis (timestamps string/numérico)
│ └── main.rs # Ponto de entrada; inicializa servidor/CLI e registra rotas (clientes, serviços, agendamentos)
├── frontend/
│ ├── package.json
│ ├── svelte.config.js
│ ├── tsconfig.json
│ └── src/
│ ├── app.css
│ ├── app.html
│ ├── lib/
│ │ ├── components/ # Componentes Svelte reutilizáveis (cards, modais, toasts)
│ │ └── toast.ts
│ └── routes/
│ ├── +layout.svelte # Layout principal com navegação
│ ├── +page.svelte # Dashboard
│ ├── clientes/
│ │ ├── +page.svelte
│ │ └── +page.server.ts
│ └── servicos/
│ ├── +page.svelte
│ └── +page.ts
│ └── agendamentos/
│ ├── +page.svelte # Listagem e UI de agendamentos
│ └── +page.ts # Carrega agendamentos via /api/agendamentos (proxy)
│ └── api/
│ └── agendamentos/
│ ├── +server.ts # Proxy GET/POST -> /agendamentos (encaminha para BACKEND via PRIVATE_API_BASE)
│ └── [id]/+server.ts # Proxy GET/PUT/DELETE -> /agendamentos/:id
│ └── clientes/ # proxies para clientes também implementados em server-side
│ ├── README.md # Instruções de desenvolvimento e .env (explica PRIVATE_API_BASE para proxy)
│ ├── .env.example # Exemplo de variáveis de ambiente (PRIVATE_API_BASE usado no server-side proxy)
│ └── .env # Arquivo de desenvolvimento local (NÃO versionado)
│ └── src/
│ └── routes/
│ └── api/ # Endpoints server-side (proxy) implementados para encaminhar ao backend
│ └── clientes/
│ ├── +server.ts # GET, POST proxy para /clientes
│ └── [id]/+server.ts # GET, PUT, DELETE proxy para /clientes/:id
├── doc/
│ ├── PROJECT_STRUCTURE.MD # (este arquivo)
│ └── contexto_projeto.txt # Resumo técnico do projeto (gerado)
└── .gitignore

```

```
