pub mod author;
pub mod book;
pub mod book_author;

pub use author::{Author, NewAuthor};
pub use book::{Book, BookResponse, NewBook};
pub use book_author::{BookAuthor, NewBookAuthor};
