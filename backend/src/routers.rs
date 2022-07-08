// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{middleware, web, App, HttpServer, Responder};

use crate::db::get_connection_pool;
use crate::error::Error;
use crate::views::{authors, books, comments, file_formats, publishers, ratings, tags};

const CONTENT_TYPE: &str = "content-type";
const APPLICATION_JSON: &str = "application/json";

async fn index() -> impl Responder {
    "Hello, world"
}

pub async fn run() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = get_connection_pool()?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .route("/", web::get().to(index))
            .route("/api/author", web::post().to(authors::add_author))
            .route(
                "/api/author/stored/{author_id}",
                web::get().to(books::get_books_by_author),
            )
            .route("/api/author", web::get().to(authors::get_authors))
            .route("/api/book", web::post().to(books::add_book))
            .route("/api/book", web::get().to(books::get_books))
            .route("/api/book/{book_id}", web::get().to(books::get_book_detail))
            .route("/api/comment", web::post().to(comments::add_comment))
            .service(
                web::resource("/api/comment/{book_id}")
                    .route(web::get().to(comments::get_comment))
                    .route(web::put().to(comments::update_comment))
                    .route(web::delete().to(comments::delete_comment)),
            )
            .route(
                "/api/formats/stored/{format_id}",
                web::get().to(books::get_books_by_format),
            )
            .route("/api/formats", web::get().to(file_formats::get_formats))
            .route("/api/publisher", web::post().to(publishers::add_publisher))
            .route(
                "/api/publisher/stored/{publisher_id}",
                web::get().to(books::get_books_by_publisher),
            )
            .route("/api/publisher", web::get().to(publishers::get_publishers))
            .route("/api/rating", web::post().to(ratings::add_rating))
            .service(
                web::resource("/api/rating/{book_id}")
                    .route(web::get().to(ratings::get_ratings))
                    .route(web::put().to(ratings::update_rating))
                    .route(web::delete().to(ratings::delete_rating)),
            )
            .route("/api/tag", web::post().to(tags::add_tag))
            .route(
                "/api/tag/stored/{tag_id}",
                web::get().to(books::get_books_by_tag),
            )
            .route("/api/tag", web::get().to(tags::get_tags))
            .route("/api/tag/{tag_id}", web::put().to(tags::update_tag))
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
    .map_err(Into::into)
}
