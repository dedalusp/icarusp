use anyhow::Result;
use diesel::prelude::*;
use pgvector::{Vector, VectorExpressionMethods};
use serde::Serialize;

use crate::embedding::compute_embedding;
use crate::models::{Author, Book};
use crate::schema::{authors, books, books_authors};

#[derive(Serialize, Debug)]
pub struct AuthorOutput {
    pub id: i32,
    pub name: String,
    pub birth_year: i32,
    pub country: String,
}

impl From<Author> for AuthorOutput {
    fn from(author: Author) -> Self {
        AuthorOutput {
            id: author.id,
            name: author.name,
            birth_year: author.birth_year,
            country: author.country,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct BookOutput {
    pub id: i32,
    pub title: String,
    pub publication_year: i32,
    pub abstract_text: String,
}

impl From<Book> for BookOutput {
    fn from(book: Book) -> Self {
        BookOutput {
            id: book.id,
            title: book.title,
            publication_year: book.publication_year,
            abstract_text: book.abstract_text,
        }
    }
}

/// Retrieves authors by partial name match
pub fn get_authors_by_name(conn: &mut PgConnection, name: &str) -> Result<Vec<AuthorOutput>> {
    let pattern = format!("%{}%", name);

    let authors = authors::table
        .filter(authors::name.ilike(&pattern))
        .select(Author::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("Failed to query authors by name: {}", e))?;

    Ok(authors.into_iter().map(AuthorOutput::from).collect())
}

/// Retrieves books by partial title match
pub fn get_books_by_title(conn: &mut PgConnection, title: &str) -> Result<Vec<BookOutput>> {
    let pattern = format!("%{}%", title);

    let books = books::table
        .filter(books::title.ilike(&pattern))
        .select(Book::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("Failed to query books by title: {}", e))?;

    Ok(books.into_iter().map(BookOutput::from).collect())
}

/// Retrieves a specific author by ID
pub fn get_author_by_id(conn: &mut PgConnection, author_id: i32) -> Result<AuthorOutput> {
    let author = authors::table
        .find(author_id)
        .select(Author::as_select())
        .first(conn)
        .map_err(|e| anyhow::anyhow!("Author with ID {} not found: {}", author_id, e))?;

    Ok(AuthorOutput::from(author))
}

/// Retrieves a specific book by ID
pub fn get_book_by_id(conn: &mut PgConnection, book_id: i32) -> Result<BookOutput> {
    let book = books::table
        .find(book_id)
        .select(Book::as_select())
        .first(conn)
        .map_err(|e| anyhow::anyhow!("Book with ID {} not found: {}", book_id, e))?;

    Ok(BookOutput::from(book))
}

/// Retrieves all books written by a specific author (by author ID)
pub fn get_books_by_author_id(conn: &mut PgConnection, author_id: i32) -> Result<Vec<BookOutput>> {
    let books = books::table
        .inner_join(books_authors::table)
        .filter(books_authors::author_id.eq(author_id))
        .select(Book::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("Failed to query books by author ID: {}", e))?;

    Ok(books.into_iter().map(BookOutput::from).collect())
}

/// Retrieves all books written by a specific author (by author name, partial match)
pub fn get_books_by_author_name(
    conn: &mut PgConnection,
    author_name: &str,
) -> Result<Vec<BookOutput>> {
    let pattern = format!("%{}%", author_name);

    let books = books::table
        .inner_join(books_authors::table)
        .inner_join(authors::table.on(authors::id.eq(books_authors::author_id)))
        .filter(authors::name.ilike(&pattern))
        .select(Book::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("Failed to query books by author name: {}", e))?;

    Ok(books.into_iter().map(BookOutput::from).collect())
}

/// Retrieves all authors for a specific book (by book ID)
pub fn get_authors_by_book_id(conn: &mut PgConnection, book_id: i32) -> Result<Vec<AuthorOutput>> {
    let authors = authors::table
        .inner_join(books_authors::table)
        .filter(books_authors::book_id.eq(book_id))
        .select(Author::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("Failed to query authors by book ID: {}", e))?;

    Ok(authors.into_iter().map(AuthorOutput::from).collect())
}

/// Performs a similarity search for books using pgvector cosine distance
/// Returns books ordered by cosine distance (lower distance = more similar)
pub fn similarity_search_by_prompt(
    conn: &mut PgConnection,
    query_text: &str,
    limit: i32,
) -> Result<Vec<BookOutput>> {
    // Compute the embedding for the query text
    let query_embedding = compute_embedding(query_text)?;
    let embedding_vec = Vector::from(query_embedding);

    // Use pgvector expression methods for similarity search
    let similar_books = books::table
        .order(books::embedding.cosine_distance(&embedding_vec))
        .limit(limit as i64)
        .select(Book::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("Failed to execute vector similarity query: {}", e))?;

    Ok(similar_books.into_iter().map(BookOutput::from).collect())
}

/// Finds books with similar embeddings to a given book ID using cosine distance
pub fn find_similar_books(
    conn: &mut PgConnection,
    book_id: i32,
    limit: i32,
) -> Result<Vec<BookOutput>> {
    // Get the book information
    let target_book = get_book_by_id(conn, book_id)?;

    // Compute embedding for the book's abstract text
    let query_text = &target_book.abstract_text;

    // Use the existing similarity search function to find similar books
    similarity_search_by_prompt(conn, query_text, limit)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialization::establish_connection;
    use crate::database::insertion::{insert_author, insert_book, link_book_author};
    use crate::models::{NewAuthor, NewBook};

    // Helper function to check if database is available
    fn db_available() -> bool {
        std::env::var("DATABASE_URL").is_ok()
    }

    #[test]
    fn test_get_authors_by_name() {
        if !db_available() {
            return;
        }

        let mut conn = establish_connection().expect("Failed to connect to database");

        // Insert test data
        let new_author = NewAuthor::new("Test Query Author", 2025, "Brazil");
        let author = insert_author(&mut conn, &new_author).expect("Failed to insert author");

        // Test query
        let results = get_authors_by_name(&mut conn, "Query").expect("Failed to query authors");
        assert!(!results.is_empty());
        assert!(results.iter().any(|a| a.name.contains("Query")));

        // Cleanup
        diesel::delete(authors::table.filter(authors::id.eq(author.id)))
            .execute(&mut conn)
            .ok();
    }

    #[test]
    fn test_get_books_by_title() {
        if !db_available() {
            return;
        }

        let mut conn = establish_connection().expect("Failed to connect to database");

        // Insert test data
        let new_book = NewBook::new("Test Query Book", 2025, "A test book for querying")
            .expect("Failed to create book");
        let book = insert_book(&mut conn, &new_book).expect("Failed to insert book");

        // Test query
        let results = get_books_by_title(&mut conn, "Query").expect("Failed to query books");
        assert!(!results.is_empty());
        assert!(results.iter().any(|b| b.title.contains("Query")));

        // Cleanup
        diesel::delete(books::table.filter(books::id.eq(book.id)))
            .execute(&mut conn)
            .ok();
    }

    #[test]
    fn test_similarity_search_by_prompt() {
        if !db_available() {
            return;
        }

        let mut conn = establish_connection().expect("Failed to connect to database");

        // Insert test data
        let new_book = NewBook::new(
            "Similarity Embedding Test Book",
            2025,
            "This is a book about testing embeddings",
        )
        .expect("Failed to create book");
        let book = insert_book(&mut conn, &new_book).expect("Failed to insert book");

        // Print the book details
        println!("Book ID: {}", book.id);
        println!("Title: {}", book.title);
        println!("Year: {}", book.publication_year);
        println!("Description: {}", book.abstract_text);

        // Test embedding query
        let results = similarity_search_by_prompt(&mut conn, "similarity testing embeddings", 5)
            .expect("Failed to query books by embedding");

        // Should find at least our test book
        assert!(!results.is_empty());

        // Print the results
        for result in results {
            println!("Title: {}", result.title);
        }

        // Cleanup
        diesel::delete(books::table.filter(books::id.eq(book.id)))
            .execute(&mut conn)
            .ok();

        println!("Similarity search by prompt test completed successfully!");
    }
}
