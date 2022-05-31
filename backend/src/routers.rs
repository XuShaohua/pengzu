// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_web::{web, App, HttpServer, Responder};
use std::io;

use crate::db::get_connection_pool;

async fn index() -> impl Responder {
    "Hello, world"
}

pub async fn init() -> io::Result<()> {
    let pool = get_connection_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .resource("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
