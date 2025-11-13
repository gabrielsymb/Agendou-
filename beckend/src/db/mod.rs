// --- Módulos & Bibliotecas ---
use rusqlite::{Connection, Result, params};
use chrono::{NaiveDateTime, Datelike, DateTime, Utc};
use std::collections::HashMap;
use crate::models::{Cliente, Agendamento, Servico};
use rusqlite::ToSql;


// Caminho do banco de dados
const DB_PATH: &str = "barbearia.db";

// =================================================================================
// 1. INFRAESTRUTURA
// =================================================================================

pub fn conectar_db() -> Result<Connection> {
    Connection::open(DB_PATH)
}

pub fn criar_tabelas(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS clientes (
            id                INTEGER PRIMARY KEY,
            nome              TEXT NOT NULL,
            telefone          TEXT NOT NULL,
            email             TEXT,
            validade_licenca  INTEGER
        )", [],
)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS agendamentos (
            id          INTEGER PRIMARY KEY,
            cliente_id  INTEGER NOT NULL,
            data_hora   INTEGER NOT NULL,
            preco       REAL NOT NULL,
            concluido   BOOLEAN NOT NULL CHECK (concluido IN (0, 1)),
            FOREIGN KEY(cliente_id) REFERENCES clientes(id) ON DELETE CASCADE
        )", [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS agendamento_servicos (
            agendamento_id INTEGER NOT NULL,
            servico_id     INTEGER NOT NULL,
            PRIMARY KEY (agendamento_id, servico_id),
            FOREIGN KEY (agendamento_id) REFERENCES agendamentos(id) ON DELETE CASCADE,
            FOREIGN KEY (servico_id) REFERENCES servicos(id) ON DELETE CASCADE
        )", [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS sistema (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            validade_licenca INTEGER
        )",
        [],
    )?;

    conn.execute(
        "INSERT OR IGNORE INTO sistema (id, validade_licenca) VALUES (1, 0)",
        [],
    )?;

    criar_tabela_servicos(conn)?;
    Ok(())
}

pub fn criar_tabela_servicos(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS servicos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            preco REAL NOT NULL
        )",
        [],
    )?;

    Ok(())
}

// =================================================================================
// 2. CLIENTES
// =================================================================================

pub fn salvar_cliente(conn: &Connection, cliente: &mut Cliente) -> Result<i32> {
    match cliente.id {
        Some(id) => {
            // ID existe, então UPDATE
            conn.execute(
                "UPDATE clientes SET nome = ?1, telefone = ?2, email = ?3 WHERE id = ?4",
                params![cliente.nome, cliente.telefone, cliente.email, id],
            )?;
            Ok(id)
        }
        None => {
            // ID não existe, então INSERE
            conn.execute(
                "INSERT INTO clientes (nome, telefone, email) VALUES (?1, ?2, ?3)",
                params![cliente.nome, cliente.telefone, cliente.email],
            )?;
            let id = conn.last_insert_rowid() as i32;
            cliente.id = Some(id);
            Ok(id)
        }
    }
}

pub fn listar_clientes(conn: &Connection) -> Result<Vec<Cliente>> {
    let mut stmt = conn.prepare("SELECT id, nome, telefone, email FROM clientes")?;
    let clientes = stmt.query_map([], |row| {
        Ok(Cliente {
            id: row.get(0)?,
            nome: row.get(1)?,
            telefone: row.get(2)?,
            email: row.get(3)?,
        })
    })?.collect();
    clientes
}

pub fn buscar_cliente_por_id(conn: &Connection, id: i32) -> Result<Option<Cliente>> {
    let mut stmt = conn.prepare("SELECT id, nome, telefone, email FROM clientes WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Cliente {
            id: row.get(0)?,
            nome: row.get(1)?,
            telefone: row.get(2)?,
            email: row.get(3)?,
        }))
    } else {
        Ok(None)
    }
}


// =================================================================================
// 3. AGENDAMENTOS
// =================================================================================

fn timestamp_para_naive(ts: i64) -> NaiveDateTime {
    match DateTime::<Utc>::from_timestamp(ts, 0) {
        Some(dt) => dt.naive_local(),
        None => DateTime::<Utc>::from_timestamp(0, 0).unwrap().naive_local(),
    }
}


