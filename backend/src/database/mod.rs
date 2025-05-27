use libsql::{Builder, Connection};
use anyhow::Result; // Using anyhow for simplified error handling

/// Initializes a local libsql database connection.
/// If the database file does not exist, it will be created.
pub async fn initialize_db_connection(db_path: &str) -> Result<Connection> {
    let db = Builder::new_local(db_path).build().await?;
    let conn = db.connect()?;
    Ok(conn)
}

/// Initializes the database schema by creating all necessary tables.
pub async fn create_tables(conn: &Connection, vector_dimension: u32) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Autores (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL UNIQUE,
            ano_nascimento INTEGER,
            pais TEXT
        );",
        (),
    ).await?;

    conn.execute(
        &format!("CREATE TABLE IF NOT EXISTS Livros (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL UNIQUE,
            ano_publicacao INTEGER,
            resumo TEXT,
            isbn TEXT UNIQUE,
            edicao INTEGER,
            embedding F32_BLOB({})
        );", vector_dimension),
        (),
    ).await?;

    conn.execute(
        &format!("CREATE TABLE IF NOT EXISTS Papers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL UNIQUE,
            ano_publicacao INTEGER,
            resumo TEXT,
            doi TEXT UNIQUE,
            bibliografia TEXT,
            embedding F32_BLOB({})
        );", vector_dimension),
        (),
    ).await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Escreveu_Livro (
            autor_id INTEGER NOT NULL,
            livro_id INTEGER NOT NULL,
            PRIMARY KEY (autor_id, livro_id),
            FOREIGN KEY (autor_id) REFERENCES Autores(id) ON DELETE CASCADE,
            FOREIGN KEY (livro_id) REFERENCES Livros(id) ON DELETE CASCADE
        );",
        (),
    ).await?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS Escreveu_Paper (
            autor_id INTEGER NOT NULL,
            paper_id INTEGER NOT NULL,
            PRIMARY KEY (autor_id, paper_id),
            FOREIGN KEY (autor_id) REFERENCES Autores(id) ON DELETE CASCADE,
            FOREIGN KEY (paper_id) REFERENCES Papers(id) ON DELETE CASCADE
        );",
        (),
    ).await?;

    Ok(())
}

/// Converts a Vec<f32> to its JSON string representation.
pub fn f32_vec_to_json_string(vec: &[f32]) -> String {
    serde_json::to_string(vec).unwrap_or_else(|_| "".to_string())
}

use libsql::{Connection, params};

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

/// Inserts a Livro into the Livros table and returns its generated ID.
pub async fn insert_livro(conn: &Connection, livro: &Livro) -> Result<i64> {
    let embedding_json_str = if let Some(ref emb) = livro.manuscrito.embedding {
        f32_vec_to_json_string(emb)
    } else {
        "NULL".to_string()
    };

    let sql = format!(
        "INSERT INTO Livros (titulo, ano_publicacao, resumo, isbn, edicao, embedding) VALUES (?,?,?,?,?, vector(?)) RETURNING id;"
    );
    let stmt = conn.prepare(&sql).await?;
    let mut rows = stmt.query(&).await?;
    let row = rows.next().await?.ok_or(anyhow::anyhow!("Failed to get inserted book ID"))?;
    Ok(row.get(0)?)
}

/// Inserts a Paper into the Papers table and returns its generated ID.
pub async fn insert_paper(conn: &Connection, paper: &Paper) -> Result<i64> {
    let embedding_json_str = if let Some(ref emb) = paper.manuscrito.embedding {
        f32_vec_to_json_string(emb)
    } else {
        "NULL".to_string()
    };

    let sql = format!(
        "INSERT INTO Papers (titulo, ano_publicacao, resumo, doi, bibliografia, embedding) VALUES (?,?,?,?,?, vector(?)) RETURNING id;"
    );
    let stmt = conn.prepare(&sql).await?;
    let mut rows = stmt.query(&).await?;
    let row = rows.next().await?.ok_or(anyhow::anyhow!("Failed to get inserted paper ID"))?;
    Ok(row.get(0)?)
}

