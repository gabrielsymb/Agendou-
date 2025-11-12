// --- Módulos ---
pub mod db;
pub mod models;
pub mod licenca;
pub mod menu;
mod calc_preco;

// --- Bibliotecas ---
use rusqlite::Result;

fn main() -> Result<()> {
    println!("💈 Sistema de Agendamento Barbearia - Terminal Interativo");
    menu::menu_principal()
}