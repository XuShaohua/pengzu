// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod db;
mod files;

use calibre::models::books::{get_next_book, CalibreBook};
use diesel::{PgConnection, SqliteConnection};

use crate::error::Error;
use crate::import::models::books::{add_import_book, NewImportBook};
use crate::import::models::libraries::{update_import_library, ImportLibrary};
use crate::import::options::ImportBookOptions;
use crate::models::books::{add_book, Book, NewBook};
use db::import_book_detail;
use files::copy_book_files;

/// Add book record to pgsql database.
fn add_book_record(
    calibre_library_path: &str,
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    last_book_id: i32,
) -> Result<Option<(CalibreBook, Book)>, Error> {
    log::info!("import_book({}, {})", calibre_library_path, last_book_id);
    match get_next_book(sqlite_conn, last_book_id) {
        Ok(calibre_book) => {
            let calibre_book_clone: CalibreBook = calibre_book.clone();
            let new_book = NewBook {
                title: calibre_book.title.clone(),
                path: calibre_book.path,
                author_sort: calibre_book.author_sort.unwrap_or_default(),
                uuid: calibre_book.uuid,
                has_cover: calibre_book.has_cover,
            };
            let book = add_book(pg_conn, &new_book)?;
            Ok(Some((calibre_book_clone, book)))
        }
        Err(err) => match err.kind() {
            calibre::error::ErrorKind::DbNotFoundError => {
                log::info!(
                    "No more books in calibre library: {}, last book id is: {}",
                    calibre_library_path,
                    last_book_id
                );
                Ok(None)
            }
            _ => Err(err.into()),
        },
    }
}

fn do_import_book(
    calibre_library_path: &str,
    library_path: &str,
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    calibre_book: &CalibreBook,
    book: &Book,
    options: &ImportBookOptions,
) -> Result<(), Error> {
    copy_book_files(
        calibre_library_path,
        library_path,
        sqlite_conn,
        pg_conn,
        calibre_book,
        book,
        options,
    )?;

    import_book_detail(sqlite_conn, pg_conn, calibre_book.id, book.id)
}

pub fn import_books(
    sqlite_conn: &mut SqliteConnection,
    pg_conn: &mut PgConnection,
    import_library: &ImportLibrary,
    options: &ImportBookOptions,
    mut last_book_id: i32,
) -> Result<(), Error> {
    log::info!("import_books({:?})", &import_library);

    loop {
        match add_book_record(
            &import_library.calibre_library_path,
            sqlite_conn,
            pg_conn,
            last_book_id,
        ) {
            Ok(Some((calibre_book, book))) => {
                last_book_id = calibre_book.id;
                log::info!("last book id updated: {}", last_book_id);

                let ok = if let Err(err) = do_import_book(
                    &import_library.calibre_library_path,
                    &import_library.library_path,
                    sqlite_conn,
                    pg_conn,
                    &calibre_book,
                    &book,
                    options,
                ) {
                    log::warn!("Failed to import book: {:?}, err: {:?}", &calibre_book, err);
                    false
                } else {
                    true
                };

                let new_book = NewImportBook {
                    library: import_library.id,
                    calibre_book: calibre_book.id,
                    ok,
                    book: Some(book.id),
                };
                if let Err(err) = add_import_book(pg_conn, &new_book) {
                    log::error!("Failed to add import-book {:?}, err: {}", new_book, err);
                }
            }
            Ok(None) => {
                log::info!("All books are imported: {:?}", import_library);
                return update_import_library(pg_conn, import_library.id, true);
            }
            Err(err) => return Err(err),
        }
    }
}
