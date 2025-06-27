use anyhow::Result;
use sqlx::PgPool;

/// Hardcoded constant for the vector dimension.
const VECTOR_DIMENSION: i32 = 512;

/// Initializes a PostgreSQL database connection pool.
pub async fn initialize_db_connection(database_url: &str) -> Result<PgPool> {
    let pool = PgPool::connect(database_url).await?;
    Ok(pool)
}

/// Enables the pgvector extension in the database.
pub async fn enable_pgvector_extension(pool: &PgPool) -> Result<()> {
    sqlx::query("CREATE EXTENSION IF NOT EXISTS vector;")
        .execute(pool)
        .await?;
    Ok(())
}

/// Creates the Autores table with nome as the primary key.
pub async fn create_autores_table(pool: &PgPool) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS Autores (
            nome TEXT PRIMARY KEY,
            ano_nascimento INTEGER,
            pais TEXT
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Creates the Publicacoes table with titulo as the primary key.
pub async fn create_publicacoes_table(pool: &PgPool, vector_dimension: i32) -> Result<()> {
    let query = format!(
        "CREATE TABLE IF NOT EXISTS Publicacoes (
            titulo TEXT PRIMARY KEY,
            ano_publicacao INTEGER,
            resumo TEXT,
            embedding vector({}) NOT NULL
        );",
        vector_dimension
    );

    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

/// Creates the Escreveu_Publicacao table with foreign key constraints.
pub async fn create_escreveu_publicacao_table(pool: &PgPool) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS Escreveu_Publicacao (
            autor_nome TEXT NOT NULL,
            publicacao_titulo TEXT NOT NULL,
            PRIMARY KEY (autor_nome, publicacao_titulo),
            FOREIGN KEY (autor_nome) REFERENCES Autores(nome) ON DELETE CASCADE,
            FOREIGN KEY (publicacao_titulo) REFERENCES Publicacoes(titulo) ON DELETE CASCADE
        );",
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Creates vector indexes for the Publicacoes table using pgvector.
pub async fn create_vector_indexes(pool: &PgPool) -> Result<()> {
    // Create HNSW index for cosine distance on embeddings
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_publicacoes_embedding_cosine
         ON Publicacoes USING hnsw (embedding vector_cosine_ops);",
    )
    .execute(pool)
    .await?;

    // Create IVFFlat index for L2 distance as well (optional)
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_publicacoes_embedding_l2
         ON Publicacoes USING ivfflat (embedding vector_l2_ops) WITH (lists = 100);",
    )
    .execute(pool)
    .await?;

    Ok(())
}

/// Initializes all tables by calling each table creation function.
pub async fn init_tables(pool: &PgPool) -> Result<()> {
    enable_pgvector_extension(pool).await?;
    create_autores_table(pool).await?;
    create_publicacoes_table(pool, VECTOR_DIMENSION).await?;
    create_escreveu_publicacao_table(pool).await?;
    create_vector_indexes(pool).await?;
    Ok(())
}
