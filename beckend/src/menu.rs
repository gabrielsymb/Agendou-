use chrono::Local;
//  Padrão da linguagem
use std::io::{self, Write};

//  Datas e horários
use chrono::{Datelike, NaiveDateTime};

//  Banco de dados
use rusqlite::{params, Connection};

//  Módulos internos
use crate::db::*;
use crate::models::*;
use crate::licenca::*;
use crate::calc_preco::{calcular_lucro_total, calcular_lucro_mensal, calcular_lucro_do_dia};

/// Menu principal interativo
pub fn menu_principal() -> rusqlite::Result<()> {
    let conn = conectar_db()?;
    criar_tabelas(&conn)?;

    loop {
        println!("\n💈 Menu Principal");
        println!("0. Cadastrar serviço");
        println!("1. Cadastrar cliente");
        println!("2. Agendar serviço");
        println!("3. Ver agendamentos");
        println!("4. Validar licença");
        println!("5. Gerar relatório");
        println!("6. Sair");
        println!("7. Excluir serviço");
        println!("8. Excluir cliente");
        println!("9. Excluir agendamento");
        println!("10. Editar serviço");
        println!("11. Editar cliente");
        println!("12. Editar agendamento");
        println!("13. Ver clientes");
        println!("14. Ver serviços");
        println!("15. Ver agendamentos");
        println!("16. Marcar agendamento como concluído");
        println!("17. Ver agendamentos pendentes");
        println!("18. Ver agendamentos concluídos");
        println!("19. Relatório de produtividade do mês");
        println!("20. Relatório de produtividade do dia");
        print!("Escolha uma opção: ");
        io::stdout().flush().unwrap();

        let mut escolha = String::new();
        io::stdin().read_line(&mut escolha).unwrap();

        match escolha.trim() {
            "0" => cadastrar_servico(&conn)?,
            "1" => cadastrar_cliente(&conn)?,
            "2" => agendar_servico(&conn)?,
            "3" => ver_agendamentos(&conn)?,
            "4" => validar_licenca_terminal(&conn)?,
            "5" => gerar_relatorio_terminal(&conn)?,
            "6" => {
                println!("👋 Saindo...");
                break;
            }
            "7" => excluir_servico_terminal(&conn)?,
            "8" => excluir_cliente_terminal(&conn)?,
            "9" => excluir_agendamento_terminal(&conn)?,
            "10" => editar_servico_terminal(&conn)?,
            "11" => editar_cliente_terminal(&conn)?,
            "12" => editar_agendamento_terminal(&conn)?,
            "13" => listar_clientes_terminal(&conn)?,
            "14" => listar_servicos_terminal(&conn)?,
            "15" => listar_agendamentos_terminal(&conn)?,
            "16" => marcar_agendamento_concluido(&conn)?,
            "17" => listar_agendamentos_por_status(&conn, false)?,
            "18" => listar_agendamentos_por_status(&conn, true)?,
            "19" => relatorio_produtividade_mensal(&conn)?,
            "20" => relatorio_produtividade_do_dia(&conn)?,
            _ => println!("❌ Opção inválida."),
        }
    }

    Ok(())
}

pub fn cadastrar_cliente(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let mut nome = String::new();
    let mut telefone = String::new();
    let mut email = String::new();

    print!("Nome: "); io::stdout().flush().unwrap(); io::stdin().read_line(&mut nome).unwrap();
    print!("Telefone: "); io::stdout().flush().unwrap(); io::stdin().read_line(&mut telefone).unwrap();
    print!("Email (opcional): "); io::stdout().flush().unwrap(); io::stdin().read_line(&mut email).unwrap();

    if nome.trim().is_empty() || telefone.trim().is_empty() {
        println!("❌ Nome e telefone são obrigatórios.");
        return Ok(());
    }

    let mut cliente = Cliente::new(nome.trim().into(), telefone.trim().into(), Some(email.trim().into()));
    let id = salvar_cliente(conn, &mut cliente)?;
    println!("✅ Cliente cadastrado com ID: {}", id);
    Ok(())
}

