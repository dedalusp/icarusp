use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{Author, Book};
use crate::schema::books_authors;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Clone, PartialEq, Serialize)]
#[diesel(belongs_to(Book))]
#[diesel(belongs_to(Author))]
#[diesel(table_name = books_authors)]
#[diesel(primary_key(book_id, author_id))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = books_authors)]
pub struct NewBookAuthor {
    pub book_id: i32,
    pub author_id: i32,
}

impl NewBookAuthor {
    pub fn new(book_id: i32, author_id: i32) -> Self {
        NewBookAuthor { book_id, author_id }
    }
}

impl BookAuthor {
    pub fn get_book_id(&self) -> i32 {
        self.book_id
    }

    pub fn get_author_id(&self) -> i32 {
        self.author_id
    }
}

impl std::fmt::Display for BookAuthor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "BookAuthor: Book ID {} - Author ID {}",
            self.book_id, self.author_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_book_author_creation() {
        let new_book_author = NewBookAuthor::new(1, 2);
        assert_eq!(new_book_author.book_id, 1);
        assert_eq!(new_book_author.author_id, 2);
    }

    #[test]
    fn test_book_author_display() {
        let book_author = BookAuthor {
            book_id: 1,
            author_id: 2,
        };
        let display_str = format!("{}", book_author);
        assert_eq!(display_str, "BookAuthor: Book ID 1 - Author ID 2");
    }
}
