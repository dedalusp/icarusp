use anyhow::Result;
use diesel::prelude::*;
use pgvector::Vector;
use serde::Serialize;

use crate::embedding::compute_embedding;
use crate::schema::books;

#[derive(Queryable, Selectable, Identifiable, PartialEq, Debug, Clone)]
#[diesel(table_name = books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub publication_year: i32,
    pub abstract_text: String,
    pub embedding: Option<Vector>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub publication_year: i32,
    pub abstract_text: String,
    pub embedding: Option<Vector>,
}

impl NewBook {
    pub fn new(title: &str, publication_year: i32, abstract_text: &str) -> Result<Self> {
        let embedding_vec = compute_embedding(abstract_text)?;
        let embedding = Vector::from(embedding_vec);

        Ok(NewBook {
            title: title.to_string(),
            publication_year: publication_year,
            abstract_text: abstract_text.to_string(),
            embedding: Some(embedding),
        })
    }
}

impl Book {
    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_publication_year(&self) -> i32 {
        self.publication_year
    }

    pub fn get_abstract(&self) -> &str {
        &self.abstract_text
    }

    pub fn get_embedding(&self) -> Option<&Vector> {
        self.embedding.as_ref()
    }

    pub fn has_embedding(&self) -> bool {
        self.embedding.is_some()
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Book: {} (ID: {}, Has Embedding: {})",
            self.title,
            self.id,
            self.has_embedding()
        )
    }
}

#[derive(Serialize)]
pub struct BookResponse {
    id: i32,
    title: String,
    publication_year: i32,
    abstract_text: String,
}

impl From<Book> for BookResponse {
    fn from(book: Book) -> Self {
        BookResponse {
            id: book.get_id(),
            title: book.get_title().to_string(),
            publication_year: book.get_publication_year(),
            abstract_text: book.abstract_text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_book_creation() {
        let new_book = NewBook::new(
            "Dom Casmurro",
            1899,
            "Um clássico da literatura brasileira.",
        )
        .expect("Failed to create new book");

        assert_eq!(new_book.title, "Dom Casmurro");
        assert_eq!(new_book.publication_year, 1899);
        assert_eq!(
            new_book.abstract_text,
            "Um clássico da literatura brasileira."
        );
        assert!(new_book.embedding.is_some());
    }

    #[test]
    fn test_book_display() {
        let new_book = NewBook::new(
            "Dom Casmurro",
            1899,
            "Um clássico da literatura brasileira.",
        )
        .expect("Failed to create new book");
        let book = Book {
            id: 1,
            title: new_book.title,
            publication_year: new_book.publication_year,
            abstract_text: new_book.abstract_text,
            embedding: new_book.embedding,
        };
        let display_str = format!("{}", book);
        // Note: The original test expected "Has Embedding: false" but the NewBook constructor sets it to Some(embedding), so it should be true.
        assert_eq!(
            display_str,
            "Book: Dom Casmurro (ID: 1, Has Embedding: true)"
        );
    }

    #[test]
    fn test_book_response_conversion() {
        let new_book = NewBook::new(
            "Dom Casmurro",
            1899,
            "Um clássico da literatura brasileira.",
        )
        .expect("Failed to create new book");
        let book = Book {
            id: 1,
            title: new_book.title,
            publication_year: new_book.publication_year,
            abstract_text: new_book.abstract_text,
            embedding: new_book.embedding,
        };
        let book_response = BookResponse::from(book);
        assert_eq!(book_response.id, 1);
        assert_eq!(book_response.title, "Dom Casmurro");
        assert_eq!(book_response.publication_year, 1899);
        assert_eq!(
            book_response.abstract_text,
            "Um clássico da literatura brasileira."
        );
    }
}
