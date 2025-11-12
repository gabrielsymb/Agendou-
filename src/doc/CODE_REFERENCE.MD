# 📚 Referência de Código

Este documento detalha as principais estruturas (structs), métodos e funções do projeto, organizados por arquivo para facilitar a consulta.

---

### `src/calc_preco.rs`

Responsável pelos cálculos de valores e relatórios financeiros.

- **Funções:**
  - `calcular_lucro_total(conn: &Connection) -> Result<f64>`: Calcula a soma dos preços de todos os agendamentos concluídos.
  - `calcular_lucro_mensal(conn: &Connection) -> Result<f64>`: Calcula o lucro de todos os agendamentos concluídos no mês corrente.
  - `calcular_lucro_do_dia(conn: &Connection) -> Result<f64>`: Calcula o lucro de todos os agendamentos concluídos no dia corrente.

---

### `src/licenca.rs`

Gerencia a lógica de licenciamento do sistema.

- **Funções:**
  - `verificar_licenca_sistema(conn: &Connection) -> Result<bool>`: Checa se a licença do sistema está dentro da validade.
  - `atualizar_licenca(conn: &Connection, cliente_id: i32, nova_data: NaiveDateTime) -> Result<()>`: Atualiza a data de validade da licença de um cliente.
  - `gerar_codigo_semanal(chave: &str) -> String`: Gera um código de ativação baseado em uma chave secreta e na semana atual.
  - `gerar_codigo_mensal(chave: &str) -> String`: Gera um código de ativação baseado em uma chave secreta e no mês atual.
  - `validar_codigo(codigo_digitado: &str, chave: &str) -> Option<NaiveDateTime>`: Valida se um código digitado corresponde ao código semanal ou mensal e retorna a nova data de validade.

---

### `src/main.rs`

Ponto de entrada da aplicação.

- **Funções:**
  - `main() -> Result<()>`: Função principal que inicia o programa e chama o menu interativo.

---

### `src/menu.rs`

Controla toda a interface de linha de comando (CLI) para interação com o usuário.

- **Funções:**
  - `menu_principal() -> Result<()>`: Exibe o menu principal e gerencia a navegação do usuário.
  - `cadastrar_cliente(conn: &Connection) -> Result<()>`: Coleta os dados e cadastra um novo cliente.
  - `agendar_servico(conn: &Connection) -> Result<()>`: Guia o usuário no processo de criar um novo agendamento.
  - `ver_agendamentos(conn: &Connection) -> Result<()>`: Lista todos os agendamentos.
  - `validar_licenca_terminal(conn: &Connection) -> Result<()>`: Interface para o usuário validar uma licença.
  - `gerar_relatorio_terminal(conn: &Connection) -> Result<()>`: Interface para gerar relatórios de lucro.
  - `cadastrar_servico(conn: &Connection) -> Result<()>`: Coleta os dados e cadastra um novo serviço.
  - `excluir_servico_terminal(conn: &Connection) -> Result<()>`: Remove um serviço existente.
  - `excluir_cliente_terminal(conn: &Connection) -> Result<()>`: Remove um cliente existente.
  - `excluir_agendamento_terminal(conn: &Connection) -> Result<()>`: Remove um agendamento.
  - `editar_servico_terminal(conn: &Connection) -> Result<()>`: Altera os dados de um serviço.
  - `editar_cliente_terminal(conn: &Connection) -> Result<()>`: Altera os dados de um cliente.
  - `editar_agendamento_terminal(conn: &Connection) -> Result<()>`: Altera os dados de um agendamento.
  - `listar_clientes_terminal(conn: &Connection) -> Result<()>`: Exibe todos os clientes.
  - `listar_servicos_terminal(conn: &Connection) -> Result<()>`: Exibe todos os serviços.
  - `listar_agendamentos_terminal(conn: &Connection) -> Result<()>`: Exibe todos os agendamentos com detalhes.
  - `marcar_agendamento_concluido(conn: &Connection) -> Result<()>`: Marca um agendamento como "concluído".
  - `listar_agendamentos_por_status(conn: &Connection, concluido: bool) -> Result<()>`: Filtra e exibe agendamentos por status (pendente/concluído).
  - `relatorio_produtividade_mensal(conn: &Connection) -> Result<()>`: Exibe o relatório de lucro do mês.
  - `relatorio_produtividade_do_dia(conn: &Connection) -> Result<()>`: Exibe o relatório de lucro do dia.

---

### `src/db/mod.rs`

Módulo central para todas as operações de banco de dados com `rusqlite`.

- **Funções:**
  - `conectar_db() -> Result<Connection>`: Abre uma conexão com o arquivo do banco de dados.
  - `criar_tabelas(conn: &Connection) -> Result<()>`: Cria as tabelas do banco de dados se elas não existirem.
  - `salvar_cliente(conn: &Connection, cliente: &mut Cliente) -> Result<i32>`: Insere um novo cliente no banco.
  - `listar_clientes(conn: &Connection) -> Result<Vec<Cliente>>`: Retorna uma lista de todos os clientes.
  - `buscar_cliente_por_id(conn: &Connection, id: i32) -> Result<Option<Cliente>>`: Busca um cliente pelo seu ID.
  - `salvar_agendamento(conn: &Connection, agendamento: &Agendamento) -> Result<i32>`: Insere um novo agendamento.
  - `buscar_agendamento_por_id(conn: &Connection, id: i32) -> Result<Agendamento>`: Busca um agendamento pelo seu ID.
  - `listar_agendamentos(conn: &Connection) -> Result<Vec<Agendamento>>`: Retorna todos os agendamentos.
  - `verificar_conflito(conn: &Connection, data_hora: NaiveDateTime) -> Result<bool>`: Verifica se já existe um agendamento em um horário específico.
  - `atualizar_agendamento(...) -> Result<()>`: Atualiza os dados de um agendamento.
  - `excluir_agendamento(conn: &Connection, id: i32) -> Result<()>`: Remove um agendamento do banco.
  - `excluir_cliente(conn: &Connection, id: i32) -> Result<()>`: Remove um cliente, se ele não tiver agendamentos.
  - `listar_agendamentos_por_data(conn: &Connection, data: NaiveDateTime) -> Result<Vec<Agendamento>>`: Lista agendamentos de uma data específica.
  - `listar_agendamentos_por_cliente(conn: &Connection, cliente_id: i32) -> Result<Vec<Agendamento>>`: Lista agendamentos de um cliente.
  - `salvar_servico(conn: &Connection, servico: &Servico) -> Result<i32>`: Insere um novo serviço.
  - `listar_servicos(conn: &Connection) -> Result<Vec<Servico>>`: Retorna todos os serviços.
  - `excluir_servico(conn: &Connection, id: i32) -> Result<()>`: Remove um serviço do banco.
  - `buscar_nomes_servicos(conn: &Connection, ids: &[i32]) -> Result<Vec<String>>`: Retorna os nomes dos serviços a partir de uma lista de IDs.

---

### `src/models/mod.rs`

Define as estruturas de dados (modelos) que representam as entidades do negócio.

- **Structs:**
  - `Cliente`: Representa um cliente da barbearia (ID, nome, telefone, email).
  - `Agendamento`: Representa um agendamento (ID, cliente, serviços, data/hora, preço, status).
  - `Servico`: Representa um serviço oferecido (ID, nome, preço).
  - `UsuarioSistema`: Representa um usuário do sistema (não totalmente implementado no CLI).

- **Métodos:**
  - `impl Cliente`:
    - `new(nome: String, telefone: String, email: Option<String>) -> Self`: Método construtor para criar uma nova instância de `Cliente`.