/// Links an author to a book in the Escreveu_Livro table.
pub async fn link_autor_livro(conn: &Connection, autor_id: i64, livro_id: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO Escreveu_Livro (autor_id, livro_id) VALUES (?,?);",
        params![autor_id, livro_id],
    ).await?;
    Ok(())
}

/// Links an author to a paper in the Escreveu_Paper table.
pub async fn link_autor_paper(conn: &Connection, autor_id: i64, paper_id: i64) -> Result<()> {
    conn.execute(
        "INSERT INTO Escreveu_Paper (autor_id, paper_id) VALUES (?,?);",
        params![autor_id, paper_id],
    ).await?;
    Ok(())
}

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

/// Retrieves a Livro by title.
pub async fn get_livro_by_title(conn: &Connection, title: &str) -> Result<Option<(i64, Livro)>> {
    let mut stmt = conn.prepare(
        "SELECT id, titulo, ano_publicacao, resumo, isbn, edicao, embedding FROM Livros WHERE titulo =?;"
    ).await?;
    let mut rows = stmt.query(&[&title]).await?;
    if let Some(row) = rows.next().await? {
        let id: i64 = row.get(0)?;
        let livro = Livro::try_from(row)?;
        Ok(Some((id, livro)))
    } else {
        Ok(None)
    }
}

/// Retrieves a Paper by title.
pub async fn get_paper_by_title(conn: &Connection, title: &str) -> Result<Option<(i64, Paper)>> {
    let mut stmt = conn.prepare(
        "SELECT id, titulo, ano_publicacao, resumo, doi, bibliografia, embedding FROM Papers WHERE titulo =?;"
    ).await?;
    let mut rows = stmt.query(&[&title]).await?;
    if let Some(row) = rows.next().await? {
        let id: i64 = row.get(0)?;
        let paper = Paper::try_from(row)?;
        Ok(Some((id, paper)))
    } else {
        Ok(None)
    }
}

/// Retrieves all Livros and Papers written by a specific author name.
pub async fn get_works_by_autor_name(conn: &Connection, autor_name: &str) -> Result<(Vec<Livro>, Vec<Paper>)> {
    // Query for Livros by author
    let mut livros_stmt = conn.prepare(
        "SELECT L.id, L.titulo, L.ano_publicacao, L.resumo, L.isbn, L.edicao, L.embedding
         FROM Livros L
         JOIN Escreveu_Livro EL ON L.id = EL.livro_id
         JOIN Autores A ON EL.autor_id = A.id
         WHERE A.nome =?;"
    ).await?;
    let mut livros = Vec::new();
    let mut rows = livros_stmt.query(&[&autor_name]).await?;
    while let Some(row) = rows.next().await? {
        livros.push(Livro::try_from(row)?);
    }

    // Query for Papers by author
    let mut papers_stmt = conn.prepare(
        "SELECT P.id, P.titulo, P.ano_publicacao, P.resumo, P.doi, P.bibliografia, P.embedding
         FROM Papers P
         JOIN Escreveu_Paper EP ON P.id = EP.paper_id
         JOIN Autores A ON EP.autor_id = A.id
         WHERE A.nome =?;"
    ).await?;
    let mut papers = Vec::new();
    let mut rows = papers_stmt.query(&[&autor_name]).await?;
    while let Some(row) = rows.next().await? {
        papers.push(Paper::try_from(row)?);
    }

    Ok((livros, papers))
}

/// Creates vector indexes for Livros and Papers tables.
pub async fn create_vector_indexes(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_livros_embedding ON Livros(libsql_vector_idx(embedding));",
        (),
    ).await?;
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_papers_embedding ON Papers(libsql_vector_idx(embedding));",
        (),
    ).await?;
    Ok(())
}

/// Parses a vector string in the format "vector('[f1,f2,...]')" into a Vec<f32>.
pub fn parse_vector_string(input: &str) -> Result<Vec<f32>> {
    if!input.starts_with("vector('") ||!input.ends_with("')") {
        return Err(anyhow::anyhow!("Invalid vector string format: must be vector('[f1,f2,...]')"));
    }
    let json_str = &input[8..input.len() - 2]; // Extract "[f1,f2,...]"
    let vec: Vec<f32> = serde_json::from_str(json_str)?;
    Ok(vec)
}

