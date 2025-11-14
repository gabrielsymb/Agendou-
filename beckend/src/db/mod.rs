// --- Módulos & Bibliotecas ---
use rusqlite::{Connection, Result, params, OpenFlags};
use chrono::{NaiveDateTime, Datelike, DateTime, Utc};
use std::collections::HashMap;
use crate::models::{Cliente, Agendamento, Servico};
use rusqlite::ToSql;
use std::{fs, path::Path, time::Duration};


// Caminho do banco de dados (padrão) — pode ser sobrescrito pela variável de ambiente APP_DB_PATH
// Agora apontamos por padrão para a pasta `src/bd` conforme solicitado.
const DB_PATH: &str = "src/bd/Banco.db";

// =================================================================================
// 1. INFRAESTRUTURA
// =================================================================================

pub fn conectar_db() -> Result<Connection> {
    // Allow overriding DB path via env var for flexibility in dev/production
    let db_path = std::env::var("APP_DB_PATH").unwrap_or_else(|_| DB_PATH.to_string());
    // Resolve absolute path for logging/debugging
    // canonicalize pode falhar se o arquivo ainda não existir; use PathBuf direto como fallback
    let abs_path = match std::fs::canonicalize(&db_path) {
        Ok(p) => p,
        Err(_) => std::path::PathBuf::from(&db_path),
    };
    println!("[DB] Abrindo arquivo de banco de dados em: {}", abs_path.display());
    // Garantir que o diretório pai exista (cria se necessário)
    let path = Path::new(&db_path);
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            if let Err(e) = fs::create_dir_all(parent) {
                eprintln!("[DB] Falha ao criar diretório {}: {}", parent.display(), e);
                // Tentaremos abrir mesmo assim; Connection::open_with_flags retornará erro apropriado se necessário
            }
        }
    }

    // Abrir em read-write e criar se não existir
    let conn = Connection::open_with_flags(&db_path, OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE)?;
    // Ajustar timeout para evitar falhas se outro processo estiver acessando momentaneamente
    conn.busy_timeout(Duration::from_secs(5))?;
    Ok(conn)
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
    criar_tabela_work_schedule(conn)?;
    Ok(())
}

pub fn criar_tabela_work_schedule(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS work_windows (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            weekday INTEGER NOT NULL,
            start_time TEXT NOT NULL,
            end_time TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

/// Retorna as janelas de trabalho (start,end) para a data informada.
pub fn get_work_windows_for_date(conn: &Connection, date: chrono::NaiveDate) -> Result<Vec<(chrono::NaiveTime, chrono::NaiveTime)>> {
    let weekday = date.weekday().num_days_from_monday() as i32; // 0 = Monday
    let mut stmt = conn.prepare("SELECT start_time, end_time FROM work_windows WHERE weekday = ?1")?;
    let rows = stmt.query_map(params![weekday], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)))?;

    let mut windows = Vec::new();
    for r in rows {
        let (s,e) = r?;
        if let (Ok(st), Ok(en)) = (chrono::NaiveTime::parse_from_str(&s, "%H:%M"), chrono::NaiveTime::parse_from_str(&e, "%H:%M")) {
            windows.push((st,en));
        }
    }
    Ok(windows)
}

/// Lista todas as work_windows (id, weekday, start_time, end_time)
pub fn listar_work_windows(conn: &Connection) -> Result<Vec<(i32, i32, String, String)>> {
    let mut stmt = conn.prepare("SELECT id, weekday, start_time, end_time FROM work_windows ORDER BY weekday, start_time")?;
    let rows = stmt.query_map([], |row| Ok((row.get::<_, i32>(0)?, row.get::<_, i32>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?)))?;
    let mut out = Vec::new();
    for r in rows { out.push(r?); }
    Ok(out)
}

pub fn salvar_work_window(conn: &Connection, weekday: i32, start_time: &str, end_time: &str) -> Result<i64> {
    conn.execute("INSERT INTO work_windows (weekday, start_time, end_time) VALUES (?1, ?2, ?3)", params![weekday, start_time, end_time])?;
    Ok(conn.last_insert_rowid())
}

