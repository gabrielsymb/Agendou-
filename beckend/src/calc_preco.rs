use crate::db::listar_agendamentos;
use chrono::{Datelike, Local};
use rusqlite::Connection;

/// Calcula o lucro total de todos os agendamentos concluídos
pub fn calcular_lucro_total(conn: &Connection) -> rusqlite::Result<f64> {
    let agendamentos = listar_agendamentos(conn)?;
    let total: f64 = agendamentos
        .into_iter()
        .filter(|a| a.concluido)
        .map(|a| a.preco)
        .sum();
    Ok(if total.abs() < 0.005 { 0.0 } else { total })
}

/// Calcula o lucro do mês atual
pub fn calcular_lucro_mensal(conn: &Connection) -> rusqlite::Result<f64> {
    let hoje = Local::now().naive_local();
    let mes = hoje.month();
    let ano = hoje.year();

    let agendamentos = listar_agendamentos(conn)?;
    let total: f64 = agendamentos
        .into_iter()
        .filter(|a| a.concluido && a.data_hora.month() == mes && a.data_hora.year() == ano)
        .map(|a| a.preco)
        .sum();
    Ok(if total.abs() < 0.005 { 0.0 } else { total })
}

/// Calcula o lucro do dia atual
pub fn calcular_lucro_do_dia(conn: &Connection) -> rusqlite::Result<f64> {
    let hoje = Local::now().naive_local().date();
    let agendamentos = listar_agendamentos(conn)?;
    let total: f64 = agendamentos
        .into_iter()
        .filter(|a| a.concluido && a.data_hora.date() == hoje)
        .map(|a| a.preco)
        .sum();
    Ok(if total.abs() < 0.005 { 0.0 } else { total })
}