pub fn salvar_agendamento(conn: &Connection, agendamento: &Agendamento) -> Result<i32> {
    conn.execute(
        "INSERT INTO agendamentos (cliente_id, data_hora, preco, concluido)
         VALUES (?1, ?2, ?3, ?4)",
        params![
            agendamento.cliente_id,
            agendamento.data_hora.and_utc().timestamp(),
            agendamento.preco,
            agendamento.concluido,
        ],
    )?;
    let agendamento_id = conn.last_insert_rowid() as i32;

    for servico_id in &agendamento.servicos_ids {
        conn.execute(
            "INSERT INTO agendamento_servicos (agendamento_id, servico_id)
             VALUES (?1, ?2)",
            params![agendamento_id, servico_id],
        )?;
    }

    Ok(agendamento_id)
}

pub fn buscar_agendamento_por_id(conn: &Connection, id: i32) -> Result<Agendamento> {
    let mut agendamento: Agendamento = conn.query_row(
        "SELECT id, cliente_id, data_hora, preco, concluido FROM agendamentos WHERE id = ?1",
        params![id],
        |row| Ok(Agendamento {
            id: row.get(0)?,
            cliente_id: row.get(1)?,
            servicos_ids: Vec::new(), // Initialize as empty, will be populated next
            data_hora: timestamp_para_naive(row.get::<_, i64>(2)?),
            preco: row.get(3)?,
            concluido: row.get(4)?,
        }),
    )?;

    let mut stmt = conn.prepare("SELECT servico_id FROM agendamento_servicos WHERE agendamento_id = ?1")?;
    let servico_ids_iter = stmt.query_map(params![id], |row| row.get(0))?;
    
    let mut servicos_ids = Vec::new();
    for servico_id_result in servico_ids_iter {
        servicos_ids.push(servico_id_result?);
    }
    agendamento.servicos_ids = servicos_ids;

    Ok(agendamento)
}

pub fn listar_agendamentos(conn: &Connection) -> Result<Vec<Agendamento>> {
    let mut stmt = conn.prepare(
        "SELECT id, cliente_id, data_hora, preco, concluido FROM agendamentos"
    )?;
    let agendamentos_iter = stmt.query_map([], |row| {
        Ok(Agendamento {
            id: row.get(0)?,
            cliente_id: row.get(1)?,
            servicos_ids: Vec::new(), // Initialize as empty
            data_hora: timestamp_para_naive(row.get::<_, i64>(2)?),
            preco: row.get(3)?,
            concluido: row.get(4)?,
        })
    })?;

    let mut agendamentos_com_servicos = Vec::new();
    for agendamento_result in agendamentos_iter {
        let mut agendamento = agendamento_result?;
        if let Some(agendamento_id) = agendamento.id {
            let mut stmt_servicos = conn.prepare("SELECT servico_id FROM agendamento_servicos WHERE agendamento_id = ?1")?;
            let servico_ids_iter = stmt_servicos.query_map(params![agendamento_id], |row| row.get(0))?;
            
            let mut servicos_ids = Vec::new();
            for servico_id_result in servico_ids_iter {
                servicos_ids.push(servico_id_result?);
            }
            agendamento.servicos_ids = servicos_ids;
        }
        agendamentos_com_servicos.push(agendamento);
    }
    Ok(agendamentos_com_servicos)
}

pub fn verificar_conflito(conn: &Connection, data_hora: NaiveDateTime) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM agendamentos WHERE data_hora = ?1 AND concluido = 0")?;
    let count: i32 = stmt.query_row(params![data_hora.and_utc().timestamp()], |row| row.get(0))?;
    Ok(count > 0)
}

