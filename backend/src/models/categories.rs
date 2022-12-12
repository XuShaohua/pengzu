// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::{ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use shared::categories::{Category, CategoryAndBook, CategoryAndBookList};
use shared::page::Page;
use shared::recursive_query::RecursiveQuery;

use crate::error::Error;
use crate::schema::categories;

#[derive(Debug, Serialize, Deserialize, Insertable)]
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

pub fn get_categories(
    conn: &mut PgConnection,
    query: &RecursiveQuery,
) -> Result<CategoryAndBookList, Error> {
    use crate::schema::books_categories_link;

    // TODO(Shaohua): Support multiple joins.
    // Expected sql query is:
    // ```sql
    // SELECT categories.id, categories.order_index, categories.serial_number,
    //        categories.name, categories.parent, count(c2.id) children, COUNT(bcl.id) books
    // FROM categories
    // LEFT JOIN categories c2 ON categories.id = c2.parent
    // LEFT JOIN books_categories_link bcl ON categories.id = bcl.category
    // WHERE categories.parent = 0
    // GROUP BY categories.id
    // ORDER BY categories.id;
    // ```
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
        .order_by(categories::id.asc())
        .load::<CategoryAndBook>(conn)?;

    Ok(CategoryAndBookList {
        page: Page::default(),
        list,
    })
}
