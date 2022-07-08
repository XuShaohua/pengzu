// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::{
    ExpressionMethods, GroupByDsl, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable,
    RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use super::common_page;
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

#[derive(Debug, Clone, Deserialize)]
pub struct GetAuthorsQuery {
    #[serde(default = "common_page::default_page_id")]
    pub page: i64,
    #[serde(default = "default_has_book")]
    pub has_book: bool,
}

#[must_use]
pub const fn default_has_book() -> bool {
    true
}

#[derive(Debug, Serialize, Queryable)]
pub struct AuthorAndBooks {
    pub id: i32,
    pub name: String,
    pub link: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct GetAuthorsResp {
    pub page: common_page::Page,
    pub list: Vec<AuthorAndBooks>,
}

pub fn get_authors(conn: &PgConnection, query: &GetAuthorsQuery) -> Result<GetAuthorsResp, Error> {
    log::info!("query: {:?}", query);

    use crate::schema::books_authors_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 100;
    let offset = page_id * each_page;

    let list = if query.has_book {
        authors::table
            .inner_join(books_authors_link::table.on(books_authors_link::author.eq(authors::id)))
            .group_by(authors::id)
            .select((
                authors::id,
                authors::name,
                authors::link,
                diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_authors_link.id)"),
            ))
            .limit(each_page)
            .offset(offset)
            .load::<AuthorAndBooks>(conn)?
    } else {
        let list = authors::table
            .limit(each_page)
            .offset(offset)
            .load::<Author>(conn)?;
        list.into_iter()
            .map(|author| AuthorAndBooks {
                id: author.id,
                name: author.name,
                link: author.link,
                count: 0,
            })
            .collect()
    };

    let total = if query.has_book {
        books_authors_link::table
            .select(diesel::dsl::sql::<diesel::sql_types::BigInt>(
                "count(distinct(books_authors_link.author))",
            ))
            .first(conn)?
    } else {
        authors::table.count().first(conn)?
    };

    Ok(GetAuthorsResp {
        page: common_page::Page {
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
