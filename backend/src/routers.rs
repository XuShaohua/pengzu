// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use crate::db;
use crate::error::Error;
use crate::views::auth::{admin_guard, auth_validator};
use crate::views::{
    advanced_search, authors, books, categories, comments, discover, download_history,
    file_formats, files, images, publishers, ratings, series, simple_search, tags, user_tags,
    users,
};

const CONTENT_TYPE: &str = "content-type";
const APPLICATION_JSON: &str = "application/json";

#[allow(clippy::too_many_lines)]
fn scoped_config(cfg: &mut web::ServiceConfig) {
    let auth = HttpAuthentication::bearer(auth_validator);

    cfg
        // For /api/author
        .service(
            web::resource("/author")
                .wrap(auth.clone())
                .route(web::get().to(authors::get_authors))
                .route(web::post().guard(admin_guard()).to(authors::add_author)),
        )
        .service(
            web::resource("/author/{author_id}")
                .wrap(auth.clone())
                .route(web::get().to(authors::get_author)),
        )
        .service(
            web::resource("/author/books/{author_id}")
                .wrap(auth.clone())
                .route(web::get().to(authors::get_books_by_author)),
        )
        // For /api/book
        .service(
            web::resource("/book")
                .wrap(auth.clone())
                .route(web::get().to(books::get_books))
                .route(web::post().guard(admin_guard()).to(books::add_book)),
        )
        .service(
            web::resource("/book/{book_id}")
                .wrap(auth.clone())
                .route(web::get().to(books::get_book_detail))
                .route(web::put().guard(admin_guard()).to(books::update_book)),
        )
        // For /api/categories
        .service(
            web::resource("/category")
                .wrap(auth.clone())
                .route(web::get().to(categories::get_categories)),
        )
        .service(
            web::resource("/category/{category_id}")
                .wrap(auth.clone())
                .route(web::get().to(categories::get_category)),
        )
        .service(
            web::resource("/category/books/{category_id}")
                .wrap(auth.clone())
                .route(web::get().to(categories::get_books_by_category)),
        )
        // For /api/comment
        .service(
            web::resource("/comment")
                .wrap(auth.clone())
                .route(web::post().guard(admin_guard()).to(comments::add_comment)),
        )
        .service(
            web::resource("/comment/{book_id}")
                .wrap(auth.clone())
                .route(web::get().to(comments::get_comment)),
        )
        .service(
            web::resource("/comment/{book_id}")
                .route(web::put().guard(admin_guard()).to(comments::update_comment))
                .route(
                    web::delete()
                        .guard(admin_guard())
                        .to(comments::delete_comment),
                ),
        )
        // For /api/discover
        .service(
            web::resource("/discover/books")
                .wrap(auth.clone())
                .route(web::get().to(discover::get_books)),
        )
        // For /api/download
        .service(
            web::resource("/download/books")
                .wrap(auth.clone())
                .route(web::get().to(download_history::get_books)),
        )
        // For /api/file
        // Note that authentication is checked in callback explicitly.
        .service(web::resource("/file").route(web::get().to(files::get_file_by_path)))
        // For /api/formats
        .service(
            web::resource("/format")
                .wrap(auth.clone())
                .route(web::get().to(file_formats::get_formats)),
        )
        .service(
            web::resource("/format/{format_id}")
                .wrap(auth.clone())
                .route(web::get().to(file_formats::get_format)),
        )
        .service(
            web::resource("/format/books/{format_id}")
                .wrap(auth.clone())
                .route(web::get().to(file_formats::get_books_by_file_format)),
        )
        // For /api/image
        .service(web::resource("/image").route(web::get().to(images::get_image_by_path)))
        // For /api/publisher
        .service(
            web::resource("/publisher")
                .wrap(auth.clone())
                .route(web::get().to(publishers::get_publishers))
                .route(
                    web::post()
                        .guard(admin_guard())
                        .to(publishers::add_publisher),
                ),
        )
        .service(
            web::resource("/publisher/{publisher_id}")
                .wrap(auth.clone())
                .route(web::get().to(publishers::get_publisher))
                .route(
                    web::delete()
                        .guard(admin_guard())
                        .to(publishers::delete_publisher),
                ),
        )
        .service(
            web::resource("/publisher/books/{publisher_id}")
                .wrap(auth.clone())
                .route(web::get().to(publishers::get_books_by_publisher)),
        )
        // For /api/rating
        .service(
            web::resource("/rating")
                .wrap(auth.clone())
                .route(web::post().guard(admin_guard()).to(ratings::add_rating)),
        )
        .service(
            web::resource("/rating/{book_id}")
                .wrap(auth.clone())
                .route(web::get().to(ratings::get_ratings))
                .route(web::put().guard(admin_guard()).to(ratings::update_rating))
                .route(
                    web::delete()
                        .guard(admin_guard())
                        .to(ratings::delete_rating),
                ),
        )
        // For /api/series
        .service(
            web::resource("/series")
                .wrap(auth.clone())
                .route(web::get().to(series::get_series_list))
                .route(web::post().guard(admin_guard()).to(series::add_series)),
        )
        .service(
            web::resource("/series/{series_id}")
                .wrap(auth.clone())
                .route(web::get().to(series::get_series))
                .route(web::delete().guard(admin_guard()).to(series::delete_series)),
        )
        .service(
            web::resource("/series/books/{series_id}")
                .wrap(auth.clone())
                .route(web::get().to(series::get_books_by_series)),
        )
        // For /api/tag
        .service(
            web::resource("/tag")
                .wrap(auth.clone())
                .route(web::get().to(tags::get_tags))
                .route(web::post().guard(admin_guard()).to(tags::add_tag)),
        )
        .service(
            web::resource("/tag/{tag_id}")
                .wrap(auth.clone())
                .route(web::get().to(tags::get_tag))
                .route(web::put().guard(admin_guard()).to(tags::update_tag))
                .route(web::delete().guard(admin_guard()).to(tags::delete_tag)),
        )
        .service(
            web::resource("/tag/books/{tag_id}")
                .wrap(auth.clone())
                .route(web::get().to(tags::get_books_by_tag)),
        )
        // For /api/user-tag
        .service(
            web::resource("/user-tag")
                .wrap(auth.clone())
                .route(web::get().to(user_tags::get_tags))
                .route(web::post().guard(admin_guard()).to(user_tags::add_tag)),
        )
        .service(
            web::resource("/user-tag/{tag_id}")
                .wrap(auth.clone())
                .route(web::get().to(user_tags::get_tag)),
        )
        .service(
            web::resource("/user-tag/books/{tag_id}")
                .wrap(auth.clone())
                .route(web::get().to(user_tags::get_books_by_user_tag)),
        )
        // For /api/search
        .service(
            web::resource("/search/books")
                .wrap(auth.clone())
                .route(web::get().to(simple_search::get_books_by_simple_search)),
        )
        // For /api/advanced-search
        .service(
            web::resource("/advanced-search/books")
                .wrap(auth.clone())
                .route(web::get().to(advanced_search::get_books_by_advanced_search)),
        )
        // For /api/user
        .route("/user/login", web::post().to(users::login))
        .service(
            web::resource("/user/self")
                .wrap(auth.clone())
                .route(web::get().to(users::get_user_info)),
        )
        .service(
            web::resource("/user")
                .wrap(auth.clone())
                .route(web::get().guard(admin_guard()).to(users::get_users))
                .route(web::post().guard(admin_guard()).to(users::add_user)),
        )
        .service(
            web::resource("/user/{user_id}")
                .wrap(auth)
                .route(web::delete().guard(admin_guard()).to(users::delete_user)),
        );
}

pub async fn run() -> Result<(), Error> {
    let pool = db::get_connection_pool()?;
    {
        log::info!("Initialize database tables");
        let mut conn = pool.get()?;
        db::create_table_schema(&mut conn)?;
    }

    let server = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api").configure(scoped_config))
    });

    server
        .bind(("0.0.0.0", 3000))?
        .run()
        .await
        .map_err(Into::into)
}
