use limbo::{Connection, params};
use anyhow::Result;

/// Inserts an Autor into the Autores table and returns its generated ID.
pub async fn insert_autor(conn: &Connection, autor: &Autor) -> Result<i64> {
    let stmt = conn.prepare(
        "INSERT INTO Autores (nome, ano_nascimento, pais) VALUES (?,?,?) RETURNING id;"
    ).await?;
    let mut rows = stmt.query(&[
        &autor.nome,
        &autor.ano_nascimento,
        &autor.pais,
    ]).await?;
    let row = rows.next().await?.ok_or(anyhow::anyhow!("Failed to get inserted author ID"))?;
    Ok(row.get(0)?)
}

/// Inserts a Publicacao into the Publicacoes table and returns its generated ID.
pub async fn insert_publicacao(conn: &Connection, publicacao: &Publicacao) -> Result<i64> {
    let embedding_json_str = serde_json::to_string(&publicacao.embedding)?;

    let stmt = format!(
        "INSERT INTO Publicacoes (titulo, ano_publicacao, resumo, embedding) VALUES (?,?,?, vector(?)) RETURNING id;"
    );
    let mut rows = stmt.query(&[
        &publicacao.titulo,
        &publicacao.ano_publicacao,
        &publicacao.resumo,
        &embedding_json_str,
    ]).await?;
    let row = rows.next().await?.ok_or(anyhow::anyhow!("Failed to get inserted publication ID"))?;
    Ok(row.get(0)?)
}

/// Links an author to a publication in the Escreveu_Publicacao table.
pub async fn link_autor_publication(conn: &Connection, autor_id: i64, publication_id: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO Escreveu_Publicacao (autor_id, publication_id) VALUES (?,?);",
        params![autor_id, publication_id],
    ).await?;
    Ok(())
}

/// Inserts a publication and links it to an author by name.
/// Returns the IDs of the author and the publication.
pub async fn insert_publication_with_author(
    conn: &Connection,
    publicacao: &Publicacao,
    autor_nome: &str,
) -> Result<(i64, i64)> {
    // Retrieve author ID by name
    let stmt = conn.prepare("SELECT id FROM Autores WHERE nome = ?;").await?;
    let mut rows = stmt.query(&[&autor_nome]).await?;
    let row = rows.next().await?.ok_or(anyhow::anyhow!("Author not found"))?;
    let autor_id: i64 = row.get(0)?;

    // Insert publication and get its ID
    let publicacao_id = insert_publicacao(conn, publicacao).await?;

    // Link author and publication
    link_autor_publication(conn, autor_id, publicacao_id).await?; 

    Ok((autor_id, publicacao_id))
}
