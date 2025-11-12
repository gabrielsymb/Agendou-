# 💈 Sistema de Agendamento da Barbearia (Web App)

Este documento descreve a nova arquitetura e o roteiro de desenvolvimento do sistema, que evoluiu de um protótipo em Rust para um aplicativo web moderno, offline-first e de alta performance.

## 🎯 Objetivo Principal

Substituir a agenda de papel por um **aplicativo web funcional que roda em qualquer dispositivo, incluindo iPhones, sem depender de lojas de aplicativos**. O sistema oferece persistência de dados real no navegador, garantindo que o barbeiro possa trabalhar mesmo sem conexão com a internet.

## 🚀 Arquitetura Central

A nova abordagem combina o poder do Flutter para a interface com a robustez do SQLite para o banco de dados, tudo executado no lado do cliente.

- **Frontend:** **Flutter Web**, escolhido por sua capacidade de criar interfaces ricas, compiladas para um padrão web de alta performance (HTML, CSS, CanvasKit).
- **Banco de Dados:** **`sql.js`** (SQLite compilado para WebAssembly), que permite executar o banco de dados SQLite diretamente no navegador. Isso garante a reutilização da lógica de banco de dados já validada e a consistência dos dados.
- **Camada de Persistência:**
  - **Primária:** **Origin Private File System (OPFS)**, uma API moderna que oferece acesso a um sistema de arquivos privado e de alta performance. O banco de dados é salvo como um arquivo, eliminando os gargalos do `IndexedDB`.
  - **Fallback:** **`IndexedDB`**, utilizada como uma alternativa robusta para navegadores que ainda não suportam OPFS, garantindo ampla compatibilidade.
- **Sincronização:** O sistema foi projetado para ser resiliente. O banco de dados é **restaurado automaticamente** da camada de persistência ao iniciar o app e **salvo após cada operação**, prevenindo a perda de dados.

---

##  Kanban de Desenvolvimento (Nova Fase)

O backlog abaixo reflete as etapas para a construção do aplicativo web.

### 🔹 FASE 1: Persistência Web & Fundação

- `[WEB-1]` Configurar um projeto base **Flutter Web**.
- `[WEB-2]` Integrar **`sql.js`** e testar a execução de comandos SQLite básicos no navegador.
- `[WEB-3]` Implementar a camada de persistência com detecção de **OPFS** e fallback para **`IndexedDB`**.
- `[WEB-4]` Criar a lógica para carregar o banco de dados na inicialização e salvá-lo após cada transação (escrita, atualização, exclusão).
- `[WEB-5]` Portar o esquema do banco de dados (`CREATE TABLE`s) do `db/mod.rs` para o ambiente web.

### 🔹 FASE 2: Lógica de Negócios & Interface (UI)

- `[UI-1]` Desenvolver as telas principais com Flutter: Dashboard, Agendamentos, Clientes, Serviços e Relatórios.
- `[LOGIC-1]` Portar as operações de **CRUD de Clientes e Serviços** da lógica Rust para Dart/Flutter.
- `[LOGIC-2]` Portar a lógica de **agendamento**, incluindo a verificação de conflitos de horário.
- `[LOGIC-3]` Portar a geração de **relatórios de produtividade** (lucro diário, mensal, total).
- `[UI-2]` Criar formulários para cadastrar e editar clientes, serviços e agendamentos.

### 🔹 FASE 3: Progressive Web App (PWA) & Funcionalidades Avançadas

- `[PWA-1]` Configurar o app Flutter para ser um **Progressive Web App (PWA)**, permitindo que seja "instalado" na tela inicial dos dispositivos.
- `[PWA-2]` Implementar um **Service Worker** para cache de recursos e melhor performance offline.
- `[FEATURE-1]` (Opcional) Reavaliar e, se necessário, reimplementar a lógica de licenciamento no contexto web.
- `[FEATURE-2]` (Opcional) Adicionar notificações push para lembretes de agendamento.

---

### ✅ Legado (Backend Rust - Concluído)

A implementação original em Rust serviu como uma excelente prova de conceito e continua sendo uma referência valiosa para a lógica de negócios.

- **Structs com `serde`**
- **Conexão com SQLite (`rusqlite`)**
- **Tabelas de Clientes, Serviços e Agendamentos**
- **CRUD para Clientes e Serviços**
- **Criação de agendamentos com `chrono`**
- **Verificação de conflito de horário**
- **Cálculo de preço e lucro**
- **Relatórios de produtividade (dia, mês, total)**
- **Marcar agendamento como concluído**
- **Remarcar/realocar agendamentos**