pub fn agendar_servico(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let clientes = listar_clientes(conn)?;
    if clientes.is_empty() {
        println!("❌ Nenhum cliente cadastrado. Cadastre um cliente primeiro.");
        return Ok(());
    }

    println!("📋 Clientes disponíveis:");
    for c in &clientes {
        println!("  [{}] {} - {}", c.id.unwrap_or(0), c.nome, c.telefone);
    }

    let mut cliente_id = String::new();
    print!("ID do cliente (ou 'cancelar' para voltar): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut cliente_id).unwrap();

    if cliente_id.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Agendamento cancelado.");
        return Ok(());
    }

    let cliente_id: i32 = match cliente_id.trim().parse() {
        Ok(id) => id,
        _ => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };

    let cliente = match buscar_cliente_por_id(conn, cliente_id)? {
    Some(c) => c,
    None => {
        println!("❌ Cliente com ID {} não encontrado.", cliente_id);
        return Ok(());
    }
};

println!("📌 Agendando para: {} ({})", cliente.nome, cliente.telefone);

    let mut servicos = listar_servicos(conn)?;
    while servicos.is_empty() {
        println!("❌ Nenhum serviço cadastrado. Deseja cadastrar um agora? (s/n): ");
        let mut resposta = String::new();
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut resposta).unwrap();

        if resposta.trim().eq_ignore_ascii_case("s") {
            cadastrar_servico(conn)?;
            servicos = listar_servicos(conn)?;
        } else {
            println!("↩️ Agendamento cancelado.");
            return Ok(());
        }
    }

    println!("\n📋 Serviços Disponíveis:");
    for s in &servicos {
        println!("  {}. {} (R$ {:.2})", s.id.unwrap_or(0), s.nome, s.preco);
    }

    print!("Escolha os números dos serviços (separados por vírgula, ou 'cancelar' para voltar): ");
    io::stdout().flush().unwrap();
    let mut servicos_input = String::new();
    io::stdin().read_line(&mut servicos_input).unwrap();

    if servicos_input.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Agendamento cancelado.");
        return Ok(());
    }

    let servico_ids_str: Vec<&str> = servicos_input.trim().split(',').collect();
    let mut parsed_servico_ids: Vec<i32> = Vec::new();
    let mut total_preco = 0.0;
    let mut invalid_ids_found = false;

    let available_servicos = listar_servicos(conn)?; // Re-fetch available services

    for s_id_str in servico_ids_str {
        match s_id_str.trim().parse::<i32>() {
            Ok(id) => {
                if let Some(s) = available_servicos.iter().find(|serv| serv.id == Some(id)) {
                    parsed_servico_ids.push(id);
                    total_preco += s.preco;
                } else {
                    println!("❌ Serviço com ID {} não encontrado.", id);
                    invalid_ids_found = true;
                }
            },
            Err(_) => {
                println!("❌ ID de serviço inválido: '{}'.", s_id_str.trim());
                invalid_ids_found = true;
            }
        }
    }

    if invalid_ids_found || parsed_servico_ids.is_empty() {
        println!("❌ Agendamento cancelado devido a IDs de serviço inválidos ou nenhum serviço selecionado.");
        return Ok(());
    }

    print!("Data e hora (YYYY-MM-DD HH:MM) ou 'cancelar' para voltar: ");
    io::stdout().flush().unwrap();
    let mut data_hora = String::new();
    io::stdin().read_line(&mut data_hora).unwrap();

    if data_hora.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Agendamento cancelado.");
        return Ok(());
    }

    let horario = match NaiveDateTime::parse_from_str(data_hora.trim(), "%Y-%m-%d %H:%M") {
        Ok(h) => h,
        Err(_) => {
            println!("❌ Formato inválido. Use: YYYY-MM-DD HH:MM");
            return Ok(());
        }
    };

    if horario < Local::now().naive_local() {
        println!("❌ Não é possível agendar no passado. Por favor, escolha uma data e hora futuras.");
        return Ok(());
    }

    if verificar_conflito(conn, horario)? {
        println!("⚠️ Já existe um agendamento nesse horário.");
    } else {
        let agendamento = Agendamento {
            id: None,
            cliente_id,
            servicos_ids: parsed_servico_ids,
            data_hora: horario,
            preco: total_preco,
            concluido: false,
        };
        let id = salvar_agendamento(conn, &agendamento)?;
        println!("✅ Agendamento criado com ID: {}", id);
    }
    Ok(())
}

