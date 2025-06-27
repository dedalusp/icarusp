use actix_cors::Cors;
use actix_web::{post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

mod classes;
mod database;
mod embedding;

use classes::{Autor, Publicacao};
use database::initialization::{init_tables, initialize_db_connection};
use database::insertion::{insert_autor, insert_publication_with_author};
use database::query::{
    get_autor_by_name, get_publicacao_by_title, get_works_by_autor_name,
    query_publicacoes_by_embedding,
};

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
    nome: String,
    ano_nascimento: u32,
    pais: String,
}

#[derive(Serialize)]
struct InserirPublicacaoResponse {
    titulo: String,
    ano_publicacao: u32,
    resumo: String,
    autor: String,
}

#[post("/inserirAutor")]
async fn inserir_autor(dados: web::Json<InserirAutor>, pool: web::Data<PgPool>) -> impl Responder {
    let autor = Autor::new(&dados.nome, dados.ano_nascimento, &dados.pais);
    match insert_autor(&pool, &autor).await {
        Ok(_nome) => web::Json(InserirAutorResponse {
            nome: dados.nome.clone(),
            ano_nascimento: dados.ano_nascimento,
            pais: dados.pais.clone(),
        }),
        Err(e) => {
            eprintln!("Erro ao inserir autor: {}", e);
            web::Json(InserirAutorResponse {
                nome: "".to_string(),
                ano_nascimento: 0,
                pais: "".to_string(),
            })
        }
    }
}

#[post("/inserirPublicacao")]
async fn inserir_publicacao(
    dados: web::Json<InserirPublicacao>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let publicacao = match Publicacao::new(&dados.titulo, dados.ano_publicacao, &dados.resumo) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Erro ao criar publicacao: {}", e);
            return web::Json(InserirPublicacaoResponse {
                titulo: "".to_string(),
                ano_publicacao: 0,
                resumo: "".to_string(),
                autor: "".to_string(),
            });
        }
    };
    match insert_publication_with_author(&pool, &publicacao, &dados.autor).await {
        Ok((_autor_nome, _publicacao_titulo)) => web::Json(InserirPublicacaoResponse {
            titulo: dados.titulo.clone(),
            ano_publicacao: dados.ano_publicacao,
            resumo: dados.resumo.clone(),
            autor: dados.autor.clone(),
        }),
        Err(e) => {
            eprintln!("Erro ao inserir publicacao: {}", e);
            web::Json(InserirPublicacaoResponse {
                titulo: "".to_string(),
                ano_publicacao: 0,
                resumo: "".to_string(),
                autor: "".to_string(),
            })
        }
    }
}

#[post("/buscaVetorial")]
async fn busca_vetorial(
    query: web::Query<BuscaVetorial>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match query_publicacoes_by_embedding(&pool, &query.resumo, 5).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying publicacoes by embedding: {}", e);
            web::Json(vec![])
        }
    }
}

#[post("/buscaPorPublicacoes")]
async fn busca_por_publicacoes(
    query: web::Query<BuscaPorPublicacoes>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match get_publicacao_by_title(&pool, &query.titulo).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying publicacoes by title: {}", e);
            web::Json(vec![])
        }
    }
}

#[post("/buscaPorPublicacoesDoAutor")]
async fn busca_por_publicacoes_do_autor(
    query: web::Query<BuscaPorPublicacoesDoAutor>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match get_works_by_autor_name(&pool, &query.nome).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying publicacoes by autor: {}", e);
            web::Json(vec![])
        }
    }
}

#[post("/buscaPorAutor")]
async fn busca_por_autor(
    query: web::Query<BuscaPorAutor>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    match get_autor_by_name(&pool, &query.nome).await {
        Ok(results) => web::Json(results),
        Err(e) => {
            eprintln!("Error querying autor by name: {}", e);
            web::Json(vec![])
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Get database URL from environment variable or use default
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://username:password@localhost/database_name".to_string());

    let pool = initialize_db_connection(&database_url)
        .await
        .expect("Failed to initialize database connection pool");

    init_tables(&pool)
        .await
        .expect("Failed to initialize tables");

    println!("Starting server on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Cors::permissive())
            .service(inserir_autor)
            .service(inserir_publicacao)
            .service(busca_vetorial)
            .service(busca_por_publicacoes)
            .service(busca_por_publicacoes_do_autor)
            .service(busca_por_autor)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
