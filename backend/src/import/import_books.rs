// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use calibre::models::books::get_next_book;
use calibre::models::books_authors::get_book_authors;
use calibre::models::comments::get_comment;
use diesel::{PgConnection, SqliteConnection};

use crate::error::Error;
use crate::models::authors::get_author_by_name;
use crate::models::books::{add_book, NewBook};
use crate::models::books_authors::{add_book_author, NewBookAuthor};
use crate::models::comments::{add_comment, NewComment};

fn import_authors(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    let author_list = get_book_authors(sqlite_conn, calibre_book_id)?;
    for calibre_author in &author_list {
        let author = get_author_by_name(pg_conn, &calibre_author.name)?;
        let new_book_author = NewBookAuthor {
            book: book_id,
            author: author.id,
        };
        add_book_author(pg_conn, &new_book_author)?;
    }

    Ok(())
}

fn import_comment(
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
    calibre_book_id: i32,
    book_id: i32,
) -> Result<(), Error> {
    let comment = get_comment(sqlite_conn, calibre_book_id)?;
    let new_comment = NewComment {
        book: book_id,
        text: comment.text,
    };
    add_comment(pg_conn, &new_comment)
}

fn import_book(
    calibre_path: &str,
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
    last_book_id: i32,
) -> Result<(i32, i32), Error> {
    log::info!("calibre path: {}", calibre_path);
    let calibre_book = get_next_book(sqlite_conn, last_book_id)?;
    log::info!("book: {:#?}", calibre_book);
    let new_book = NewBook {
        title: calibre_book.title.clone(),
        sort: calibre_book.sort.unwrap_or_else(|| calibre_book.title),
        author_sort: calibre_book.author_sort.unwrap_or_default(),
        path: calibre_book.path,
        uuid: calibre_book.uuid,
        has_cover: calibre_book.has_cover,
    };
    let book_id = add_book(pg_conn, &new_book)?;
    Ok((calibre_book.id, book_id))
}

pub fn import_books(
    calibre_path: &str,
    sqlite_conn: &SqliteConnection,
    pg_conn: &PgConnection,
) -> Result<(), Error> {
    log::info!("calibre path: {}", calibre_path);
    let mut last_book_id = 0;
    let (calibre_book_id, book_id) = import_book(calibre_path, sqlite_conn, pg_conn, last_book_id)?;
    last_book_id = calibre_book_id;
    log::info!("last book id updated: {}", last_book_id);

    import_authors(sqlite_conn, pg_conn, calibre_book_id, book_id)?;
    import_comment(sqlite_conn, pg_conn, calibre_book_id, book_id)?;

    Ok(())
}
