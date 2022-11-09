// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, PgConnection, PgTextExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use shared::page::{default_page_id, PageId};

use crate::error::Error;
use crate::models::books::{get_books_by_ids, GetBooksResp};
use crate::models::books_query::{GetBooksOrder, GetBooksQuery};

#[derive(Debug, Clone, Deserialize)]
pub struct AdvancedSearchQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,

    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
}

#[allow(unused_assignments)]
pub fn get_books_by_advanced_search(
    conn: &mut PgConnection,
    query: &AdvancedSearchQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::authors;
    use crate::schema::books;
    use crate::schema::books_authors_link;
    use crate::schema::books_publishers_link;
    use crate::schema::publishers;

    let books_query = GetBooksQuery {
        page: query.page,
        order: query.order,
    };

    let mut book_ids = Vec::new();
    let mut book_id_nil = true;
    if let Some(author_name) = &query.author {
        let author_pattern = format!("%{}%", author_name);
        let author_ids = authors::table
            .filter(authors::name.ilike(author_pattern))
            .select(authors::id)
            .load::<i32>(conn)?;

        book_ids = books_authors_link::table
            .filter(books_authors_link::author.eq_any(author_ids))
            .select(books_authors_link::book)
            .load::<i32>(conn)?;
        book_id_nil = false;
    }

    if let Some(publisher_name) = &query.publisher {
        let publisher_pattern = format!("%{}%", publisher_name);
        let publisher_ids = publishers::table
            .filter(publishers::name.ilike(publisher_pattern))
            .select(publishers::id)
            .load::<i32>(conn)?;

        if book_id_nil {
            book_id_nil = false;
            book_ids = books_publishers_link::table
                .filter(books_publishers_link::publisher.eq_any(publisher_ids))
                .select(books_publishers_link::book)
                .load::<i32>(conn)?;
        } else {
            book_ids = books_publishers_link::table
                .filter(books_publishers_link::publisher.eq_any(publisher_ids))
                .filter(books_publishers_link::book.eq_any(book_ids))
                .select(books_publishers_link::book)
                .load::<i32>(conn)?;
        }
    }

    if let Some(title) = &query.title {
        let title_pattern = format!("%{}%", title);
        if book_id_nil {
            book_id_nil = false;
            book_ids = books::table
                .filter(books::title.ilike(&title_pattern))
                .select(books::id)
                .load::<i32>(conn)?;
        } else {
            book_ids = books::table
                .filter(books::title.ilike(&title_pattern))
                .filter(books::id.eq_any(book_ids))
                .select(books::id)
                .load::<i32>(conn)?;
        }
    }

    get_books_by_ids(conn, &books_query, &book_ids)
}
