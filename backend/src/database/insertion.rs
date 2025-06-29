use anyhow::Result;
use diesel::prelude::*;

use crate::models::{Author, Book, BookAuthor, NewAuthor, NewBook, NewBookAuthor};
use crate::schema::{authors, books, books_authors};

/// Inserts a new Author into the database and returns the created Author
pub fn insert_author(conn: &mut PgConnection, new_author: &NewAuthor) -> Result<Author> {
    let author = diesel::insert_into(authors::table)
        .values(new_author)
        .returning(Author::as_returning())
        .get_result(conn)
        .map_err(|e| anyhow::anyhow!("Failed to insert author: {}", e))?;

    Ok(author)
}

/// Inserts a new Book into the database and returns the created Book
pub fn insert_book(conn: &mut PgConnection, new_book: &NewBook) -> Result<Book> {
    let book = diesel::insert_into(books::table)
        .values(new_book)
        .returning(Book::as_returning())
        .get_result(conn)
        .map_err(|e| anyhow::anyhow!("Failed to insert book: {}", e))?;

    Ok(book)
}

/// Links an author to a book in the books_authors junction table
pub fn link_book_author(
    conn: &mut PgConnection,
    book_id: i32,
    author_id: i32,
) -> Result<BookAuthor> {
    let new_book_author = NewBookAuthor::new(book_id, author_id);

    let book_author = diesel::insert_into(books_authors::table)
        .values(&new_book_author)
        .returning(BookAuthor::as_returning())
        .get_result(conn)
        .map_err(|e| anyhow::anyhow!("Failed to link book and author: {}", e))?;

    Ok(book_author)
}

/// Links a book to multiple authors by their IDs
/// Returns a vector of created BookAuthor relationships
pub fn link_book_to_authors(
    conn: &mut PgConnection,
    book_id: i32,
    authors_ids: &[i32],
) -> Result<Vec<BookAuthor>> {
    // First, verify all authors exist
    let existing_authors = authors::table
        .filter(authors::id.eq_any(authors_ids))
        .select(Author::as_select())
        .load(conn)
        .map_err(|e| anyhow::anyhow!("Failed to verify authors: {}", e))?;

    if existing_authors.len() != authors_ids.len() {
        let existing_ids: Vec<i32> = existing_authors.iter().map(|a| a.id).collect();
        let missing_ids: Vec<i32> = authors_ids
            .iter()
            .filter(|&id| !existing_ids.contains(id))
            .cloned()
            .collect();
        return Err(anyhow::anyhow!(
            "Authors with IDs {:?} not found",
            missing_ids
        ));
    }

    // Create BookAuthor relationships for each author
    let mut book_authors = Vec::new();
    for &author_id in authors_ids {
        let book_author = link_book_author(conn, book_id, author_id)?;
        book_authors.push(book_author);
    }

    Ok(book_authors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::initialization::establish_connection;
    use crate::models::NewBook;

    // Helper function to check if database is available
    fn db_available() -> bool {
        std::env::var("DATABASE_URL").is_ok()
    }

    #[test]
    fn test_insert_author() {
        if !db_available() {
            return;
        }

        let mut conn = establish_connection().expect("Failed to connect to database");
        let new_author = NewAuthor::new("Test Author", 2025, "Brazil");

        let result = insert_author(&mut conn, &new_author);
        assert!(result.is_ok());

        let author = result.unwrap();
        assert_eq!(author.name, "Test Author");
        assert_eq!(author.birth_year, 2025);
        assert_eq!(author.country, "Brazil");
        assert!(author.id > 0);

        // Cleanup
        diesel::delete(authors::table.filter(authors::id.eq(author.id)))
            .execute(&mut conn)
            .ok();
    }

    #[test]
    fn test_insert_book() {
        if !db_available() {
            return;
        }

        let mut conn = establish_connection().expect("Failed to connect to database");
        let new_book =
            NewBook::new("Test Book", 2025, "A test book summary").expect("Failed to create book");

        let result = insert_book(&mut conn, &new_book);
        assert!(result.is_ok());

        let book = result.unwrap();
        assert_eq!(book.title, "Test Book");
        assert_eq!(book.publication_year, 2025);
        assert_eq!(book.abstract_text, "A test book summary");
        assert!(book.id > 0);
        assert!(book.embedding.is_some());

        // Cleanup
        diesel::delete(books::table.filter(books::id.eq(book.id)))
            .execute(&mut conn)
            .ok();
    }

    #[test]
    fn test_link_book_author() {
        if !db_available() {
            return;
        }

        let mut conn = establish_connection().expect("Failed to connect to database");

        // Insert two authors
        let author1 = insert_author(&mut conn, &NewAuthor::new("Author One", 1980, "USA"))
            .expect("Failed to insert author1");
        let author2 = insert_author(&mut conn, &NewAuthor::new("Author Two", 1990, "UK"))
            .expect("Failed to insert author2");

        // Insert a book
        let new_book = NewBook::new("Book With Two Authors", 2024, "Summary").unwrap();
        let book = insert_book(&mut conn, &new_book).expect("Failed to insert book");

        // Link book to both authors
        let result = link_book_to_authors(&mut conn, book.id, &[author1.id, author2.id]);
        assert!(result.is_ok());
        let book_authors = result.unwrap();
        assert_eq!(book_authors.len(), 2);
        let author_ids: Vec<i32> = book_authors.iter().map(|ba| ba.author_id).collect();
        assert!(author_ids.contains(&author1.id));
        assert!(author_ids.contains(&author2.id));

        // Cleanup
        diesel::delete(books_authors::table.filter(books_authors::book_id.eq(book.id)))
            .execute(&mut conn)
            .ok();
        diesel::delete(books::table.filter(books::id.eq(book.id)))
            .execute(&mut conn)
            .ok();
        diesel::delete(authors::table.filter(authors::id.eq(author1.id)))
            .execute(&mut conn)
            .ok();
        diesel::delete(authors::table.filter(authors::id.eq(author2.id)))
            .execute(&mut conn)
            .ok();
    }
}
