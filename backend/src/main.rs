use actix_web::{post, web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_cors::Cors;
use limbo::Connection;

mod classes;
mod database;

use classes::{Autor, Publicacao};
use database::initialization::{initialize_db_connection, init_tables};
use database::query::{get_autor_by_name, get_publicacao_by_title, get_works_by_autor_name, query_publicacoes_by_embedding};

#[derive(Deserialize)]
struct InserirAutor {
    nome: String,
    ano_nascimento: u32,
    pais: String,
}

#[derive(Deserialize)]
struct InserirPublicacao {
    titulo: String,
    ano_publicacao: u32,
    resumo: String,
    autor: String,
}

#[derive(Deserialize)]
struct BuscaVetorial {
    resumo: String,
}

#[derive(Deserialize)]
struct BuscaPorPublicacoes {
    titulo: String,
}

#[derive(Deserialize)]
struct BuscaPorPublicacoesDoAutor {
    nome: String,
}

#[derive(Deserialize)]
struct BuscaPorAutor {
    nome: String,
}

#[derive(Serialize)]
struct InserirAutorResponse {
    id: i64,
    nome: String,
    ano_nascimento: u32,
    pais: String,
}

#[derive(Serialize)]
struct InserirPublicacaoResponse {
    publicacao_id: i64,
    autor_id: i64,
    titulo: String,
    ano_publicacao: u32,
    resumo: String,
    autor: String,
}

#[post("/inserirAutor")]
async fn inserir_autor(dados: web::Json<InserirAutor>, conn: web::Data<Connection>) -> impl Responder {
    let autor = Autor::new(&dados.nome, dados.ano_nascimento, &dados.pais);
    match autor.insert(&conn).await {
        Ok(id) => web::Json(InserirAutorResponse {
            id,
            nome: dados.nome.clone(),
            ano_nascimento: dados.ano_nascimento,
            pais: dados.pais.clone(),
        }),
        Err(e) => {
            eprintln!("Erro ao inserir autor: {}", e);
            web::Json(InserirAutorResponse {
                id: -1,
                nome: "".to_string(),
                ano_nascimento: 0,
                pais: "".to_string(),
            })
        }
    }
}
#[post("/inserirPublicacao")]
async fn inserir_autor(dados: web::Json<InserirAutor>, conn: web::Data<Connection>) -> impl Responder {
    let autor = Autor::new(&dados.nome, dados.ano_nascimento, &dados.pais);
    match autor.insert(&conn).await {
        Ok(id) => web::Json(InserirAutorResponse {
            id,
            nome: dados.nome.clone(),
            ano_nascimento: dados.ano_nascimento,
            pais: dados.pais.clone(),
        }),
        Err(e) => {
            eprintln!("Erro ao inserir autor: {}", e);
            web::Json(InserirAutorResponse {
                id: -1,
                nome: "".to_string(),
                ano_nascimento: 0,
                pais: "".to_string(),
            })
        }
    }
}

#[post("/inserirPublicacao")]
async fn inserir_publicacao(dados: web::Json<InserirPublicacao>, conn: web::Data<Connection>) -> impl Responder {
    let publicacao = match Publicacao::new(&dados.titulo, dados.ano_publicacao, &dados.resumo) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Erro ao criar publicacao: {}", e);
            return web::Json(InserirPublicacaoResponse {
                publicacao_id: -1,
                autor_id: -1,
                titulo: "".to_string(),
                ano_publicacao: 0,
                resumo: "".to_string(),
                autor: "".to_string(),
            });
        }
    };
    match publicacao.insert(&conn, &dados.autor).await {
        Ok((autor_id, publicacao_id)) => web::Json(InserirPublicacaoResponse {
            publicacao_id,
            autor_id,
            titulo: dados.titulo.clone(),
            ano_publicacao: dados.ano_publicacao,
            resumo: dados.resumo.clone(),
            autor: dados.autor.clone(),
        }),
        Err(e) => {
            eprintln!("Erro ao inserir publicacao: {}", e);
            web::Json(InserirPublicacaoResponse {
                publicacao_id: -1,
                autor_id: -1,
                titulo: "".to_string(),
                ano_publicacao: 0,
                resumo: "".to_string(),
                autor: "".to_string(),
            })
        }
    }
}

#[post("/buscaVetorial")]
async fn busca_vetorial(query: web::Query<BuscaVetorial>, conn: web::Data<Connection>) -> impl Responder {
    match query_publicacoes_by_embedding(&conn, &query.resumo, 5).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying publicacoes by embedding: {}", e);
            web::Json(vec![])
        }
    }
}

#[post("/buscaPorPublicacoes")]
async fn busca_por_publicacoes(query: web::Query<BuscaPorPublicacoes>, conn: web::Data<Connection>) -> impl Responder {
    match get_publicacao_by_title(&conn, &query.titulo).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying publicacoes by title: {}", e);
            web::Json(vec![])
        }
    }
}

#[post("/buscaPorPublicacoesDoAutor")]
async fn busca_por_publicacoes_do_autor(query: web::Query<BuscaPorPublicacoesDoAutor>, conn: web::Data<Connection>) -> impl Responder {
    match get_works_by_autor_name(&conn, &query.nome).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying publicacoes by autor: {}", e);
            web::Json(vec![])
        }
    }
}

#[post("/buscaPorAutor")]
async fn busca_por_autor(query: web::Query<BuscaPorAutor>, conn: web::Data<Connection>) -> impl Responder {
    match get_autor_by_name(&conn, &query.nome).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying autor by name: {}", e);
            web::Json(vec![])
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = initialize_db_connection("database.db").await.expect("Failed to initialize database");
    init_tables(&conn).await.expect("Failed to initialize tables");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(conn.clone()))
            .wrap(Cors::permissive())
            .service(inserir_autor)
            .service(inserir_publicacao)
            .service(busca_vetorial)
            .service(busca_por_publicacoes)
            .service(busca_por_publicacoes_do_autor)
            .service(busca_por_autor)
    })
    .bind("127.0.1:8080")?
    .run()
    .await
}
