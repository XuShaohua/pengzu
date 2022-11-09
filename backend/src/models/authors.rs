// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, PgTextExpressionMethods, QueryDsl,
    RunQueryDsl,
};
use serde::Deserialize;
use shared::authors::{Author, AuthorAndBook, AuthorAndBookList};
use shared::books::{AuthorAndBookId, BookAndAuthorsList};
use shared::books_query::GetBooksQuery;
use shared::general_query::GeneralQuery;
use shared::page::{Page, AUTHORS_EACH_PAGE};

use crate::error::Error;
use crate::models::books::{get_books_by_ids, Book};
use crate::schema::authors;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = authors)]
pub struct NewAuthor {
    pub name: String,
    pub link: String,
}

pub fn add_author(conn: &mut PgConnection, new_author: &NewAuthor) -> Result<Author, Error> {
    diesel::insert_into(authors::table)
        .values(new_author)
        .get_result::<Author>(conn)
        .map_err(Into::into)
}

pub fn get_authors(
    conn: &mut PgConnection,
    query: &GeneralQuery,
) -> Result<AuthorAndBookList, Error> {
    use crate::schema::books_authors_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let offset = page_id * AUTHORS_EACH_PAGE;

    // TODO(Shaohua): Support order.
    let list = authors::table
        .left_join(books_authors_link::table.on(books_authors_link::author.eq(authors::id)))
        .group_by(authors::id)
        .select((
            authors::id,
            authors::name,
            authors::link,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_authors_link.id)"),
        ))
        .limit(AUTHORS_EACH_PAGE)
        .offset(offset)
        .load::<AuthorAndBook>(conn)?;

    let total = authors::table.count().first(conn)?;

    Ok(AuthorAndBookList {
        page: Page {
            page_num: page_id + 1,
            each_page: AUTHORS_EACH_PAGE,
            total,
        },
        list,
    })
}

pub fn get_author_by_id(conn: &mut PgConnection, author_id: i32) -> Result<Author, Error> {
    authors::table
        .find(author_id)
        .first::<Author>(conn)
        .map_err(Into::into)
}

pub fn get_author_by_name(conn: &mut PgConnection, author_name: &str) -> Result<Author, Error> {
    use crate::schema::authors::dsl::{authors, name};
    authors
        .filter(name.eq(author_name))
        .first::<Author>(conn)
        .map_err(Into::into)
}

pub fn get_author_by_name_pattern(
    conn: &mut PgConnection,
    name_pattern: &str,
) -> Result<Author, Error> {
    use crate::schema::authors::dsl::{authors, name};
    authors
        .filter(name.ilike(name_pattern))
        .first::<Author>(conn)
        .map_err(Into::into)
}

pub fn update_author(
    conn: &mut PgConnection,
    author_id: i32,
    new_author: &NewAuthor,
) -> Result<(), Error> {
    use crate::schema::authors::dsl::{authors, name};
    diesel::update(authors.find(author_id))
        .set(name.eq(new_author.name.as_str()))
        .execute(conn)?;
    Ok(())
}

pub fn get_authors_by_book_id(
    conn: &mut PgConnection,
    book_list: &[Book],
) -> Result<Vec<AuthorAndBookId>, Error> {
    use crate::schema::books_authors_link;

    let book_ids: Vec<i32> = book_list.iter().map(|book| book.id).collect();

    authors::table
        .inner_join(books_authors_link::table.on(books_authors_link::author.eq(authors::id)))
        .filter(books_authors_link::book.eq_any(book_ids))
        .select((authors::id, authors::name, books_authors_link::book))
        .load::<AuthorAndBookId>(conn)
        .map_err(Into::into)
}

pub fn get_books_by_author(
    conn: &mut PgConnection,
    author_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, Error> {
    use crate::schema::books_authors_link;

    let book_ids = books_authors_link::table
        .filter(books_authors_link::author.eq(author_id))
        .select(books_authors_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}

pub fn delete_by_id(conn: &mut PgConnection, id: i32) -> Result<(), Error> {
    diesel::delete(authors::table.find(id)).execute(conn)?;
    Ok(())
}
