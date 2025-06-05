use limbo::{Connection};
use anyhow::Result;
use crate::embedding::compute_embedding; // Import the compute_embedding function
use crate::classes::Publicacao; // Import the Publicacao struct

/// Retrieves an Autor by name.
pub async fn get_autor_by_name(conn: &Connection, name: &str) -> Result<Option<(i64, Autor)>> {
    let mut stmt = conn.prepare(
        "SELECT id, nome, ano_nascimento, pais FROM Autores WHERE nome =?;"
    ).await?;
    let mut rows = stmt.query(&[&name]).await?;
    if let Some(row) = rows.next().await? {
        let id: i64 = row.get(0)?;
        let autor = Autor::try_from(row)?;
        Ok(Some((id, autor)))
    } else {
        Ok(None)
    }
}

/// Retrieves a Publicacao by title.
pub async fn get_publicacao_by_title(conn: &Connection, title: &str) -> Result<Option<(i64, Publicacao)>> {
    let mut stmt = conn.prepare(
        "SELECT id, titulo, ano_publicacao, resumo, embedding FROM Publicacoes WHERE titulo =?;"
    ).await?;
    let mut rows = stmt.query(&[&title]).await?;
    if let Some(row) = rows.next().await? {
        let id: i64 = row.get(0)?;
        let publicacao = Publicacao::try_from(row)?;
        Ok(Some((id, publicacao)))
    } else {
        Ok(None)
    }
}

/// Retrieves all Publicacoes written by a specific author name.
pub async fn get_works_by_autor_name(conn: &Connection, autor_name: &str) -> Result<(Vec<Publicacao>, Vec<Paper>)> {
    // Query for Publicacoes by author
    let mut publicacoes_stmt = conn.prepare(
        "SELECT P.id, P.titulo, P.ano_publicacao, P.resumo, P.embedding
         FROM Publicacoes P
         JOIN Escreveu_Publicacao EP ON P.id = EP.publicacao_id
         JOIN Autores A ON EP.autor_id = A.id
         WHERE A.nome =?;"
    ).await?;
    let mut publicacoes = Vec::new();
    let mut rows = publicacoes_stmt.query(&[&autor_name]).await?;
    while let Some(row) = rows.next().await? {
        publicacoes.push(Publicacao::try_from(row)?);
    }

    Ok((publicacoes, Vec::new()))
}

/// Parses a vector string in the format "vector('[f1,f2,...]')" into a Vec<f32>.
pub fn parse_vector_string(input: &str) -> Result<Vec<f32>> {
    if !input.starts_with("vector('") || !input.ends_with("')") {
        return Err(anyhow::anyhow!("Invalid vector string format: must be vector('[f1,f2,...]')"));
    }
    let json_str = &input[8..input.len() - 2]; // Extract "[f1,f2,...]"
    let vec: Vec<f32> = serde_json::from_str(json_str)?;
    Ok(vec)
}

/// Performs a similarity search for Publicacoes using vector_distance_cos.
/// Returns publications ordered by cosine distance (lower is more similar).
pub async fn query_publicacoes_by_embedding(conn: &Connection, prompt: &str, limit: u32) -> Result<Vec<(Publicacao, f64)>> {
    // Compute the embedding for the query text
    let query_embedding = compute_embedding(prompt)?;
    let embedding_json_str = serde_json::to_string(&query_embedding)?;

    let stmt = conn.prepare(
        "SELECT id, titulo, ano_publicacao, resumo, embedding,
                vector_distance_cos(embedding, vector(?)) AS distance
         FROM Publicacoes
         ORDER BY distance ASC
         LIMIT ?;"
    ).await?;
    let mut rows = stmt.query(&[&embedding_json_str, &limit]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        let publicacao = Publicacao::try_from(row.clone())?; // Clone row to allow multiple gets
        let distance: f64 = row.get(5)?; // Assuming distance is the 6th column (index 5)
        results.push((publicacao, distance));
    }
    Ok(results)
}
