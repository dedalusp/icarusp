use anyhow::Result;
use sqlx::PgPool;

use crate::classes::{Autor, Publicacao};

/// Inserts an Autor into the Autores table and returns the nome as string.
pub async fn insert_autor(pool: &PgPool, autor: &Autor) -> Result<String> {
    sqlx::query("INSERT INTO Autores (nome, ano_nascimento, pais) VALUES ($1, $2, $3);")
        .bind(autor.get_nome())
        .bind(autor.get_ano_nascimento() as i32)
        .bind(autor.get_pais())
        .execute(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to insert Autor: {}", e))?;

    Ok(autor.get_nome().to_string())
}

/// Inserts a Publicacao into the Publicacoes table and returns the titulo as string.
pub async fn insert_publicacao(pool: &PgPool, publicacao: &Publicacao) -> Result<String> {
    // Convert Vec<f32> to pgvector::Vector
    let embedding_vec = pgvector::Vector::from(publicacao.embedding.clone());

    sqlx::query(
        "INSERT INTO Publicacoes (titulo, ano_publicacao, resumo, embedding) VALUES ($1, $2, $3, $4);",
    )
    .bind(&publicacao.titulo)
    .bind(publicacao.ano_publicacao as i32)
    .bind(&publicacao.resumo)
    .bind(embedding_vec)
    .execute(pool)
    .await
    .map_err(|e| anyhow::anyhow!("Failed to insert Publicacao: {}", e))?;

    Ok(publicacao.titulo.clone())
}

/// Links an author to a publication in the Escreveu_Publicacao table.
pub async fn link_autor_publication(
    pool: &PgPool,
    autor_nome: &str,
    publicacao_titulo: &str,
) -> Result<()> {
    sqlx::query("INSERT INTO Escreveu_Publicacao (autor_nome, publicacao_titulo) VALUES ($1, $2);")
        .bind(autor_nome)
        .bind(publicacao_titulo)
        .execute(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to link autor and publicacao: {}", e))?;

    Ok(())
}

/// Inserts a publication and links it to an author by name.
/// Returns the names of the author and the publication.
pub async fn insert_publication_with_author(
    pool: &PgPool,
    publicacao: &Publicacao,
    autor_nome: &str,
) -> Result<(String, String)> {
    // Check if author exists
    let row = sqlx::query("SELECT nome FROM Autores WHERE nome = $1;")
        .bind(autor_nome)
        .fetch_optional(pool)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to execute author lookup query: {}", e))?;

    if row.is_none() {
        return Err(anyhow::anyhow!("Author '{}' not found", autor_nome));
    }

    // Insert publication and get its titulo
    let publicacao_titulo = insert_publicacao(pool, publicacao).await?;

    // Link author and publication
    link_autor_publication(pool, autor_nome, &publicacao_titulo).await?;

    Ok((autor_nome.to_string(), publicacao_titulo))
}
