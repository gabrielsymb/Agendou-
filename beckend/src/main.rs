
use axum::{
    extract::{Path, State, Query},
    http::{Method, StatusCode},
    routing::get,
    Json, Router,};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::env;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod models;
mod menu;
mod licenca;
mod calc_preco;
mod servicos;
mod agendamentos;
 
use db::{buscar_cliente_por_id, excluir_cliente, salvar_cliente};
use models::Cliente;
use serde::Serialize;
 
type Db = Arc<Mutex<Connection>>;

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
}
 
#[derive(serde::Deserialize)]
struct ClientesQuery {
    search: Option<String>,
    limit: Option<i32>,
}

async fn listar_clientes_api(Query(q): Query<ClientesQuery>, State(conn): State<Db>) -> Json<Vec<Cliente>> {
    let conn = conn.lock().unwrap();
    if let Some(search) = q.search {
        let limit = q.limit.unwrap_or(15);
        let clientes = db::listar_clientes_search(&conn, &search, limit).unwrap_or_else(|e| {
            eprintln!("Erro ao buscar clientes: {}", e);
            vec![]
        });
        return Json(clientes);
    }

    let clientes = db::listar_clientes(&conn).unwrap_or_else(|e| {
        eprintln!("Erro ao listar clientes: {}", e);
        vec![]
    });
    Json(clientes)
}
 
async fn criar_cliente(State(conn): State<Db>, Json(mut cliente): Json<Cliente>) -> (StatusCode, Json<ApiResponse<Cliente>>) {
    let conn = conn.lock().unwrap();
    match salvar_cliente(&conn, &mut cliente) {
        Ok(_) => {
            let response = ApiResponse {
                success: true,
                message: "Cliente cadastrado com sucesso!".to_string(),
                data: Some(cliente),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            eprintln!("Erro ao criar cliente: {}", e);
            let response = ApiResponse {
                success: false,
                message: format!("Erro ao cadastrar cliente: {}", e),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

async fn obter_cliente(Path(id): Path<i32>, State(conn): State<Db>) -> Json<Option<Cliente>> {
    let conn = conn.lock().unwrap();
    let cliente = buscar_cliente_por_id(&conn, id).unwrap_or(None);
    Json(cliente)
}

async fn atualizar_cliente_api(Path(id): Path<i32>, State(conn): State<Db>, Json(mut cliente): Json<Cliente>) -> (StatusCode, Json<ApiResponse<Cliente>>) {
    cliente.id = Some(id);
    let conn = conn.lock().unwrap();
    match salvar_cliente(&conn, &mut cliente) {
        Ok(_) => {
            let response = ApiResponse {
                success: true,
                message: "Cliente atualizado com sucesso!".to_string(),
                data: Some(cliente),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            eprintln!("Erro ao atualizar cliente: {}", e);
            let response = ApiResponse {
                success: false,
                message: format!("Erro ao atualizar cliente: {}", e),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

async fn deletar_cliente_api(Path(id): Path<i32>, State(conn): State<Db>) -> (StatusCode, Json<ApiResponse<()>>) {
    let conn = conn.lock().unwrap();
    let result = excluir_cliente(&conn, id);
 
    match result {
        Ok(_) => (StatusCode::OK, Json(ApiResponse { success: true, message: "Cliente excluído com sucesso!".to_string(), data: None })),
        Err(e) => {
            if let Some(app_err) = e.sqlite_error() {
                if app_err.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_FOREIGNKEY {
                    let response = ApiResponse { success: false, message: "Não é possível excluir o cliente pois ele possui agendamentos.".to_string(), data: None };
                    return (StatusCode::CONFLICT, Json(response));
                }
            }
            eprintln!("Erro ao excluir cliente: {}", e);
            let response = ApiResponse { success: false, message: format!("Erro ao excluir cliente: {}", e), data: None };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

fn main() {
    // Pega os argumentos passados na linha de comando.
    let args: Vec<String> = env::args().collect();

    // O primeiro argumento (índice 1) é o que nos interessa.
    // Se nenhum argumento for passado, o padrão será "server".
    // Aceitamos tanto `cli`/`server` quanto `-cli`/`--cli` etc.
    let command_raw = args.get(1).map(|s| s.as_str()).unwrap_or("server");
    let command = command_raw.trim_start_matches('-');

    match command {
        "cli" => {
            println!("Iniciando modo CLI...");
            if let Err(e) = menu::menu_principal() {
                eprintln!("Erro ao executar o menu: {}", e);
            }
        }
        "server" => {
            println!("Iniciando modo Servidor...");
            // Inicia o runtime do Tokio e executa a lógica do servidor.
            if let Err(e) = iniciar_servidor() {
                eprintln!("Falha ao iniciar o servidor: {}", e);
            }
        }
        _ => {
            eprintln!("Comando inválido. Use 'cli' ou 'server'.");
        }
    }
}

#[tokio::main]
async fn iniciar_servidor() -> Result<(), Box<dyn std::error::Error>> {
    // --- Código do servidor web ---
    let conn = db::conectar_db()?;
    db::criar_tabelas(&conn)?;
 
    let db = Arc::new(Mutex::new(conn));
 
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);
 
    let app = Router::new()
        .route("/clientes", get(listar_clientes_api).post(criar_cliente))
        .route("/clientes/:id", get(obter_cliente).put(atualizar_cliente_api).delete(deletar_cliente_api))
    .route("/servicos", get(servicos::listar_servicos_query).post(servicos::criar_servico))
    .route("/servicos/:id", get(servicos::obter_servico).put(servicos::atualizar_servico).delete(servicos::excluir_servico))
    .route("/agendamentos", get(agendamentos::listar_agendamentos_api).post(agendamentos::criar_agendamento_api_incoming))
    .route("/availability", get(agendamentos::availability_api))
    .route("/work_windows", get(agendamentos::listar_work_windows_api).post(agendamentos::criar_work_window_api))
    .route("/agendamentos/:id", get(agendamentos::obter_agendamento_api).put(agendamentos::atualizar_agendamento_api).delete(agendamentos::excluir_agendamento_api))
        .layer(cors)
        .with_state(db);
 
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!(
        "Servidor rodando em http://{}",
        listener.local_addr()?
    );
    axum::serve(listener, app).await?;

    Ok(())
}