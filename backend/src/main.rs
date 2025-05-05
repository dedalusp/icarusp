mod classes;

use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;

#[derive(Serialize)]
struct Manuscrito {
    titulo: String,
    ano_publicacao: u32,
    nome_autor: String,
    resumo: String,
    nacionalidade: String,
}

#[derive(Deserialize)]
struct BuscaQuery {
    valor_para_busca: String,
    campo_de_busca: String,
}

async fn buscar(query: web::Query<BuscaQuery>) -> impl Responder {
    let valor = query.valor_para_busca.to_lowercase();
    let campo = query.campo_de_busca.to_lowercase();

    match campo.as_str() {
        "autor" => web::Json(Manuscrito {
            titulo: "".to_string(),
            nome_autor: valor.to_string(),
            ano_publicacao: 0,
            resumo: "".to_string(),
            nacionalidade: "".to_string(),
        }),
        "titulo" => web::Json(Manuscrito {
            titulo: valor.to_string(),
            nome_autor: "".to_string(),
            ano_publicacao: 0,
            resumo: "".to_string(),
            nacionalidade: "".to_string(),
        }),
        "conteudo" => web::Json(Manuscrito {
            titulo: "".to_string(),
            nome_autor: "".to_string(),
            ano_publicacao: 0,
            resumo: valor.to_string(),
            nacionalidade: "".to_string(),
        }),
        "ano" => web::Json(Manuscrito {
            titulo: "".to_string(),
            nome_autor: "".to_string(),
            ano_publicacao: valor.parse().unwrap_or(100),
            resumo: "".to_string(),
            nacionalidade: "".to_string(),
        }),
        "nacionalidade" => web::Json(Manuscrito {
            titulo: "".to_string(),
            nome_autor: "".to_string(),
            ano_publicacao: 0,
            resumo: "".to_string(),
            nacionalidade: valor.to_string(),
        }),
        _ => web::Json(Manuscrito {
            titulo: "Não encontrado".to_string(),
            nome_autor: "".to_string(),
            ano_publicacao: 0,
            resumo: "".to_string(),
            nacionalidade: "".to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default().allow_any_origin()) // Permitir requisições de qualquer origem
            .route("/buscar", web::get().to(buscar)) // Endpoint genérico para busca
    })
    .bind("127.0.0.1:8080")? // Porta do backend
    .run()
    .await
}
