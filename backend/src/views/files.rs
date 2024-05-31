// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use actix_files::NamedFile;
use actix_web::{web, HttpRequest};
use shared::files::FileQuery;

use crate::db::DbPool;
use crate::error::{Error, ErrorKind};
use crate::models::{download_history, files};
use crate::settings;
use crate::views::auth::get_claims_from_cookie;

pub async fn get_file_by_path(
    pool: web::Data<DbPool>,
    req: HttpRequest,
    query: web::Query<FileQuery>,
) -> Result<NamedFile, Error> {
    log::info!("filepath: {:?}", query.path);

    // 1. check auth token
    let claims = get_claims_from_cookie(&req)?;
    let user_id = claims.id();

    // 2. check book file exists
    let book_id = query.book;
    let file_id = query.file;
    let book_file_path = {
        let mut conn = pool.get()?;
        web::block(move || files::get_book_file_path(&mut conn, book_id, file_id)).await??
    };
    if book_file_path != query.path {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!("Invalid filepath: {query:?}"),
        ));
    }

    // 3. add download history record
    {
        let mut conn = pool.get()?;
        web::block(move || {
            let new_history = download_history::NewHistory {
                user_id,
                book: book_id,
                file: file_id,
            };
            download_history::add(&mut conn, &new_history)
        })
        .await??;
    }

    // 4. return real file path
    let root_dir = settings::get_library_root_dir()?;
    let filepath = root_dir.join(&query.path);
    log::info!("real path: {filepath:?}");
    if !filepath.starts_with(root_dir) {
        return Err(Error::from_string(
            ErrorKind::IoError,
            format!("Invalid filepath: {query:?}"),
        ));
    }
    let file = NamedFile::open(filepath)?;
    Ok(file.use_last_modified(true))
}
