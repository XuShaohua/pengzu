// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, GroupByDsl, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use super::page::{Page, PageQuery};
use crate::error::Error;
use crate::schema::authors;

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "authors"]
pub struct NewAuthor {
    pub name: String,
    pub link: String,
}

#[derive(Debug, Serialize, Queryable)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_author(conn: &PgConnection, new_author: &NewAuthor) -> Result<(), Error> {
    use crate::schema::authors::dsl::authors;
    diesel::insert_into(authors)
        .values(new_author)
        .execute(conn)?;
    Ok(())
}

#[derive(Debug, Serialize, Queryable)]
pub struct AuthorAndBook {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct GetAuthorsResp {
    pub page: Page,
    pub list: Vec<AuthorAndBook>,
}

pub fn get_authors(conn: &PgConnection, query: &PageQuery) -> Result<GetAuthorsResp, Error> {
    use crate::schema::books_authors_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 100;
    let offset = page_id * each_page;

    let list = authors::table
        .left_join(books_authors_link::table.on(books_authors_link::author.eq(authors::id)))
        .group_by(authors::id)
        .select((
            authors::id,
            authors::name,
            authors::link,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_authors_link.id)"),
        ))
        .limit(each_page)
        .offset(offset)
        .load::<AuthorAndBook>(conn)?;

    let total = authors::table.count().first(conn)?;

    Ok(GetAuthorsResp {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

pub fn get_author_by_name(conn: &PgConnection, author_name: &str) -> Result<Author, Error> {
    use crate::schema::authors::dsl::{authors, name};
    authors
        .filter(name.eq(author_name))
        .first::<Author>(conn)
        .map_err(Into::into)
}

pub fn update_author(
    conn: &PgConnection,
    author_id: i32,
    new_author: &NewAuthor,
) -> Result<(), Error> {
    use crate::schema::authors::dsl::{authors, name};
    diesel::update(authors.find(author_id))
        .set(name.eq(new_author.name.as_str()))
        .execute(conn)?;
    Ok(())
}
