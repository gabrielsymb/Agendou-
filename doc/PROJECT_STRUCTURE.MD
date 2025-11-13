# 🌳 Estrutura do Projeto

Esta é a árvore de diretórios do projeto `barber_agenda`.

> **Nota sobre a Monetização:** O escopo do projeto foi alterado. A ideia inicial de um sistema de licença paga (`licenca.rs`) foi abandonada em favor de um modelo baseado em doações (donations). A arquitetura primariamente offline torna um sistema de assinatura facilmente contornável, sendo o modelo de apoio voluntário mais sustentável e honesto para com os usuários.

> **Nota sobre a Arquitetura:** A abordagem de frontend foi alterada. O desenvolvimento com Flutter (`agendou_web`) foi descontinuado. O projeto agora utilizará **Svelte** para o frontend, mantendo o backend em **Rust** intacto. A aplicação completa (backend + frontend) será hospedada na plataforma **Fly.io**.

```
/
├── src/
│   ├── db/
│   │   ├── mod.rs      # Módulo de interação com o banco de dados (SQLite).
│   │   └── ss          # Arquivo ou diretório adicional (sem extensão, verificar conteúdo).
│   ├── doc/
│   │   └── READ.MD     # Documentação principal do projeto.
│   ├── models/
│   │   └── mod.rs      # Definição das estruturas de dados (structs) do sistema.
│   ├── calc_preco.rs   # Módulo para cálculo de preços e relatórios de lucro.
│   ├── licenca.rs      # Módulo para gerenciamento e validação de licenças.
│   ├── main.rs         # Ponto de entrada da aplicação.
│   └── menu.rs         # Módulo que controla o menu interativo do terminal (CLI).
├── .gitignore          # Arquivos e diretórios ignorados pelo Git.
├── Cargo.lock          # Arquivo de lock com as versões exatas das dependências.
└── Cargo.toml          # Arquivo de manifesto do projeto Rust (metadados e dependências).

```
