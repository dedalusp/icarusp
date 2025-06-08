use limbo::{Connection};
use anyhow::Result;
use serde::Serialize; // Import Serialize for JSON serialization
use crate::embedding::compute_embedding; // Import the compute_embedding function
use crate::classes::Publicacao; // Import the Publicacao struct

#[derive(Serialize)] // Derive Serialize for JSON serialization
pub struct AutorOutput {
    pub nome: String,
    pub ano_nascimento: u32,
    pub pais: String,
}

#[derive(Serialize)] // Derive Serialize for JSON serialization
pub struct PublicacaoOutput {
    pub titulo: String,
    pub ano_publicacao: u32,
    pub resumo: String,
}

#[derive(Serialize)] // Derive Serialize for JSON serialization
pub struct PublicacaoVetorial {
    pub publicacao: PublicacaoOutput,
    pub distance: f64,
}

/// Retrieves an Autor by a partial name match.
pub async fn get_autor_by_name(conn: &Connection, name: &str) -> Result<Vec<AutorOutput>> {
    let mut stmt = conn.prepare(
        "SELECT nome, ano_nascimento, pais FROM Autores WHERE nome LIKE ?;"
    ).await?;
    let mut rows = stmt.query(&[&format!("%{}%", name)]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        results.push(AutorOutput {
            nome: row.get(0)?,
            ano_nascimento: row.get(1)?,
            pais: row.get(2)?,
        });
    }
    Ok(results)
}

/// Retrieves a Publicacao by a partial title match.
pub async fn get_publicacao_by_title(conn: &Connection, title: &str) -> Result<Vec<PublicacaoOutput>> {
    let mut stmt = conn.prepare(
        "SELECT titulo, ano_publicacao, resumo FROM Publicacoes WHERE titulo LIKE ?;"
    ).await?;
    let mut rows = stmt.query(&[&format!("%{}%", title)]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        results.push(PublicacaoOutput {
            titulo: row.get(0)?,
            ano_publicacao: row.get(1)?,
            resumo: row.get(2)?,
        });
    }
    Ok(results)
}

/// Retrieves all Publicacoes written by a specific author name (partial match).
pub async fn get_works_by_autor_name(conn: &Connection, autor_name: &str) -> Result<Vec<PublicacaoOutput>> {
    let mut stmt = conn.prepare(
        "SELECT P.titulo, P.ano_publicacao, P.resumo
         FROM Publicacoes P
         JOIN Escreveu_Publicacao EP ON P.id = EP.publicacao_id
         JOIN Autores A ON EP.autor_id = A.id
         WHERE A.nome LIKE ?;"
    ).await?;
    let mut rows = stmt.query(&[&format!("%{}%", autor_name)]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        results.push(PublicacaoOutput {
            titulo: row.get(0)?,
            ano_publicacao: row.get(1)?,
            resumo: row.get(2)?,
        });
    }
    Ok(results)
}

/// Performs a similarity search for Publicacoes using vector_distance_cos.
/// Returns publications ordered by cosine distance (lower is more similar).
pub async fn query_publicacoes_by_embedding(conn: &Connection, prompt: &str, limit: u32) -> Result<Vec<PublicacaoVetorial>> {
    // Compute the embedding for the query text
    let query_embedding = compute_embedding(prompt)?;
    let embedding_json_str = serde_json::to_string(&query_embedding)?;

    let stmt = conn.prepare(
        "SELECT titulo, ano_publicacao, resumo,
                vector_distance_cos(embedding, vector(?)) AS distance
         FROM Publicacoes
         ORDER BY distance ASC
         LIMIT ?;"
    ).await?;
    let mut rows = stmt.query(&[&embedding_json_str, &limit]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        results.push(PublicacaoVetorial {
            publicacao: PublicacaoOutput {
                titulo: row.get(0)?,
                ano_publicacao: row.get(1)?,
                resumo: row.get(2)?,
            },
            distance: row.get(3)?, // Assuming distance is the 5th column (index 4)
        });
    }
    Ok(results)
}