pub fn ver_agendamentos(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let agendamentos = listar_agendamentos(conn)?;
    if agendamentos.is_empty() {
        println!("📭 Nenhum agendamento encontrado.");
    } else {
        for a in agendamentos {
            let nomes = buscar_nomes_servicos(conn, &a.servicos_ids)?;
            println!(
                "[ID: {}] Cliente: {} | Serviço: {} | Data: {} | Preço: R$ {:.2} | Concluído: {}",
                a.id.unwrap_or(0), a.cliente_id, nomes.join(", "), a.data_hora, a.preco, a.concluido
            );
        }
    }
    Ok(())
}

pub fn validar_licenca_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let mut cliente_id = String::new();
    let chave = "BARBER2025";

    print!("ID do cliente: "); io::stdout().flush().unwrap(); io::stdin().read_line(&mut cliente_id).unwrap();
    let id: i32 = match cliente_id.trim().parse() {
        Ok(id) if id > 0 => id,
        _ => {
            println!("❌ ID do cliente inválido.");
            return Ok(());
        }
    };

    let codigo = gerar_codigo_semanal(chave);
    println!("🔐 Código semanal gerado: {}", codigo);

    print!("Digite o código recebido: "); io::stdout().flush().unwrap();
    let mut entrada = String::new(); io::stdin().read_line(&mut entrada).unwrap();

    if let Some(validade) = validar_codigo(entrada.trim(), chave) {
        atualizar_licenca(conn, id, validade)?;
        println!("✅ Licença válida até: {}", validade);
    } else {
        println!("❌ Código inválido.");
    }
    Ok(())
}

pub fn gerar_relatorio_terminal(conn: &Connection) -> rusqlite::Result<()> {
    let total = calcular_lucro_total(conn)?;
    println!("💰 Lucro total: R$ {:.2}", total);
    Ok(())
}
pub fn cadastrar_servico(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let mut nome = String::new();
    let mut preco_str = String::new();

    print!("Nome do serviço: "); io::stdout().flush().unwrap(); io::stdin().read_line(&mut nome).unwrap();
    print!("Preço do serviço: "); io::stdout().flush().unwrap(); io::stdin().read_line(&mut preco_str).unwrap();

    let preco: f64 = match preco_str.trim().parse() {
        Ok(p) if p >= 0.0 => p,
        _ => {
            println!("❌ Preço inválido.");
            return Ok(());
        }
    };

    if nome.trim().is_empty() {
        println!("❌ Nome do serviço não pode ser vazio.");
        return Ok(());
    }

    let servico = Servico { id: None, nome: nome.trim().into(), preco };
    let id = salvar_servico(conn, &servico)?;
    println!("✅ Serviço cadastrado com ID: {}", id);
    Ok(())
}

pub fn excluir_servico_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let servicos = listar_servicos(conn)?;
    if servicos.is_empty() {
        println!("📭 Nenhum serviço cadastrado.");
        return Ok(());
    }
    println!("🗑️ Serviços disponíveis:");
    for s in &servicos {
        println!("  [{}] {} - R$ {:.2}", s.id.unwrap_or(0), s.nome, s.preco);
    }
    print!("Digite o ID do serviço a excluir (ou 'cancelar'): ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new(); io::stdin().read_line(&mut entrada).unwrap();
    if entrada.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Exclusão cancelada.");
        return Ok(());
    }
    let id: i32 = match entrada.trim().parse() {
        Ok(i) => i,
        _ => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };
    excluir_servico(conn, id)?;
    println!("✅ Serviço excluído com sucesso.");
    Ok(())
}

