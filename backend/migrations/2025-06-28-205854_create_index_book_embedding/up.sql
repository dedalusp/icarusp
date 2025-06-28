CREATE INDEX book_embedding_cosine_index ON books USING hnsw (embedding vector_cosine_ops)
