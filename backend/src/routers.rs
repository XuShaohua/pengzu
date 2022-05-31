// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{guard, middleware, web, App, HttpServer, Responder};

use crate::db::get_connection_pool;
use crate::error::Error;
use crate::views::comments;

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
            .service(
                web::resource("/api/comment/{book_id}")
                    .guard(guard::Header(CONTENT_TYPE, APPLICATION_JSON))
                    .route(web::get().to(comments::get_comment))
                    .route(web::put().to(comments::update_comment))
                    .route(web::delete().to(comments::delete_comment)),
            )
            .route("/api/comment", web::post().to(comments::add_comment))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
    .map_err(Into::into)
}
