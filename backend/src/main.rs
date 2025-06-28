use actix_cors::Cors;
use actix_web::{App, HttpResponse, HttpServer, Responder, Result, middleware::Logger, post, web};
use serde::{Deserialize, Serialize};

mod database;
mod embedding;
mod models;
mod schema;

use database::initialization::establish_connection;
use database::insertion::{insert_author, insert_book, link_book_to_authors};
use database::query::{
    get_authors_by_name, get_books_by_author_name, get_books_by_title, similarity_search_by_prompt,
};
use models::{BookResponse, NewAuthor, NewBook};

#[derive(Deserialize)]
struct CreateAuthorRequest {
    name: String,
    birth_year: i32,
    country: String,
}

#[derive(Deserialize)]
struct CreateBookRequest {
    title: String,
    publication_year: i32,
    abstract_text: String,
}

#[derive(Deserialize)]
struct CreateBookAuthorsLinkRequest {
    book_id: i32,
    authors_ids: Vec<i32>,
}

#[derive(Deserialize)]
struct SearchAuthorsRequest {
    name: String,
}

#[derive(Deserialize)]
struct SearchBooksRequest {
    title: String,
}

#[derive(Deserialize)]
struct SearchBooksByAuthorRequest {
    author_name: String,
}

#[derive(Deserialize)]
struct EmbeddingSearchRequest {
    query: String,
    limit: Option<i32>,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    message: String,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        ApiResponse {
            success: true,
            data: Some(data),
            message: "Success".to_string(),
        }
    }

    fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message,
        }
    }
}

#[post("/insert/author")]
async fn create_author(req: web::Json<CreateAuthorRequest>) -> Result<impl Responder> {
    let mut connection = match establish_connection() {
        // Correctly handle the Result
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to establish database connection: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to connect to database: {}",
                    e
                ))),
            );
        }
    };
    let new_author = NewAuthor::new(&req.name, req.birth_year, &req.country);

    match insert_author(&mut connection, &new_author) {
        // Pass mutable reference
        Ok(author) => Ok(HttpResponse::Created().json(ApiResponse::success(author))),
        Err(e) => {
            log::error!("Failed to create author: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to create author: {}",
                    e
                ))),
            )
        }
    }
}

#[post("/insert/book")]
async fn create_book(req: web::Json<CreateBookRequest>) -> Result<impl Responder> {
    let mut connection = match establish_connection() {
        // Correctly handle the Result
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to establish database connection: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to connect to database: {}",
                    e
                ))),
            );
        }
    };

    let new_book = match NewBook::new(&req.title, req.publication_year, &req.abstract_text) {
        Ok(book) => book,
        Err(e) => {
            log::error!("Failed to create book: {}", e);
            return Ok(
                HttpResponse::BadRequest().json(ApiResponse::<()>::error(format!(
                    "Failed to create book: {}",
                    e
                ))),
            );
        }
    };

    match insert_book(&mut connection, &new_book) {
        // Pass mutable reference
        Ok(book) => {
            Ok(HttpResponse::Created().json(ApiResponse::success(BookResponse::from(book))))
        }
        Err(e) => {
            log::error!("Failed to create book with author: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to create book with author: {}",
                    e
                ))),
            )
        }
    }
}

#[post("/insert/book-author-link")]
async fn create_book_author_link(
    req: web::Json<CreateBookAuthorsLinkRequest>,
) -> Result<impl Responder> {
    let mut connection = match establish_connection() {
        // Correctly handle the Result
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to establish database connection: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to connect to database: {}",
                    e
                ))),
            );
        }
    };

    match link_book_to_authors(&mut connection, req.book_id, &req.authors_ids) {
        // Pass mutable reference
        Ok(book_authors) => Ok(HttpResponse::Created().json(ApiResponse::success(book_authors))),
        Err(e) => {
            log::error!("Failed to link book to authors: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to link book to authors: {}",
                    e
                ))),
            )
        }
    }
}

#[post("/search/authors")]
async fn search_authors(req: web::Json<SearchAuthorsRequest>) -> Result<impl Responder> {
    let mut connection = match establish_connection() {
        // Correctly handle the Result
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to establish database connection: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to connect to database: {}",
                    e
                ))),
            );
        }
    };

    match get_authors_by_name(&mut connection, &req.name) {
        // Pass mutable reference
        Ok(authors) => Ok(HttpResponse::Ok().json(ApiResponse::success(authors))),
        Err(e) => {
            log::error!("Failed to search authors: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to search authors: {}",
                    e
                ))),
            )
        }
    }
}

#[post("/search/books")]
async fn search_books(req: web::Json<SearchBooksRequest>) -> Result<impl Responder> {
    let mut connection = match establish_connection() {
        // Correctly handle the Result
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to establish database connection: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to connect to database: {}",
                    e
                ))),
            );
        }
    };

    match get_books_by_title(&mut connection, &req.title) {
        // Pass mutable reference
        Ok(books) => Ok(HttpResponse::Ok().json(ApiResponse::success(books))),
        Err(e) => {
            log::error!("Failed to search books: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to search books: {}",
                    e
                ))),
            )
        }
    }
}

#[post("/search/author/books")]
async fn search_books_by_author(
    req: web::Json<SearchBooksByAuthorRequest>,
) -> Result<impl Responder> {
    let mut connection = match establish_connection() {
        // Correctly handle the Result
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to establish database connection: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to connect to database: {}",
                    e
                ))),
            );
        }
    };

    match get_books_by_author_name(&mut connection, &req.author_name) {
        // Pass mutable reference
        Ok(books) => Ok(HttpResponse::Ok().json(ApiResponse::success(books))),
        Err(e) => {
            log::error!("Failed to search books by author: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to search books by author: {}",
                    e
                ))),
            )
        }
    }
}

#[post("/search/book/embedding")]
async fn search_books_by_embedding(
    req: web::Json<EmbeddingSearchRequest>,
) -> Result<impl Responder> {
    let mut connection = match establish_connection() {
        // Correctly handle the Result
        Ok(conn) => conn,
        Err(e) => {
            log::error!("Failed to establish database connection: {}", e);
            return Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to connect to database: {}",
                    e
                ))),
            );
        }
    };

    let limit = req.limit.unwrap_or(10);
    if limit <= 0 || limit > 100 {
        return Ok(HttpResponse::BadRequest().json(ApiResponse::<()>::error(
            "Limit must be between 1 and 100".to_string(),
        )));
    }

    match similarity_search_by_prompt(&mut connection, &req.query, limit) {
        // Pass mutable reference
        Ok(books) => Ok(HttpResponse::Ok().json(ApiResponse::success(books))),
        Err(e) => {
            log::error!("Failed to search books by embedding: {}", e);
            Ok(
                HttpResponse::InternalServerError().json(ApiResponse::<()>::error(format!(
                    "Failed to search books by embedding: {}",
                    e
                ))),
            )
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Test database connection
    match database::initialization::establish_connection() {
        Ok(_) => log::info!("Database connection test successful"),
        Err(e) => {
            log::error!("Database connection test failed: {}", e);
            std::process::exit(1);
        }
    }

    let bind_address = "127.0.0.1:8080";
    log::info!("Starting server at http://{}", bind_address);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .service(create_author)
            .service(create_book)
            .service(create_book_author_link)
            .service(search_authors)
            .service(search_books)
            .service(search_books_by_author)
            .service(search_books_by_embedding)
    })
    .bind(bind_address)?
    .run()
    .await
}