pub fn excluir_cliente_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let clientes = listar_clientes(conn)?;
    if clientes.is_empty() {
        println!("📭 Nenhum cliente cadastrado.");
        return Ok(());
    }
    println!("🗑️ Clientes disponíveis:");
    for c in &clientes {
        println!("  [{}] {} - {}", c.id.unwrap_or(0), c.nome, c.telefone);
    }
    print!("Digite o ID do cliente a excluir (ou 'cancelar'): ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new(); io::stdin().read_line(&mut entrada).unwrap();
    if entrada.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Exclusão cancelada.");
        return Ok(());
    }
    let id: i32 = match entrada.trim().parse() {
        Ok(i) => i,
        _ => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };
    match excluir_cliente(conn, id) {
        Ok(_) => {
            println!("✅ Cliente excluído com sucesso.");
        }
        Err(e) => {
            // Verifica se o erro é o específico que criamos para "cliente com agendamentos"
            if let Some(app_err) = e.sqlite_error() {
                if app_err.extended_code == rusqlite::ffi::SQLITE_CONSTRAINT_FOREIGNKEY {
                     println!("❌ Não foi possível excluir o cliente. Existem agendamentos vinculados a ele.");
                     return Ok(()); // Retorna Ok para não parar o programa
                }
            }
            // Para outros erros, imprime a mensagem genérica e propaga o erro.
            println!("❌ Erro ao excluir cliente: {}", e);
        }
    }    
    Ok(())
}

pub fn excluir_agendamento_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let agendamentos = listar_agendamentos(conn)?;
    if agendamentos.is_empty() {
        println!("📭 Nenhum agendamento cadastrado.");
        return Ok(());
    }
    println!("🗑️ Agendamentos disponíveis:");
    for a in &agendamentos {
        let nomes = buscar_nomes_servicos(conn, &a.servicos_ids)?;
        println!(
            "  [ID: {}] Cliente: {} | Serviço: {} | Data: {} | Preço: R$ {:.2} | Concluído: {}",
            a.id.unwrap_or(0), a.cliente_id, nomes.join(", "), a.data_hora, a.preco, a.concluido
        );
    }
    print!("Digite o ID do agendamento a excluir (ou 'cancelar'): ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new(); io::stdin().read_line(&mut entrada).unwrap();
    if entrada.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Exclusão cancelada.");
        return Ok(());
    }
    let id: i32 = match entrada.trim().parse() {
        Ok(i) => i,
        _ => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };
    excluir_agendamento(conn, id)?;
    println!("✅ Agendamento excluído com sucesso.");
    Ok(())
}

pub fn editar_servico_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let servicos = listar_servicos(conn)?;
    if servicos.is_empty() {
        println!("📭 Nenhum serviço cadastrado.");
        return Ok(());
    }

    println!("📝 Serviços disponíveis:");
    for s in &servicos {
        println!("  [{}] {} - R$ {:.2}", s.id.unwrap_or(0), s.nome, s.preco);
    }

    print!("Digite o ID do serviço a editar (ou 'cancelar'): ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new(); io::stdin().read_line(&mut entrada).unwrap();
    if entrada.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Edição cancelada.");
        return Ok(());
    }

    let id: i32 = match entrada.trim().parse() {
        Ok(i) => i,
        _ => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };

    print!("Novo nome do serviço: "); io::stdout().flush().unwrap();
    let mut nome = String::new(); io::stdin().read_line(&mut nome).unwrap();

    print!("Novo preço: "); io::stdout().flush().unwrap();
    let mut preco = String::new(); io::stdin().read_line(&mut preco).unwrap();

    let preco: f64 = match preco.trim().parse() {
        Ok(p) => p,
        _ => {
            println!("❌ Preço inválido.");
            return Ok(());
        }
    };

    conn.execute(
        "UPDATE servicos SET nome = ?1, preco = ?2 WHERE id = ?3",
        params![nome.trim(), preco, id],
    )?;
    println!("✅ Serviço atualizado com sucesso.");
    Ok(())
}

