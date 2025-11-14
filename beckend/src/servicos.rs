use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    Json,
};
use std::sync::{Arc, Mutex};
use rusqlite::Connection;

use crate::models::Servico;
use crate::db;

type Db = Arc<Mutex<Connection>>;

#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}

#[allow(dead_code)]
pub async fn listar_servicos(State(conn): State<Db>) -> Json<Vec<Servico>> {
    let conn = conn.lock().unwrap();
    let servicos = db::listar_servicos(&conn).unwrap_or_else(|e| {
        eprintln!("Erro ao listar serviços: {}", e);
        vec![]
    });
    Json(servicos)
}


#[derive(serde::Deserialize)]
pub struct ServicosQuery {
    search: Option<String>,
    limit: Option<i32>,
}

pub async fn listar_servicos_query(Query(q): Query<ServicosQuery>, State(conn): State<Db>) -> Json<Vec<Servico>> {
    let conn = conn.lock().unwrap();
    if let Some(search) = q.search {
        let limit = q.limit.unwrap_or(15);
        let servicos = db::listar_servicos_search(&conn, &search, limit).unwrap_or_else(|e| {
            eprintln!("Erro ao buscar serviços: {}", e);
            vec![]
        });
        return Json(servicos);
    }
    let servicos = db::listar_servicos(&conn).unwrap_or_else(|e| {
        eprintln!("Erro ao listar serviços: {}", e);
        vec![]
    });
    Json(servicos)
}

pub async fn criar_servico(State(conn): State<Db>, Json(mut servico): Json<Servico>) -> (StatusCode, Json<ApiResponse<Servico>>) {
    let conn = conn.lock().unwrap();
    match db::salvar_servico(&conn, &mut servico) {
        Ok(_) => {
            let response = ApiResponse {
                success: true,
                message: "Serviço cadastrado com sucesso!".to_string(),
                data: Some(servico),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            eprintln!("Erro ao criar serviço: {}", e);
            let response = ApiResponse {
                success: false,
                message: format!("Erro ao cadastrar serviço: {}", e),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn obter_servico(Path(id): Path<i32>, State(conn): State<Db>) -> Json<Option<Servico>> {
    let conn = conn.lock().unwrap();
    let servico = db::buscar_servico_por_id(&conn, id).unwrap_or(None);
    Json(servico)
}

pub async fn atualizar_servico(Path(id): Path<i32>, State(conn): State<Db>, Json(mut servico): Json<Servico>) -> (StatusCode, Json<ApiResponse<Servico>>) {
    servico.id = Some(id);
    let conn = conn.lock().unwrap();
    match db::salvar_servico(&conn, &mut servico) {
        Ok(_) => {
            let response = ApiResponse {
                success: true,
                message: "Serviço atualizado com sucesso!".to_string(),
                data: Some(servico),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            eprintln!("Erro ao atualizar serviço: {}", e);
            let response = ApiResponse {
                success: false,
                message: format!("Erro ao atualizar serviço: {}", e),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

pub async fn excluir_servico(Path(id): Path<i32>, State(conn): State<Db>) -> (StatusCode, Json<ApiResponse<()>>) {
    let conn = conn.lock().unwrap();
    match db::excluir_servico(&conn, id) {
        Ok(_) => (StatusCode::OK, Json(ApiResponse { success: true, message: "Serviço excluído com sucesso!".to_string(), data: None })),
        Err(e) => {
            eprintln!("Erro ao excluir serviço: {}", e);
            let response = ApiResponse { success: false, message: format!("Erro ao excluir serviço: {}", e), data: None };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}