pub fn criar_tabela_servicos(conn: &Connection) -> Result<()> {
    // Create the table ensuring the `duracao_min` column exists with a default value.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS servicos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            preco REAL NOT NULL,
            duracao_min INTEGER NOT NULL DEFAULT 30
        )",
        [],
    )?;

    // If the table existed previously without `duracao_min`, add the column.
    let mut has_duracao = false;
    let mut stmt = conn.prepare("PRAGMA table_info(servicos)")?;
    let cols = stmt.query_map([], |row| Ok(row.get::<_, String>(1)?))?;
    for col in cols {
        if col? == "duracao_min" {
            has_duracao = true;
            break;
        }
    }
    if !has_duracao {
        // Add column with a sensible default to avoid NOT NULL violations.
        conn.execute("ALTER TABLE servicos ADD COLUMN duracao_min INTEGER NOT NULL DEFAULT 30", [])?;
    }

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

/// Lista clientes filtrando por nome (LIKE) com limite.
pub fn listar_clientes_search(conn: &Connection, search: &str, limit: i32) -> Result<Vec<Cliente>> {
    let pattern = format!("%{}%", search.replace('%', "\\%"));
    let mut stmt = conn.prepare("SELECT id, nome, telefone, email FROM clientes WHERE nome LIKE ?1 ORDER BY nome LIMIT ?2")?;
    let clientes = stmt.query_map(params![pattern, limit], |row| {
        Ok(Cliente {
            id: row.get(0)?,
            nome: row.get(1)?,
            telefone: row.get(2)?,
            email: row.get(3)?,
        })
    })?.filter_map(Result::ok).collect();
    Ok(clientes)
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

#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(dead_code)]
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
    let servico_teste = Servico { id: None, nome: "Corte Diário".into(), preco: 40.0, duracao_min: 30 };
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
    let servico_teste = Servico { id: None, nome: "Corte Teste".into(), preco: 50.0, duracao_min: 30 };
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

    #[test]
    fn test_listar_clientes_search() {
        let conn = conectar_db().unwrap();
        criar_tabelas(&conn).unwrap();

        // Insert clients
        let mut c1 = Cliente::new("João Silva".into(), "5511999999999".into(), None);
        let mut c2 = Cliente::new("Maria Joaquina".into(), "5511888888888".into(), None);
        let _ = salvar_cliente(&conn, &mut c1).unwrap();
        let _ = salvar_cliente(&conn, &mut c2).unwrap();

        let results = listar_clientes_search(&conn, "joa", 10).unwrap();
        assert!(results.len() >= 2, "Esperado encontrar pelo menos 2 clientes com 'joa'");
    }
}
pub fn salvar_servico(conn: &Connection, servico: &Servico) -> Result<i32> {
    match servico.id {
        Some(id) => {
            // Update existing service
            conn.execute(
                "UPDATE servicos SET nome = ?1, preco = ?2, duracao_min = ?3 WHERE id = ?4",
                params![servico.nome, servico.preco, servico.duracao_min, id],
            )?;
            Ok(id)
        }
        None => {
            // Insert new service
            conn.execute(
                "INSERT INTO servicos (nome, preco, duracao_min) VALUES (?1, ?2, ?3)",
                params![servico.nome, servico.preco, servico.duracao_min],
            )?;
            Ok(conn.last_insert_rowid() as i32)
        }
    }
}

pub fn listar_servicos(conn: &Connection) -> Result<Vec<Servico>> {
    let mut stmt = conn.prepare("SELECT id, nome, preco, duracao_min FROM servicos")?;
    let servicos = stmt.query_map([], |row| {
        Ok(Servico {
            id: row.get(0)?,
            nome: row.get(1)?,
            preco: row.get(2)?,
            duracao_min: row.get(3)?,
        })
    })?
    .filter_map(Result::ok)
    .collect();
    Ok(servicos)
}