pub fn editar_cliente_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let clientes = listar_clientes(conn)?;
    if clientes.is_empty() {
        println!("📭 Nenhum cliente cadastrado.");
        return Ok(());
    }

    println!("📝 Clientes disponíveis:");
    for c in &clientes {
        println!("  [{}] {} - {}", c.id.unwrap_or(0), c.nome, c.telefone);
    }

    print!("Digite o ID do cliente a editar (ou 'cancelar'): ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new(); io::stdin().read_line(&mut entrada).unwrap();
    if entrada.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Edição cancelada.");
        return Ok(());
    }

    let id: i32 = match entrada.trim().parse() {
        Ok(i) => i,
        _ => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };

    // Busca o cliente para poder modificá-lo
    let mut cliente = match buscar_cliente_por_id(conn, id)? {
        Some(c) => c,
        None => {
            println!("❌ Cliente com ID {} não encontrado.", id);
            return Ok(());
        }
    };

    print!("Novo nome: "); io::stdout().flush().unwrap();
    let mut nome = String::new(); io::stdin().read_line(&mut nome).unwrap();

    print!("Novo telefone: "); io::stdout().flush().unwrap();
    let mut telefone = String::new(); io::stdin().read_line(&mut telefone).unwrap();

    print!("Novo email: "); io::stdout().flush().unwrap();
    let mut email = String::new(); io::stdin().read_line(&mut email).unwrap();

    // Atualiza os dados do struct do cliente
    cliente.nome = nome.trim().to_string();
    cliente.telefone = telefone.trim().to_string();
    cliente.email = Some(email.trim().to_string());

    salvar_cliente(conn, &mut cliente)?;
    println!("✅ Cliente atualizado com sucesso.");
    Ok(())
}

