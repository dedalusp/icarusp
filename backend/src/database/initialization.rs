use limbo::{Builder, Connection};
use anyhow::Result;

/// Hardcoded constant for the vector dimension.
const VECTOR_DIMENSION: u32 = 512;

/// Initializes a local Limbo database connection.
/// If the database file does not exist, it will be created.
pub async fn initialize_db_connection(db_path: &str) -> Result<Connection> {
    let db = Builder::new_local(db_path).build().await?;
    let conn = db.connect()?;
    Ok(conn)
}

/// Creates the Autores table with UUID as the primary key.
pub async fn create_autores_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Autores (
            id UUID PRIMARY KEY,
            nome TEXT NOT NULL UNIQUE,
            ano_nascimento INTEGER,
            pais TEXT
        );",
        (),
    ).await?;
    Ok(())
}

/// Creates the Publicacoes table with UUID as the primary key and embedding as a vector.
pub async fn create_publicacoes_table(conn: &Connection, vector_dimension: u32) -> Result<()> {
    conn.execute(
        &format!("CREATE TABLE IF NOT EXISTS Publicacoes (
            id UUID PRIMARY KEY,
            titulo TEXT NOT NULL UNIQUE,
            ano_publicacao INTEGER,
            resumo TEXT,
            embedding VECTOR({}) NOT NULL
        );", vector_dimension),
        (),
    ).await?;
    Ok(())
}

/// Creates the Escreveu_Publicacao table with foreign key constraints.
pub async fn create_escreveu_publicacao_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS Escreveu_Publicacao (
            autor_id UUID NOT NULL,
            publicacao_id UUID NOT NULL,
            PRIMARY KEY (autor_id, publicacao_id),
            FOREIGN KEY (autor_id) REFERENCES Autores(id) ON DELETE CASCADE,
            FOREIGN KEY (publicacao_id) REFERENCES Publicacoes(id) ON DELETE CASCADE
        );",
        (),
    ).await?;
    Ok(())
}

/// Creates vector indexes for the Publicacoes table.
pub async fn create_vector_indexes(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_publicacoes_embedding ON Publicacoes(vector_distance_cos(embedding));",
        (),
    ).await?;
    Ok(())
}

/// Initializes all tables by calling each table creation function.
pub async fn init_tables(conn: &Connection) -> Result<()> {
    create_autores_table(conn).await?;
    create_publicacoes_table(conn, VECTOR_DIMENSION).await?;
    create_escreveu_publicacao_table(conn).await?;
    create_vector_indexes(conn).await?;
    Ok(())
}


