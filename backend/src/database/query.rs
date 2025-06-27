use anyhow::Result;
use pgvector::Vector;
use serde::Serialize;
use sqlx::{PgPool, Row};

use crate::embedding::compute_embedding;

#[derive(Serialize)]
pub struct AutorOutput {
    pub nome: String,
    pub ano_nascimento: u32,
    pub pais: String,
}

#[derive(Serialize)]
pub struct PublicacaoOutput {
    pub titulo: String,
    pub ano_publicacao: u32,
    pub resumo: String,
}

#[derive(Serialize)]
pub struct PublicacaoVetorial {
    pub publicacao: PublicacaoOutput,
    pub distance: f64,
}

/// Retrieves an Autor by a partial name match.
pub async fn get_autor_by_name(pool: &PgPool, name: &str) -> Result<Vec<AutorOutput>> {
    let pattern = format!("%{}%", name);

    let rows = sqlx::query("SELECT nome, ano_nascimento, pais FROM Autores WHERE nome ILIKE $1;")
        .bind(&pattern)
        .fetch_all(pool)
        .await?;

    let mut results = Vec::new();
    for row in rows {
        results.push(AutorOutput {
            nome: row.get("nome"),
            ano_nascimento: row.get::<i32, _>("ano_nascimento") as u32,
            pais: row.get("pais"),
        });
    }
    Ok(results)
}

/// Retrieves a Publicacao by a partial title match.
pub async fn get_publicacao_by_title(pool: &PgPool, title: &str) -> Result<Vec<PublicacaoOutput>> {
    let pattern = format!("%{}%", title);

    let rows = sqlx::query(
        "SELECT titulo, ano_publicacao, resumo FROM Publicacoes WHERE titulo ILIKE $1;",
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await?;

    let mut results = Vec::new();
    for row in rows {
        results.push(PublicacaoOutput {
            titulo: row.get("titulo"),
            ano_publicacao: row.get::<i32, _>("ano_publicacao") as u32,
            resumo: row.get("resumo"),
        });
    }
    Ok(results)
}

/// Retrieves all Publicacoes written by a specific author name (partial match).
pub async fn get_works_by_autor_name(
    pool: &PgPool,
    autor_name: &str,
) -> Result<Vec<PublicacaoOutput>> {
    let pattern = format!("%{}%", autor_name);

    let rows = sqlx::query(
        "SELECT P.titulo, P.ano_publicacao, P.resumo
         FROM Publicacoes P
         JOIN Escreveu_Publicacao EP ON P.titulo = EP.publicacao_titulo
         WHERE EP.autor_nome ILIKE $1;",
    )
    .bind(&pattern)
    .fetch_all(pool)
    .await?;

    let mut results = Vec::new();
    for row in rows {
        results.push(PublicacaoOutput {
            titulo: row.get("titulo"),
            ano_publicacao: row.get::<i32, _>("ano_publicacao") as u32,
            resumo: row.get("resumo"),
        });
    }
    Ok(results)
}

/// Performs a similarity search for Publicacoes using pgvector cosine distance.
/// Returns publications ordered by cosine distance (lower is more similar).
pub async fn query_publicacoes_by_embedding(
    pool: &PgPool,
    prompt: &str,
    limit: u32,
) -> Result<Vec<PublicacaoVetorial>> {
    // Compute the embedding for the query text
    let query_embedding = compute_embedding(prompt)?;
    let embedding_vec = Vector::from(query_embedding);

    let rows = sqlx::query(
        "SELECT titulo, ano_publicacao, resumo,
                (embedding <=> $1) AS distance
         FROM Publicacoes
         ORDER BY distance ASC
         LIMIT $2;",
    )
    .bind(embedding_vec)
    .bind(limit as i64)
    .fetch_all(pool)
    .await?;

    let mut results = Vec::new();
    for row in rows {
        results.push(PublicacaoVetorial {
            publicacao: PublicacaoOutput {
                titulo: row.get("titulo"),
                ano_publicacao: row.get::<i32, _>("ano_publicacao") as u32,
                resumo: row.get("resumo"),
            },
            distance: row.get::<f64, _>("distance"),
        });
    }
    Ok(results)
}
