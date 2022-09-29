// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{middleware, web, App, HttpServer};
use actix_web_grants::permissions::AuthDetails;
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::db::get_connection_pool;
use crate::error::Error;
use crate::views::auth::{auth_validator, UserPermissions};
use crate::views::{
    authors, books, categories, comments, file_formats, files, publishers, ratings, series, tags,
    user_tags, users,
};

const CONTENT_TYPE: &str = "content-type";
const APPLICATION_JSON: &str = "application/json";

pub async fn run() -> Result<(), Error> {
    let pool = get_connection_pool()?;

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth_validator);
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            // For /api/author
            .service(
                web::resource("/api/author")
                    .wrap(auth.clone())
                    .route(web::get().to(authors::get_authors))
                    .route(web::post().to(authors::add_author)),
            )
            .service(
                web::resource("/api/author/books/{author_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books_by_author)),
            )
            // For /api/book
            .service(
                web::resource("/api/book")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books))
                    .route(web::post().to(books::add_book)),
            )
            .service(
                web::resource("/api/book/{book_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_book_detail)),
            )
            // For /api/categories
            .service(
                web::resource("/api/category")
                    .wrap(auth.clone())
                    .route(web::get().to(categories::get_categories)),
            )
            .service(
                web::resource("/api/category/books/{category_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books_by_category)),
            )
            // For /api/comment
            .service(
                web::resource("/api/comment")
                    .wrap(auth.clone())
                    .route(web::post().to(comments::add_comment)),
            )
            .service(
                web::resource("/api/comment/{book_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(comments::get_comment))
                    .route(web::put().to(comments::update_comment))
                    .route(web::delete().to(comments::delete_comment)),
            )
            // For /api/file
            .service(web::resource("/api/file").route(web::get().to(files::get_file_by_path)))
            // For /api/formats
            .service(
                web::resource("/api/format/books/{format_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books_by_format)),
            )
            .service(
                web::resource("/api/format")
                    .wrap(auth.clone())
                    .route(web::get().to(file_formats::get_formats)),
            )
            // For /api/publisher
            .service(
                web::resource("/api/publisher")
                    .wrap(auth.clone())
                    .route(web::get().to(publishers::get_publishers))
                    .route(web::post().to(publishers::add_publisher)),
            )
            .service(
                web::resource("/api/publisher/books/{publisher_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books_by_publisher)),
            )
            // For /api/rating
            .service(
                web::resource("/api/rating")
                    .wrap(auth.clone())
                    .route(web::post().to(ratings::add_rating)),
            )
            .service(
                web::resource("/api/rating/{book_id}")
                    .route(web::get().to(ratings::get_ratings))
                    .route(web::put().to(ratings::update_rating))
                    .route(web::delete().to(ratings::delete_rating)),
            )
            // For /api/series
            .service(
                web::resource("/api/series")
                    .wrap(auth.clone())
                    .route(web::get().to(series::get_series))
                    .route(web::post().to(series::add_series)),
            )
            .service(
                web::resource("/api/series/books/{series_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books_by_series)),
            )
            // For /api/tag
            .service(
                web::resource("/api/tag")
                    .wrap(auth.clone())
                    .route(web::get().to(tags::get_tags))
                    .route(web::post().to(tags::add_tag)),
            )
            .service(
                web::resource("/api/tag/books/{tag_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books_by_tag)),
            )
            .service(
                web::resource("/api/tag/{tag_id}")
                    .wrap(auth.clone())
                    .route(web::put().to(tags::update_tag)),
            )
            // For /api/user-tag
            .service(
                web::resource("/api/user-tag")
                    .wrap(auth.clone())
                    .route(web::get().to(user_tags::get_tags))
                    .route(web::post().to(user_tags::add_tag)),
            )
            .service(
                web::resource("/api/user-tag/books/{tag_id}")
                    .wrap(auth.clone())
                    .route(web::get().to(books::get_books_by_user_tag)),
            )
            // For /api/user
            .route("/api/user/login", web::post().to(users::login))
            .service(
                web::resource("/api/user")
                    .wrap(auth.clone())
                    .route(web::get().to(users::get_user_info)),
            )
            // For /api/users
            .service(
                web::resource("/api/users")
                    .wrap(auth.clone())
                    .route(web::get().to(users::get_users))
                    .route(web::post().to(users::add_user)),
            )
            .service(
                web::resource("/api/users/{user_id}")
                    .wrap(auth)
                    .route(web::delete().to(users::delete_user)),
            )
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
    .map_err(Into::into)
}
