// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::books_query::GetBooksQuery;
use shared::categories::{Category, CategoryAndBook, CategoryAndBookList};
use shared::page::{default_page_id, Page};

use crate::error::Error;
use crate::models::books::{get_books_by_ids, GetBooksResp};
use crate::schema::categories;

#[derive(Debug, Deserialize, Insertable)]
#[diesel(table_name = categories)]
pub struct NewCategory<'a> {
    pub order_index: i32,
    pub serial_number: &'a str,
    pub name: &'a str,
    pub url: &'a str,
    pub description: Option<&'a str>,
    pub parent: i32,
}

pub fn add_category(conn: &mut PgConnection, new_category: &NewCategory) -> Result<(), Error> {
    use crate::schema::categories::dsl::categories;
    diesel::insert_into(categories)
        .values(new_category)
        .execute(conn)?;
    Ok(())
}

pub fn get_category_by_id(conn: &mut PgConnection, category_id: i32) -> Result<Category, Error> {
    categories::table
        .find(category_id)
        .first(conn)
        .map_err(Into::into)
}

pub fn get_category_by_serial_number(
    conn: &mut PgConnection,
    serial_number_val: &str,
) -> Result<Category, Error> {
    use crate::schema::categories::dsl::{categories, serial_number};
    categories
        .filter(serial_number.eq(serial_number_val))
        .first(conn)
        .map_err(Into::into)
}

#[must_use]
pub const fn default_parent_category_id() -> i32 {
    0
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GetCategoriesReq {
    #[serde(default = "default_parent_category_id")]
    pub parent: i32,
    #[serde(default = "default_page_id")]
    pub page: i64,
}

pub fn get_categories(
    conn: &mut PgConnection,
    query: &GetCategoriesReq,
) -> Result<CategoryAndBookList, Error> {
    use crate::schema::books_categories_link;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let each_page = 100;
    let offset = page_id * each_page;

    let list = categories::table
        .filter(categories::parent.eq(query.parent))
        .left_join(
            books_categories_link::table.on(books_categories_link::category.eq(categories::id)),
        )
        .group_by(categories::id)
        .select((
            categories::id,
            categories::order_index,
            categories::serial_number,
            categories::name,
            categories::parent,
            diesel::dsl::sql::<diesel::sql_types::BigInt>("count(books_categories_link.id)"),
        ))
        .limit(each_page)
        .offset(offset)
        .load::<CategoryAndBook>(conn)?;

    let total = categories::table
        .filter(categories::parent.eq(query.parent))
        .count()
        .first(conn)?;

    Ok(CategoryAndBookList {
        page: Page {
            page_num: page_id + 1,
            each_page,
            total,
        },
        list,
    })
}

pub fn get_books_by_category(
    conn: &mut PgConnection,
    category_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books_categories_link;

    let book_ids = books_categories_link::table
        .filter(books_categories_link::category.eq(category_id))
        .select(books_categories_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}