/// Lista serviços filtrando por nome (LIKE) com limite.
pub fn listar_servicos_search(conn: &Connection, search: &str, limit: i32) -> Result<Vec<Servico>> {
    let pattern = format!("%{}%", search.replace('%', "\\%"));
    let mut stmt = conn.prepare("SELECT id, nome, preco, duracao_min FROM servicos WHERE nome LIKE ?1 ORDER BY nome LIMIT ?2")?;
    let servicos = stmt.query_map(params![pattern, limit], |row| {
        Ok(Servico {
            id: row.get(0)?,
            nome: row.get(1)?,
            preco: row.get(2)?,
            duracao_min: row.get(3)?,
        })
    })?.filter_map(Result::ok).collect();
    Ok(servicos)
}

pub fn buscar_servico_por_id(conn: &Connection, id: i32) -> Result<Option<Servico>> {
    let mut stmt = conn.prepare("SELECT id, nome, preco, duracao_min FROM servicos WHERE id = ?1")?;
    let mut rows = stmt.query(params![id])?;

    if let Some(row) = rows.next()? {
        Ok(Some(Servico {
            id: row.get(0)?,
            nome: row.get(1)?,
            preco: row.get(2)?,
            duracao_min: row.get(3)?,
        }))
    } else {
        Ok(None)
    }
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

/// Retorna slots de disponibilidade para uma data específica.
/// Parâmetros:
/// - data_str: YYYY-MM-DD
/// - duracao_min: duração total necessária em minutos
/// - buffer_min: minutos de buffer antes/depois (padrão 15)
/// - granularity_min: granularidade dos slots em minutos (padrão 15)
pub fn calcular_disponibilidade(conn: &Connection, data_str: &str, duracao_min: i64, buffer_min: i64, granularity_min: i64) -> Result<Vec<String>> {
    use chrono::{NaiveDate, NaiveTime, Duration as ChronoDuration};

    // Parse da data
    let date = NaiveDate::parse_from_str(data_str, "%Y-%m-%d").map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?;

    // Use work windows configured in DB for this date
    // Converter durações (precisamos dessas antes de manipular janelas)
    let dur = ChronoDuration::minutes(duracao_min);
    let buf = ChronoDuration::minutes(buffer_min);
    let gran = ChronoDuration::minutes(granularity_min);

    let windows = get_work_windows_for_date(conn, date)?;
    if windows.is_empty() {
        // fallback to default window
        let work_start = NaiveTime::from_hms_opt(8, 0, 0).unwrap();
        let work_end = NaiveTime::from_hms_opt(18, 0, 0).unwrap();
        // single window
        // We'll construct windows vec with this default
        // but re-use existing logic below that iterates windows
        // to keep code paths consistent.
        // windows = vec![(work_start, work_end)]; // can't reassign immutable
        let mut def = Vec::new();
        def.push((work_start, work_end));
    // use default window contained in `def` for local computation (no need to shadow `windows`)
        // proceed
        // compute inicio_ts/fim_ts for entire day range using work_start and work_end
        let start_dt = date.and_time(work_start);
        let end_dt = date.and_time(work_end);
        let inicio_ts = start_dt.and_utc().timestamp();
        let fim_ts = end_dt.and_utc().timestamp();
        // fetch agendamentos within whole day window
        let mut stmt = conn.prepare(
            "SELECT data_hora, id FROM agendamentos WHERE data_hora BETWEEN ?1 AND ?2 AND concluido = 0"
        )?;
        let rows = stmt.query_map(params![inicio_ts, fim_ts], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i32>(1)?)))?;

        let mut ocupados: Vec<(i64,i64)> = Vec::new();
        for r in rows {
            let (ts, _id) = r?;
            let mut stmt_s = conn.prepare("SELECT s.duracao_min FROM servicos s JOIN agendamento_servicos a ON s.id = a.servico_id WHERE a.agendamento_id = ?1")?;
            let dur_iter = stmt_s.query_map(params![_id], |row| row.get::<_, i32>(0))?;
            let mut total_min = 0i64;
            for dm in dur_iter { total_min += dm? as i64; }
            if total_min == 0 { total_min = 30; }

            let ag_start = timestamp_para_naive(ts).and_utc().timestamp();
            let ag_end = (timestamp_para_naive(ts) + ChronoDuration::minutes(total_min)).and_utc().timestamp();
            let occ_start = ag_start;
            let occ_end = ag_end + buf.num_seconds();
            ocupados.push((occ_start, occ_end));
        }

    // now iterate the single default window
    let mut slots: Vec<String> = Vec::new();
    let mut cursor = start_dt;
    while cursor + dur <= end_dt {
            let slot_start_ts = cursor.and_utc().timestamp();
            let slot_end_ts = (cursor + dur).and_utc().timestamp();
            let need_start = slot_start_ts;
            let need_end = slot_end_ts + buf.num_seconds();
            let mut conflict = false;
            for (occ_s, occ_e) in &ocupados {
                if !(need_end <= *occ_s || need_start >= *occ_e) {
                    conflict = true;
                    break;
                }
            }
            if !conflict {
                slots.push(cursor.format("%Y-%m-%dT%H:%M:%S").to_string());
            }
        cursor = cursor + gran;
        }
        return Ok(slots);
    }
    // Converter agendamentos do dia para intervalos ocupados (com buffer após fim)
    // Converter agendamentos do dia para intervalos ocupados (com buffer após fim)
    // Buscaremos todos agendamentos do dia inteiro e depois os compararemos com cada window
    let day_start = date.and_time(chrono::NaiveTime::from_hms_opt(0,0,0).unwrap());
    let day_end = date.and_time(chrono::NaiveTime::from_hms_opt(23,59,59).unwrap());
    let inicio_ts = day_start.and_utc().timestamp();
    let fim_ts = day_end.and_utc().timestamp();

    let mut stmt = conn.prepare(
        "SELECT data_hora, id FROM agendamentos WHERE data_hora BETWEEN ?1 AND ?2 AND concluido = 0"
    )?;
    let rows = stmt.query_map(params![inicio_ts, fim_ts], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i32>(1)?)))?;

    let mut ocupados: Vec<(i64,i64)> = Vec::new();
    for r in rows {
        let (ts, _id) = r?;
        let mut stmt_s = conn.prepare("SELECT s.duracao_min FROM servicos s JOIN agendamento_servicos a ON s.id = a.servico_id WHERE a.agendamento_id = ?1")?;
        let dur_iter = stmt_s.query_map(params![_id], |row| row.get::<_, i32>(0))?;
        let mut total_min = 0i64;
        for dm in dur_iter { total_min += dm? as i64; }
        if total_min == 0 { total_min = 30; }

        let ag_start = timestamp_para_naive(ts).and_utc().timestamp();
        let ag_end = (timestamp_para_naive(ts) + ChronoDuration::minutes(total_min)).and_utc().timestamp();
        let occ_start = ag_start;
        let occ_end = ag_end + buf.num_seconds();
        ocupados.push((occ_start, occ_end));
    }

    // Iterate each configured window and generate slots inside it
    let mut slots: Vec<String> = Vec::new();
    for (wstart, wend) in windows {
        let start_dt = date.and_time(wstart);
        let end_dt = date.and_time(wend);
        let mut cursor = start_dt;
        while cursor + dur <= end_dt {
            let slot_start_ts = cursor.and_utc().timestamp();
            let slot_end_ts = (cursor + dur).and_utc().timestamp();
            let need_start = slot_start_ts;
            let need_end = slot_end_ts + buf.num_seconds();
            let mut conflict = false;
            for (occ_s, occ_e) in &ocupados {
                if !(need_end <= *occ_s || need_start >= *occ_e) {
                    conflict = true;
                    break;
                }
            }
            if !conflict {
                slots.push(cursor.format("%Y-%m-%dT%H:%M:%S").to_string());
            }
            cursor = cursor + gran;
        }
    }

    Ok(slots)
}
