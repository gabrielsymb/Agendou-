use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

use crate::models::Agendamento;
use crate::db;
use serde::Deserialize;
use serde_json::Value as JsonValue;
use chrono::{NaiveDateTime, DateTime, Utc, TimeZone};

type Db = Arc<Mutex<Connection>>;

#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

pub async fn listar_agendamentos_api(State(conn): State<Db>) -> Json<Vec<Agendamento>> {
    let conn = conn.lock().unwrap();
    let agendamentos = db::listar_agendamentos(&conn).unwrap_or_else(|e| {
        eprintln!("Erro ao listar agendamentos: {}", e);
        vec![]
    });
    Json(agendamentos)
}

pub async fn obter_agendamento_api(Path(id): Path<i32>, State(conn): State<Db>) -> Json<Option<Agendamento>> {
    let conn = conn.lock().unwrap();
    match db::buscar_agendamento_por_id(&conn, id) {
        Ok(ag) => Json(Some(ag)),
        Err(e) => {
            eprintln!("Agendamento não encontrado ou erro: {}", e);
            Json(None)
        }
    }
}

#[allow(dead_code)]
pub async fn criar_agendamento_api(State(conn): State<Db>, Json(agendamento): Json<Agendamento>) -> (StatusCode, Json<ApiResponse<Agendamento>>) {
    // NOTE: The frontend may send data_hora as an ISO string (e.g. new Date().toISOString())
    // or as a numeric timestamp. To avoid automatic 422 from the JSON extractor when the
    // backend expects a chrono::NaiveDateTime, we accept a lightweight DTO and parse here.
    // (The DTO handling is done below in criar_agendamento_api_incoming)
    let conn = conn.lock().unwrap();
    match db::salvar_agendamento(&conn, &agendamento) {
        Ok(id) => {
            let mut saved = agendamento.clone();
            saved.id = Some(id);
            let response = ApiResponse { success: true, message: "Agendamento criado com sucesso!".to_string(), data: Some(saved) };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            eprintln!("Erro ao criar agendamento: {}", e);
            let response: ApiResponse<Agendamento> = ApiResponse { success: false, message: format!("Erro ao criar agendamento: {}", e), data: None };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

// New handler to accept flexible incoming payloads (string timestamp or numeric)
#[derive(Deserialize)]
pub struct IncomingAgendamento {
    pub id: Option<i32>,
    pub cliente_id: i32,
    pub servicos_ids: Vec<i32>,
    pub data_hora: JsonValue,
    pub preco: f64,
    pub concluido: bool,
}

fn parse_data_hora(value: &JsonValue) -> Result<NaiveDateTime, String> {
    match value {
        JsonValue::Number(n) => {
            if let Some(ts) = n.as_i64() {
                // treat as seconds since epoch
                // Use Utc.timestamp_opt(...).single() to obtain a DateTime<Utc>
                if let Some(dt) = Utc.timestamp_opt(ts, 0).single() {
                    Ok(dt.naive_local())
                } else {
                    Err("Invalid numeric timestamp".to_string())
                }
            } else {
                Err("Invalid numeric timestamp".to_string())
            }
        }
        JsonValue::String(s) => {
            // try rfc3339 parse
            match DateTime::parse_from_rfc3339(s) {
                Ok(dt) => Ok(dt.naive_local()),
                Err(_) => {
                    // try common naive format without timezone
                    match NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                        Ok(ndt) => Ok(ndt),
                        Err(e) => Err(format!("Failed to parse date string: {}", e)),
                    }
                }
            }
        }
        _ => Err("Unsupported data_hora format".to_string()),
    }
}

pub async fn criar_agendamento_api_incoming(State(conn): State<Db>, Json(incoming): Json<IncomingAgendamento>) -> (StatusCode, Json<ApiResponse<Agendamento>>) {
    let conn = conn.lock().unwrap();

    let parsed = match parse_data_hora(&incoming.data_hora) {
        Ok(dt) => dt,
        Err(msg) => {
            let response: ApiResponse<Agendamento> = ApiResponse { success: false, message: format!("Invalid data_hora: {}", msg), data: None };
            return (StatusCode::UNPROCESSABLE_ENTITY, Json(response));
        }
    };

    let agendamento = Agendamento {
        id: incoming.id,
        cliente_id: incoming.cliente_id,
        servicos_ids: incoming.servicos_ids,
        data_hora: parsed,
        preco: incoming.preco,
        concluido: incoming.concluido,
    };

    match db::salvar_agendamento(&conn, &agendamento) {
        Ok(id) => {
            let mut saved = agendamento.clone();
            saved.id = Some(id);
            let response = ApiResponse { success: true, message: "Agendamento criado com sucesso!".to_string(), data: Some(saved) };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            eprintln!("Erro ao criar agendamento: {}", e);
            let response: ApiResponse<Agendamento> = ApiResponse { success: false, message: format!("Erro ao criar agendamento: {}", e), data: None };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn atualizar_agendamento_api(Path(id): Path<i32>, State(conn): State<Db>, Json(agendamento): Json<Agendamento>) -> (StatusCode, Json<ApiResponse<Agendamento>>) {
    let conn = conn.lock().unwrap();

    let res = db::atualizar_agendamento(
        &conn,
        id,
        Some(agendamento.data_hora),
        Some(agendamento.servicos_ids.clone()),
        Some(agendamento.preco),
        Some(agendamento.concluido),
    );

    if let Err(e) = res {
        eprintln!("Erro ao atualizar agendamento: {}", e);
        let response: ApiResponse<Agendamento> = ApiResponse { success: false, message: format!("Erro ao atualizar agendamento: {}", e), data: None };
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
    }

    match db::buscar_agendamento_por_id(&conn, id) {
        Ok(updated) => {
            let response = ApiResponse { success: true, message: "Agendamento atualizado com sucesso!".to_string(), data: Some(updated) };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            eprintln!("Erro ao buscar agendamento atualizado: {}", e);
            let response: ApiResponse<Agendamento> = ApiResponse { success: false, message: format!("Agendamento atualizado porém falha ao recuperá-lo: {}", e), data: None };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn excluir_agendamento_api(Path(id): Path<i32>, State(conn): State<Db>) -> (StatusCode, Json<ApiResponse<()>>) {
    let conn = conn.lock().unwrap();
    match db::excluir_agendamento(&conn, id) {
        Ok(_) => (StatusCode::OK, Json(ApiResponse { success: true, message: "Agendamento excluído com sucesso!".to_string(), data: None })),
        Err(e) => {
            eprintln!("Erro ao excluir agendamento: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ApiResponse { success: false, message: format!("Erro ao excluir agendamento: {}", e), data: None }))
        }
    }
}
