#![allow(dead_code)]

use chrono::{NaiveDateTime, Utc, Datelike};
use rusqlite::{Connection, Result, params};
use sha2::{Sha256, Digest};

/// Verifica se a licença do cliente está válida (data >= hoje).
pub fn verificar_licenca_sistema(conn: &Connection) -> Result<bool> {
    let hoje = Utc::now().timestamp();
    let validade: Option<i64> = conn.query_row(
        "SELECT validade_licenca FROM sistema WHERE id = 1",
        [],
        |row| row.get(0),
    )?;
    Ok(validade.unwrap_or(0) >= hoje)
}

/// Atualiza a validade da licença do cliente.
pub fn atualizar_licenca(conn: &Connection, cliente_id: i32, nova_data: NaiveDateTime) -> Result<()> {
    let timestamp = nova_data.and_utc().timestamp();
    conn.execute(
        "UPDATE clientes SET validade_licenca = ?1 WHERE id = ?2",
        params![timestamp, cliente_id],
    )?;
    Ok(())
}

/// Gera código semanal baseado em chave secreta + semana atual.
pub fn gerar_codigo_semanal(chave: &str) -> String {
    let semana = Utc::now().iso_week().week();
    let base = format!("{}-WEEK{}", chave, semana);
    let mut hasher = Sha256::new();
    hasher.update(base);
    format!("{:x}", hasher.finalize())
}

/// Gera código mensal baseado em chave secreta + mês atual.
pub fn gerar_codigo_mensal(chave: &str) -> String {
    let mes = Utc::now().month();
    let base = format!("{}-MONTH{}", chave, mes);
    let mut hasher = Sha256::new();
    hasher.update(base);
    format!("{:x}", hasher.finalize())
}

/// Valida se o código digitado é válido para semana ou mês atual.
pub fn validar_codigo(codigo_digitado: &str, chave: &str) -> Option<NaiveDateTime> {
    let semanal = gerar_codigo_semanal(chave);
    let mensal = gerar_codigo_mensal(chave);
    let hoje = Utc::now().naive_utc();

    if codigo_digitado == semanal {
        Some(hoje + chrono::Duration::days(7))
    } else if codigo_digitado == mensal {
        Some(hoje + chrono::Duration::days(30))
    } else {
        None
    }
}