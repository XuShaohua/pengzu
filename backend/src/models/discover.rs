// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{sql_function, PgConnection, QueryDsl, RunQueryDsl};

use crate::error::Error;
use crate::models::authors::get_authors_by_book_id;
use crate::models::books::{merge_books_and_authors, Book, GetBooksResp};
use crate::models::page::{default_page_id, Page, EACH_PAGE};

#[allow(clippy::redundant_pub_crate)]
pub fn get_books_by_discover(conn: &mut PgConnection) -> Result<GetBooksResp, Error> {
    use crate::schema::books;

    sql_function!(
        /// Represents the SQL RANDOM() function
        fn random() -> Integer;
    );

    let book_list = books::table
        .order(random())
        .limit(EACH_PAGE)
        .load::<Book>(conn)?;

    let author_list = get_authors_by_book_id(conn, &book_list)?;
    let list = merge_books_and_authors(book_list, &author_list);

    Ok(GetBooksResp {
        page: Page {
            page_num: default_page_id(),
            each_page: EACH_PAGE,
            total: EACH_PAGE,
        },
        list,
    })
}
