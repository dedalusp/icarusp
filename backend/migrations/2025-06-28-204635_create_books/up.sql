CREATE TABLE books (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL UNIQUE,
  publication_year INTEGER NOT NULL,
  abstract_text VARCHAR NOT NULL,
  embedding VECTOR(512)
);