pub fn editar_agendamento_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let agendamentos = listar_agendamentos(conn)?;
    if agendamentos.is_empty() {
        println!("📭 Nenhum agendamento cadastrado.");
        return Ok(());
    }

    println!("📝 Agendamentos disponíveis:");
    for a in &agendamentos {
        let nomes = buscar_nomes_servicos(conn, &a.servicos_ids)?;
        println!(
            "  [{}] Cliente: {} | Serviço: {} | Data: {} | Preço: R$ {:.2} | Concluído: {}",
            a.id.unwrap_or(0), a.cliente_id, nomes.join(", "), a.data_hora, a.preco, a.concluido
        );
    }

    print!("Digite o ID do agendamento a editar (ou 'cancelar'): ");
    io::stdout().flush().unwrap();
    let mut entrada = String::new(); io::stdin().read_line(&mut entrada).unwrap();
    if entrada.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Edição cancelada.");
        return Ok(());
    }

    let id: i32 = match entrada.trim().parse() {
        Ok(i) => i,
        _ => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };

    print!("Nova data e hora (YYYY-MM-DD HH:MM): "); io::stdout().flush().unwrap();
    let mut data_hora = String::new(); io::stdin().read_line(&mut data_hora).unwrap();

    let horario = match NaiveDateTime::parse_from_str(data_hora.trim(), "%Y-%m-%d %H:%M") {
        Ok(h) => h,
        Err(_) => {
            println!("❌ Formato inválido. Use: YYYY-MM-DD HH:MM");
            return Ok(());
        }
    };

    print!("Novo preço: "); io::stdout().flush().unwrap();
    let mut preco = String::new(); io::stdin().read_line(&mut preco).unwrap();

    let preco: f64 = match preco.trim().parse() {
        Ok(p) => p,
        _ => {
            println!("❌ Preço inválido.");
            return Ok(());
        }
    };

    conn.execute(
        "UPDATE agendamentos SET data_hora = ?1, preco = ?2 WHERE id = ?3",
        params![horario.format("%Y-%m-%d %H:%M:%S").to_string(), preco, id],
    )?;
    println!("✅ Agendamento atualizado com sucesso.");
    Ok(())
}
pub fn listar_clientes_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let clientes = listar_clientes(conn)?;
    if clientes.is_empty() {
        println!("📭 Nenhum cliente cadastrado.");
    } else {
        println!("📋 Clientes cadastrados:");
        for c in clientes {
            println!(
                "  [{}] {} - {}{}",
                c.id.unwrap_or(0),
                c.nome,
                c.telefone,
                if let Some(email) = &c.email {
                    format!(" | {}", email.trim())
                } else {
                    "".to_string()
                }
            );
        }
    }
    Ok(())
}
pub fn listar_servicos_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let servicos = listar_servicos(conn)?;
    if servicos.is_empty() {
        println!("📭 Nenhum serviço cadastrado.");
    } else {
        println!("📋 Serviços cadastrados:");
        for s in servicos {
            println!("  [{}] {} - R$ {:.2}", s.id.unwrap_or(0), s.nome, s.preco);
        }
    }
    Ok(())
}
pub fn listar_agendamentos_terminal(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let agendamentos = listar_agendamentos(conn)?;
    if agendamentos.is_empty() {
        println!("📭 Nenhum agendamento cadastrado.");
    } else {
        println!("📋 Agendamentos:");
        for a in agendamentos {
            let nomes = buscar_nomes_servicos(conn, &a.servicos_ids)?;
            println!(
                "  [{}] Cliente: {} | Serviço: {} | Data: {} | Preço: R$ {:.2} | Status: {}",
                a.id.unwrap_or(0),
                a.cliente_id,
                nomes.join(", "),
                a.data_hora.format("%d/%m/%Y %H:%M"),
                a.preco,
                if a.concluido { "[ok!]" } else { "[pendente]" }
            );
        }
    }
    Ok(())
}
pub fn marcar_agendamento_concluido(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    let agendamentos = listar_agendamentos(conn)?;
    if agendamentos.is_empty() {
        println!("📭 Nenhum agendamento encontrado.");
        return Ok(());
    }

    println!("📋 Agendamentos pendentes:");
    for a in &agendamentos {
        if !a.concluido {
            let nomes = buscar_nomes_servicos(conn, &a.servicos_ids)?;
            println!(
                "  [{}] Cliente: {} | Serviço: {} | Data: {}",
                a.id.unwrap_or(0),
                a.cliente_id,
                nomes.join(", "),
                a.data_hora.format("%d/%m/%Y %H:%M")
            );
        }
    }

    print!("Digite o ID do agendamento a concluir (ou 'cancelar'): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().eq_ignore_ascii_case("cancelar") {
        println!("↩️ Ação cancelada.");
        return Ok(());
    }

    let id: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("❌ ID inválido.");
            return Ok(());
        }
    };

    let mut stmt = conn.prepare("UPDATE agendamentos SET concluido = 1 WHERE id = ?1")?;
    let rows = stmt.execute(rusqlite::params![id])?;

    if rows == 0 {
        println!("❌ Nenhum agendamento encontrado com esse ID.");
    } else {
        println!("✅ Agendamento marcado como concluído!");
    }

    Ok(())
}
pub fn listar_agendamentos_por_status(conn: &rusqlite::Connection, concluido: bool) -> rusqlite::Result<()> {
    let agendamentos = listar_agendamentos(conn)?;
    let filtrados: Vec<_> = agendamentos.into_iter().filter(|a| a.concluido == concluido).collect();

    if filtrados.is_empty() {
        println!("📭 Nenhum agendamento {}.", if concluido { "concluído" } else { "pendente" });
    } else {
        println!("📋 Agendamentos {}:", if concluido { "concluídos" } else { "pendentes" });
        for a in filtrados {
            let nomes = buscar_nomes_servicos(conn, &a.servicos_ids)?;
            println!(
                "  [{}] Cliente: {} | Serviço: {} | Data: {} | Preço: R$ {:.2}",
                a.id.unwrap_or(0),
                a.cliente_id,
                nomes.join(", "),
                a.data_hora.format("%d/%m/%Y %H:%M"),
                a.preco
            );
        }
    }
    Ok(())
}
pub fn relatorio_produtividade_mensal(conn: &Connection) -> rusqlite::Result<()> {
    let hoje = Local::now().naive_local();
    let mes = hoje.month();
    let ano = hoje.year();

    let receita = calcular_lucro_mensal(conn)?;
    println!("📊 Relatório de produtividade - {}/{}", mes, ano);
    println!("💰 Receita total: R$ {:.2}", receita);

    Ok(())
}

pub fn relatorio_produtividade_do_dia(conn: &Connection) -> rusqlite::Result<()> {
    let hoje = Local::now().naive_local().date();
    let agendamentos = listar_agendamentos(conn)?;

    let concluidos_hoje: Vec<_> = agendamentos
        .into_iter()
        .filter(|a| a.concluido && a.data_hora.date() == hoje)
        .collect();

    let total = concluidos_hoje.len();
    let receita = calcular_lucro_do_dia(conn)?;
    println!("📅 Relatório de hoje - {}", hoje.format("%d/%m/%Y"));
    println!("✅ Serviços concluídos: {}", total);
    println!("💰 Receita do dia: R$ {:.2}", receita);

    Ok(())
}
