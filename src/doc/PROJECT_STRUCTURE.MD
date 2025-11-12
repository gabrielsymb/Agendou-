# 🌳 Estrutura do Projeto

Esta é a árvore de diretórios do código-fonte do projeto `barber_agenda`.

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
