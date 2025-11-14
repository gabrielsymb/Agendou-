use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

// =================================================================================
// ESTRUTURAS DE DADOS (Modelos de Negócio)
// =================================================================================

/// Estrutura que representa um Cliente da Barbearia.
#[derive(Debug, Serialize, Deserialize)]
pub struct Cliente {
    // Option<i32> é usado para IDs que podem ser None (antes de serem salvos)
    pub id: Option<i32>,
    pub nome: String,
    pub telefone: String,
    pub email: Option<String>,
}

/// Estrutura que representa um Agendamento (Corte/Serviço).
#[derive(Debug, Serialize, Deserialize)]
pub struct Agendamento {
    pub id: Option<i32>,
    // Chave estrangeira
    pub cliente_id: i32, 
    pub servicos_ids: Vec<i32>,
    pub data_hora: NaiveDateTime,
    pub preco: f64, 
    pub concluido: bool, 
}

// Implementar um método de criação (constructor)
impl Cliente {
    pub fn new(nome: String, telefone: String, email: Option<String>) -> Self {
        Cliente {
            id: None,
            nome,
            telefone,
            email,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Servico {
    // Agora id é um i32 (sempre presente após persistência). Usamos Option<i32>
    // durante criação no frontend, mas a struct pública do backend espera id: i32
    pub id: Option<i32>,
    pub nome: String,
    pub preco: f64,
    pub duracao_min: i32,
}
#[derive(Debug)]
pub struct UsuarioSistema {
    pub id: i32,
    pub nome: String,
    pub licenca_ativa: bool,
    pub codigo_licenca: String,
}