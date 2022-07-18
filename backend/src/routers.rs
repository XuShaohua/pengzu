// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::dev::ServiceRequest;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_grants::permissions::{AttachPermissions, AuthDetails};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::db::get_connection_pool;
use crate::error::Error;
use crate::views::auth::{Claims, UserPermissions};
use crate::views::{
    authors, books, comments, file_formats, files, publishers, ratings, series, tags, users,
};

const CONTENT_TYPE: &str = "content-type";
const APPLICATION_JSON: &str = "application/json";

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::Error> {
    // We just get permissions from JWT
    let claims = Claims::decode(credentials.token())?;
    log::info!("validator() claims: {:?}", claims);
    req.attach(vec![claims.permission()]);
    Ok(req)
}

async fn index(detail: AuthDetails<UserPermissions>) -> String {
    let permissions = &detail.permissions;
    log::info!("permissions: {:?}", permissions);
    "Hello, world".to_string()
}

pub async fn run() -> Result<(), Error> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = get_connection_pool()?;

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/api/hello")
                    .wrap(auth)
                    .route(web::get().to(index)),
            )
            // For /api/login
            .route("/api/login", web::post().to(users::login))
            // For /api/author
            .route("/api/author", web::post().to(authors::add_author))
            .route(
                "/api/author/books/{author_id}",
                web::get().to(books::get_books_by_author),
            )
            .route("/api/author", web::get().to(authors::get_authors))
            // For /api/book
            .route("/api/book", web::post().to(books::add_book))
            .route("/api/book", web::get().to(books::get_books))
            .route("/api/book/{book_id}", web::get().to(books::get_book_detail))
            // For /api/comment
            .route("/api/comment", web::post().to(comments::add_comment))
            .service(
                web::resource("/api/comment/{book_id}")
                    .route(web::get().to(comments::get_comment))
                    .route(web::put().to(comments::update_comment))
                    .route(web::delete().to(comments::delete_comment)),
            )
            // For /api/file
            .route("/api/file", web::get().to(files::get_file_by_path))
            // For /api/formats
            .route(
                "/api/format/books/{format_id}",
                web::get().to(books::get_books_by_format),
            )
            .route("/api/format", web::get().to(file_formats::get_formats))
            // For /api/publisher
            .route("/api/publisher", web::post().to(publishers::add_publisher))
            .route(
                "/api/publisher/books/{publisher_id}",
                web::get().to(books::get_books_by_publisher),
            )
            .route("/api/publisher", web::get().to(publishers::get_publishers))
            // For /api/rating
            .route("/api/rating", web::post().to(ratings::add_rating))
            .service(
                web::resource("/api/rating/{book_id}")
                    .route(web::get().to(ratings::get_ratings))
                    .route(web::put().to(ratings::update_rating))
                    .route(web::delete().to(ratings::delete_rating)),
            )
            // For /api/series
            .route("/api/series", web::post().to(series::add_series))
            .route(
                "/api/series/books/{series_id}",
                web::get().to(books::get_books_by_series),
            )
            .route("/api/series", web::get().to(series::get_series))
            // For /api/tag
            .route("/api/tag", web::post().to(tags::add_tag))
            .route(
                "/api/tag/books/{tag_id}",
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
