mod classes;
mod database;
mod embedding;

use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use anyhow::Result;
use limbo::Connection;
use database::initialization::{initialize_db_connection, init_tables};
use database::query::{get_works_by_autor_name, query_publicacoes_by_embedding};
use classes::{Autor, Publicacao};

#[derive(Deserialize)]
struct BuscaQuery {
    valor_para_busca: String,
    campo_de_busca: String,
}

#[derive(Deserialize)]
struct InsercaoDados {
    nome: Option<String>,
    ano_nascimento: Option<u32>,
    pais: Option<String>,
    titulo: Option<String>,
    autor: Option<String>,
    ano_publicacao: Option<u32>,
    resumo: Option<String>,
}

async fn buscar(query: web::Query<BuscaQuery>, conn: Connection) -> impl Responder {
    let valor = query.valor_para_busca.to_lowercase();
    let campo = query.campo_de_busca.to_lowercase();

    match campo.as_str() {
        "autor" => {
            match get_works_by_autor_name(&conn, &valor).await {
                Ok((publicacoes, _)) => web::Json(publicacoes),
                Err(e) => {
                    eprintln!("Error querying publicacoes by autor: {}", e);
                    web::Json(vec![])
                }
            }
        }
        "conteudo" => {
            match query_publicacoes_by_embedding(&conn, &valor, 5).await {
                Ok(results) => web::Json(results.into_iter().map(|(p, _)| p).collect::<Vec<_>>()),
                Err(e) => {
                    eprintln!("Error querying publicacoes by embedding: {}", e);
                    web::Json(vec![])
                }
            }
        }
        _ => web::Json(vec![]),
    }
}

async fn inserir_dados(dados: web::Json<InsercaoDados>, conn: Connection) -> impl Responder {
    if let Some(nome) = &dados.nome {
        // Insert Autor
        let autor = Autor::new(nome, dados.ano_nascimento.unwrap_or(0), &dados.pais.clone().unwrap_or_default());
        match autor.insert(&conn).await {
            Ok(id) => format!("Autor inserido com sucesso: ID {}", id),
            Err(e) => format!("Erro ao inserir autor: {}", e),
        }
    } else if let Some(titulo) = &dados.titulo {
        // Insert Publicacao
        let publicacao = match Publicacao::new(
            titulo,
            dados.ano_publicacao.unwrap_or(0),
            &dados.resumo.clone().unwrap_or_default(),
        ) {
            Ok(p) => p,
            Err(e) => return format!("Erro ao criar publicacao: {}", e),
        };
        match publicacao.insert(&conn, &dados.autor.clone().unwrap_or_default()).await {
            Ok((autor_id, publicacao_id)) => format!(
                "Publicacao inserida com sucesso: ID {} (Autor ID: {})",
                publicacao_id, autor_id
            ),
            Err(e) => format!("Erro ao inserir publicacao: {}", e),
        }
    } else {
        format!("Dados inválidos para inserção.")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = initialize_db_connection("database.db").await.expect("Failed to initialize database");
    init_tables(&conn).await.expect("Failed to initialize tables");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .wrap(Cors::default().allow_any_origin())
            .route("/buscar", web::get().to(move |query, conn| buscar(query, conn.clone())))
            .route("/inserir", web::post().to(move |dados, conn| inserir_dados(dados, conn.clone())))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