pub fn atualizar_agendamento(
    conn: &Connection,
    id: i32,
    novo_horario: Option<NaiveDateTime>,
    novo_servicos_ids: Option<Vec<i32>>,
    novo_preco: Option<f64>,
    concluido: Option<bool>,
) -> Result<()> {
    let mut updates = Vec::new();
    let mut params_vec: Vec<rusqlite::types::Value> = Vec::new();

    if let Some(horario) = novo_horario {
        updates.push("data_hora = ?");
        params_vec.push(rusqlite::types::Value::Integer(horario.and_utc().timestamp()));
    }
    if let Some(preco) = novo_preco {
        updates.push("preco = ?");
        params_vec.push(rusqlite::types::Value::Real(preco));
    }
    if let Some(conc) = concluido {
        updates.push("concluido = ?");
        params_vec.push(rusqlite::types::Value::Integer(conc as i64));
    }

    if !updates.is_empty() {
        let update_query = format!(
            "UPDATE agendamentos SET {} WHERE id = ?{}",
            updates.join(", "),
            params_vec.len() + 1
        );
        params_vec.push(rusqlite::types::Value::Integer(id as i64));
        let params_refs: Vec<&dyn ToSql> = params_vec.iter().map(|v| v as &dyn ToSql).collect();
        conn.execute(&update_query, params_refs.as_slice())?;
    }

    if let Some(servicos_ids) = novo_servicos_ids {
        // Delete existing service associations
        conn.execute("DELETE FROM agendamento_servicos WHERE agendamento_id = ?1", params![id])?;
        // Insert new service associations
        for servico_id in servicos_ids {
            conn.execute(
                "INSERT INTO agendamento_servicos (agendamento_id, servico_id)
                 VALUES (?1, ?2)",
                params![id, servico_id],
            )?;
        }
    }

    Ok(())
}

pub fn excluir_agendamento(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM agendamentos WHERE id = ?1", params![id])?;
    Ok(())
}

// =================================================================================
// 4. RELATÓRIO DE LUCRO
// =================================================================================