/// Performs a similarity search for Livros using vector_distance_cos.
/// Returns books ordered by cosine distance (lower is more similar).
pub async fn query_livros_by_embedding(conn: &Connection, query_embedding_str: &str, limit: u32) -> Result<Vec<(Livro, f64)>> {
    let parsed_embedding = parse_vector_string(query_embedding_str)?;
    let embedding_json_str = f32_vec_to_json_string(&parsed_embedding);

    let sql = format!(
        "SELECT id, titulo, ano_publicacao, resumo, isbn, edicao, embedding,
                vector_distance_cos(embedding, vector(?)) AS distance
         FROM Livros
         ORDER BY distance ASC
         LIMIT?;"
    );
    let stmt = conn.prepare(&sql).await?;
    let mut rows = stmt.query(&[&embedding_json_str, &limit]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        let livro = Livro::try_from(row.clone())?; // Clone row to allow multiple gets
        let distance: f64 = row.get(7)?; // Assuming distance is the 8th column (index 7)
        results.push((livro, distance));
    }
    Ok(results)
}

/// Performs a similarity search for Papers using vector_distance_cos.
/// Returns papers ordered by cosine distance (lower is more similar).
pub async fn query_papers_by_embedding(conn: &Connection, query_embedding_str: &str, limit: u32) -> Result<Vec<(Paper, f64)>> {
    let parsed_embedding = parse_vector_string(query_embedding_str)?;
    let embedding_json_str = f32_vec_to_json_string(&parsed_embedding);

    let sql = format!(
        "SELECT id, titulo, ano_publicacao, resumo, doi, bibliografia, embedding,
                vector_distance_cos(embedding, vector(?)) AS distance
         FROM Papers
         ORDER BY distance ASC
         LIMIT?;"
    );
    let stmt = conn.prepare(&sql).await?;
    let mut rows = stmt.query(&[&embedding_json_str, &limit]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        let paper = Paper::try_from(row.clone())?;
        let distance: f64 = row.get(7)?;
        results.push((paper, distance));
    }
    Ok(results)
}

/// Performs an optimized similarity search for Livros using vector_top_k.
/// Returns the top K most similar books.
pub async fn query_livros_top_k(conn: &Connection, query_embedding_str: &str, k: u32) -> Result<Vec<(Livro, f64)>> {
    let parsed_embedding = parse_vector_string(query_embedding_str)?;
    let embedding_json_str = f32_vec_to_json_string(&parsed_embedding);

    // vector_top_k returns rowids. We then join to get full data and calculate distance for ordering.
    let sql = format!(
        "SELECT T.*, vector_distance_cos(T.embedding, vector(?)) AS distance
         FROM Livros AS T, vector_top_k('idx_livros_embedding', vector(?),?) AS K
         WHERE T.id = K.rowid
         ORDER BY distance ASC;"
    );
    let stmt = conn.prepare(&sql).await?;
    let mut rows = stmt.query(&[&embedding_json_str, &embedding_json_str, &k]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        let livro = Livro::try_from(row.clone())?;
        let distance: f64 = row.get(7)?;
        results.push((livro, distance));
    }
    Ok(results)
}

/// Performs an optimized similarity search for Papers using vector_top_k.
/// Returns the top K most similar papers.
pub async fn query_papers_top_k(conn: &Connection, query_embedding_str: &str, k: u32) -> Result<Vec<(Paper, f64)>> {
    let parsed_embedding = parse_vector_string(query_embedding_str)?;
    let embedding_json_str = f32_vec_to_json_string(&parsed_embedding);

    let sql = format!(
        "SELECT T.*, vector_distance_cos(T.embedding, vector(?)) AS distance
         FROM Papers AS T, vector_top_k('idx_papers_embedding', vector(?),?) AS K
         WHERE T.id = K.rowid
         ORDER BY distance ASC;"
    );
    let stmt = conn.prepare(&sql).await?;
    let mut rows = stmt.query(&[&embedding_json_str, &embedding_json_str, &k]).await?;

    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        let paper = Paper::try_from(row.clone())?;
        let distance: f64 = row.get(7)?;
        results.push((paper, distance));
    }
    Ok(results)
}
