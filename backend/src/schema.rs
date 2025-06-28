// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    authors (id) {
        id -> Int4,
        name -> Varchar,
        birth_year -> Int4,
        country -> Varchar,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    books (id) {
        id -> Int4,
        title -> Varchar,
        publication_year -> Int4,
        abstract_text -> Varchar,
        embedding -> Nullable<Vector>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    books_authors (book_id, author_id) {
        book_id -> Int4,
        author_id -> Int4,
    }
}

diesel::joinable!(books_authors -> authors (author_id));
diesel::joinable!(books_authors -> books (book_id));

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors,
);