pub fn gerar_relatorio_lucro(conn: &Connection) -> Result<f64> {
    let mut stmt = conn.prepare(
        "SELECT data_hora, preco FROM agendamentos WHERE concluido = 1"
    )?;

    let agendamentos = stmt.query_map([], |row| {
        let ts: i64 = row.get(0)?;
        let preco: f64 = row.get(1)?;
        Ok((timestamp_para_naive(ts), preco))
    })?;

    let mut por_dia = HashMap::new();
    let mut por_semana = HashMap::new();
    let mut por_mes = HashMap::new();
    let mut total = 0.0;

    for resultado in agendamentos {
        let (data, preco) = resultado?;
        total += preco;

        let dia = data.date();
        let semana = format!("Semana {} de {}", data.iso_week().week(), data.year());
        let mes = format!("{}-{}", data.year(), data.month());

        por_dia.entry(dia).and_modify(|v: &mut (i32, f64)| {
            v.0 += 1;
            v.1 += preco;
        }).or_insert((1, preco));

        por_semana.entry(semana).and_modify(|v: &mut (i32, f64)| {
            v.0 += 1;
            v.1 += preco;
        }).or_insert((1, preco));

        por_mes.entry(mes).and_modify(|v: &mut (i32, f64)| {
            v.0 += 1;
            v.1 += preco;
        }).or_insert((1, preco));
    }

    println!("\n📅 Lucro por Dia:");
    for (dia, (qtd, total)) in por_dia.iter() {
        println!("  {} → {} serviços, R$ {:.2}", dia, qtd, total);
    }

    println!("\n🗓️ Lucro por Semana:");
    for (semana, (qtd, total)) in por_semana.iter() {
        println!("  {} → {} serviços, R$ {:.2}", semana, qtd, total);
    }

    println!("\n📆 Lucro por Mês:");
    for (mes, (qtd, total)) in por_mes.iter() {
        println!("  {} → {} serviços, R$ {:.2}", mes, qtd, total);
    }

    Ok(total)
}
/// Exclui um cliente do banco de dados.
/// Retorna erro se o cliente ainda possui agendamentos vinculados.
pub fn excluir_cliente(conn: &Connection, id: i32) -> Result<()> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM agendamentos WHERE cliente_id = ?1")?;
    let count: i32 = stmt.query_row(params![id], |row| row.get(0))?;
    if count > 0 {
        return Err(rusqlite::Error::ExecuteReturnedResults);
    }
    conn.execute("DELETE FROM clientes WHERE id = ?1", params![id])?;
    Ok(())
}
/// Lista todos os agendamentos para uma data específica.
pub fn listar_agendamentos_por_data(conn: &Connection, data: NaiveDateTime) -> Result<Vec<Agendamento>> {
    let inicio = data.date().and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
    let fim = data.date().and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp();

    let mut stmt = conn.prepare(
        "SELECT id, cliente_id, data_hora, preco, concluido 
         FROM agendamentos 
         WHERE data_hora BETWEEN ?1 AND ?2"
    )?;
    let agendamentos_iter = stmt.query_map(params![inicio, fim], |row| {
        Ok(Agendamento {
            id: row.get(0)?,
            cliente_id: row.get(1)?,
            servicos_ids: Vec::new(), // Initialize as empty
            data_hora: timestamp_para_naive(row.get::<_, i64>(2)?),
            preco: row.get(3)?,
            concluido: row.get(4)?,
        })
    })?;

    let mut agendamentos_com_servicos = Vec::new();
    for agendamento_result in agendamentos_iter {
        let mut agendamento = agendamento_result?;
        if let Some(agendamento_id) = agendamento.id {
            let mut stmt_servicos = conn.prepare("SELECT servico_id FROM agendamento_servicos WHERE agendamento_id = ?1")?;
            let servico_ids_iter = stmt_servicos.query_map(params![agendamento_id], |row| row.get(0))?;
            
            let mut servicos_ids = Vec::new();
            for servico_id_result in servico_ids_iter {
                servicos_ids.push(servico_id_result?);
            }
            agendamento.servicos_ids = servicos_ids;
        }
        agendamentos_com_servicos.push(agendamento);
    }
    Ok(agendamentos_com_servicos)
}
/// Lista todos os agendamentos de um cliente específico.
pub fn listar_agendamentos_por_cliente(conn: &Connection, cliente_id: i32) -> Result<Vec<Agendamento>> {
    let mut stmt = conn.prepare(
        "SELECT id, cliente_id, data_hora, preco, concluido 
         FROM agendamentos 
         WHERE cliente_id = ?1"
    )?;
    let agendamentos_iter = stmt.query_map(params![cliente_id], |row| {
        Ok(Agendamento {
            id: row.get(0)?,
            cliente_id: row.get(1)?,
            servicos_ids: Vec::new(), // Initialize as empty
            data_hora: timestamp_para_naive(row.get::<_, i64>(2)?),
            preco: row.get(3)?,
            concluido: row.get(4)?,
        })
    })?;

    let mut agendamentos_com_servicos = Vec::new();
    for agendamento_result in agendamentos_iter {
        let mut agendamento = agendamento_result?;
        if let Some(agendamento_id) = agendamento.id {
            let mut stmt_servicos = conn.prepare("SELECT servico_id FROM agendamento_servicos WHERE agendamento_id = ?1")?;
            let servico_ids_iter = stmt_servicos.query_map(params![agendamento_id], |row| row.get(0))?;
            
            let mut servicos_ids = Vec::new();
            for servico_id_result in servico_ids_iter {
                servicos_ids.push(servico_id_result?);
            }
            agendamento.servicos_ids = servicos_ids;
        }
        agendamentos_com_servicos.push(agendamento);
    }
    Ok(agendamentos_com_servicos)
}
// =================================================================================
// 5. TESTES DE CRUD (para validação interna)
// =================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDateTime;

    #[test]
    fn test_excluir_cliente_sem_agendamentos() {
        let conn = conectar_db().unwrap();
        criar_tabelas(&conn).unwrap();

        let mut cliente = Cliente::new("Teste Sem Agendamentos".into(), "5599999999999".into(), None);
        let id = salvar_cliente(&conn, &mut cliente).unwrap();
        let resultado = excluir_cliente(&conn, id);
        assert!(resultado.is_ok(), "Cliente sem agendamentos deveria ser excluído com sucesso");
    }

    #[test]
    fn test_excluir_cliente_com_agendamentos() {
        let conn = conectar_db().unwrap();
        criar_tabelas(&conn).unwrap();

        let mut cliente = Cliente::new("Teste Com Agendamentos".into(), "5588888888888".into(), None);
        let cliente_id = salvar_cliente(&conn, &mut cliente).unwrap();

        let horario = NaiveDateTime::parse_from_str("2025-12-01 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let agendamento = Agendamento {
            id: None,
            cliente_id,
            servicos_ids: vec![1], // Assuming service with ID 1 exists
            data_hora: horario,
            preco: 30.0,
            concluido: false,
        };
        salvar_agendamento(&conn, &agendamento).unwrap();

        let resultado = excluir_cliente(&conn, cliente_id);
        assert!(resultado.is_err(), "Cliente com agendamentos não deveria ser excluído");
    }

    #[test]
    fn test_listar_agendamentos_por_data() {
        let conn = conectar_db().unwrap();
        criar_tabelas(&conn).unwrap();

        // Insert a dummy service for the test
        let servico_teste = Servico { id: None, nome: "Corte Diário".into(), preco: 40.0 };
        let servico_id = salvar_servico(&conn, &servico_teste).unwrap();

        // Insert a dummy client for the test
        let mut cliente = Cliente::new("Cliente Diário".into(), "5511987654321".into(), None);
        let cliente_id = salvar_cliente(&conn, &mut cliente).unwrap();

        let data_agendamento = NaiveDateTime::parse_from_str("2025-12-01 11:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let agendamento = Agendamento {
            id: None,
            cliente_id,
            servicos_ids: vec![servico_id],
            data_hora: data_agendamento,
            preco: 40.0,
            concluido: false,
        };
        salvar_agendamento(&conn, &agendamento).unwrap();

        let data_para_busca = NaiveDateTime::parse_from_str("2025-12-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let agendamentos = listar_agendamentos_por_data(&conn, data_para_busca).unwrap();
        
        assert!(!agendamentos.is_empty(), "Deveria retornar pelo menos um agendamento para a data");
        assert_eq!(agendamentos[0].servicos_ids, vec![servico_id]);
        println!("Agendamentos encontrados para a data: {}", agendamentos.len());
    }

    #[test]
    fn test_listar_agendamentos_por_cliente() {
        let conn = conectar_db().unwrap();
        criar_tabelas(&conn).unwrap();

        // Insert a dummy service for the test
        let servico_teste = Servico { id: None, nome: "Corte Teste".into(), preco: 50.0 };
        let servico_id = salvar_servico(&conn, &servico_teste).unwrap();

        let mut cliente = Cliente::new("Teste Listagem Cliente".into(), "5577777777777".into(), None);
        let cliente_id = salvar_cliente(&conn, &mut cliente).unwrap();

        let horario = NaiveDateTime::parse_from_str("2025-12-02 14:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let agendamento = Agendamento {
            id: None,
            cliente_id,
            servicos_ids: vec![servico_id],
            data_hora: horario,
            preco: 50.0,
            concluido: true,
        };
        salvar_agendamento(&conn, &agendamento).unwrap();

        let agendamentos = listar_agendamentos_por_cliente(&conn, cliente_id).unwrap();
        assert!(!agendamentos.is_empty(), "Deveria retornar pelo menos um agendamento");
        assert_eq!(agendamentos[0].servicos_ids, vec![servico_id]);
    }
}
pub fn salvar_servico(conn: &Connection, servico: &Servico) -> Result<i32> {
    conn.execute(
        "INSERT INTO servicos (nome, preco) VALUES (?1, ?2)",
        params![servico.nome, servico.preco],
    )?;
    Ok(conn.last_insert_rowid() as i32)
}

pub fn listar_servicos(conn: &Connection) -> Result<Vec<Servico>> {
    let mut stmt = conn.prepare("SELECT id, nome, preco FROM servicos")?;
    let servicos = stmt.query_map([], |row| {
        Ok(Servico {
            id: row.get(0)?,
            nome: row.get(1)?,
            preco: row.get(2)?,
        })
    })?
    .filter_map(Result::ok)
    .collect();
    Ok(servicos)
}

pub fn excluir_servico(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM servicos WHERE id = ?1", params![id])?;
    Ok(())
}
/// Retorna os nomes dos serviços vinculados a um agendamento
pub fn buscar_nomes_servicos(conn: &Connection, ids: &[i32]) -> Result<Vec<String>> {
    let mut nomes = Vec::new();
    for id in ids {
        let nome: String = conn.query_row(
            "SELECT nome FROM servicos WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        nomes.push(nome);
    }
    Ok(nomes)
}
