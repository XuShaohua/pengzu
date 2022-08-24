// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::dsl::any;
use diesel::{
    ExpressionMethods, Insertable, JoinOnDsl, PgConnection, QueryDsl, Queryable, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

use super::page::{default_page_id, Page};
use crate::error::Error;
use crate::models::file_data;
use crate::schema::books;

const EACH_PAGE: i64 = 50;

#[derive(Debug, Serialize, Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub uuid: String,
    pub has_cover: bool,
    pub pubdate: Option<NaiveDateTime>,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct BookWithCover {
    pub id: i32,
    pub title: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub pubdate: Option<NaiveDateTime>,
    pub created: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub path: String,
    pub uuid: String,
    pub has_cover: bool,
}

pub fn add_book(conn: &PgConnection, new_book: &NewBook) -> Result<Book, Error> {
    use crate::schema::books::dsl::books;
    diesel::insert_into(books)
        .values(new_book)
        .get_result::<Book>(conn)
        .map_err(Into::into)
}

pub fn get_book_by_id(conn: &PgConnection, book_id: i32) -> Result<BookWithCover, Error> {
    books::table
        .find(book_id)
        .first(conn)
        .map(|book| book_to_book_cover(book))
        .map_err(Into::into)
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GetBooksOrder {
    IdDesc,
    IdAsc,
    TitleDesc,
    TitleAsc,
    CreatedDesc,
    CreatedAsc,
    LastModifiedDesc,
    LastModifiedAsc,
    PubdateDesc,
    PubdateAsc,
}

impl Default for GetBooksOrder {
    fn default() -> Self {
        Self::PubdateDesc
    }
}

impl GetBooksOrder {
    #[must_use]
    pub fn get_column(
        self,
    ) -> Box<dyn diesel::BoxableExpression<books::dsl::books, diesel::pg::Pg, SqlType = ()>> {
        use crate::schema::books::dsl;
        match self {
            Self::IdAsc => Box::new(dsl::id.asc()),
            Self::IdDesc => Box::new(dsl::id.desc()),
            Self::TitleAsc => Box::new(dsl::title.asc()),
            Self::TitleDesc => Box::new(dsl::title.desc()),
            Self::CreatedAsc => Box::new(dsl::created.asc()),
            Self::CreatedDesc => Box::new(dsl::created.desc()),
            Self::LastModifiedAsc => Box::new(dsl::last_modified.asc()),
            Self::LastModifiedDesc => Box::new(dsl::last_modified.desc()),
            Self::PubdateAsc => Box::new(dsl::pubdate.asc()),
            Self::PubdateDesc => Box::new(dsl::pubdate.desc()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBooksQuery {
    #[serde(default = "default_page_id")]
    pub page: i64,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}

#[derive(Debug, Clone, Serialize, Queryable)]
pub struct AuthorAndBookId {
    pub id: i32,
    pub name: String,
    pub book: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct BookResp {
    pub book: BookWithCover,
    pub authors: Vec<AuthorAndBookId>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GetBooksResp {
    pub page: Page,
    pub list: Vec<BookResp>,
}

pub fn book_to_book_cover(book: Book) -> BookWithCover {
    BookWithCover {
        id: book.id,
        title: book.title,
        has_cover: book.has_cover,
        small_cover: file_data::get_small_cover(&book.path, book.has_cover),
        large_cover: file_data::get_large_cover(&book.path, book.has_cover),
        created: book.created,
        pubdate: book.pubdate,
    }
}

fn merge_books_and_authors(book_list: Vec<Book>, author_list: &[AuthorAndBookId]) -> Vec<BookResp> {
    let mut list = Vec::with_capacity(book_list.len());

    for book in book_list {
        let authors = author_list
            .iter()
            .filter(|author| author.book == book.id)
            .map(Clone::clone)
            .collect();
        list.push(BookResp {
            book: book_to_book_cover(book),
            authors,
        })
    }

    list
}

fn get_authors_by_book_id(
    conn: &PgConnection,
    book_list: &[Book],
) -> Result<Vec<AuthorAndBookId>, Error> {
    use crate::schema::authors;
    use crate::schema::books_authors_link;

    let book_ids: Vec<i32> = book_list.iter().map(|book| book.id).collect();

    authors::table
        .inner_join(books_authors_link::table.on(books_authors_link::author.eq(authors::id)))
        .filter(books_authors_link::book.eq(any(book_ids)))
        .select((authors::id, authors::name, books_authors_link::book))
        .load::<AuthorAndBookId>(conn)
        .map_err(Into::into)
}

pub fn get_books(conn: &PgConnection, query: &GetBooksQuery) -> Result<GetBooksResp, Error> {
    use crate::schema::books::dsl::books;

    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let offset = page_id * EACH_PAGE;
    let order_column = query.order.get_column();

    let book_list = books
        .order_by(order_column)
        .limit(EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;
    let author_list = get_authors_by_book_id(conn, &book_list)?;
    let list = merge_books_and_authors(book_list, &author_list);

    let total = books.count().first(conn)?;

    Ok(GetBooksResp {
        page: Page {
            page_num: page_id + 1,
            each_page: EACH_PAGE,
            total,
        },
        list,
    })
}

fn get_books_by_ids(
    conn: &PgConnection,
    query: &GetBooksQuery,
    book_ids: &[i32],
) -> Result<GetBooksResp, Error> {
    let page_id = if query.page < 1 { 0 } else { query.page - 1 };
    let offset = page_id * EACH_PAGE;
    let order_column = query.order.get_column();
    let total = book_ids.len() as i64;

    let book_list = books::table
        .filter(books::id.eq(any(book_ids)))
        .order_by(order_column)
        .limit(EACH_PAGE)
        .offset(offset)
        .load::<Book>(conn)?;
    let author_list = get_authors_by_book_id(conn, &book_list)?;
    let list = merge_books_and_authors(book_list, &author_list);

    Ok(GetBooksResp {
        page: Page {
            page_num: page_id + 1,
            each_page: EACH_PAGE,
            total,
        },
        list,
    })
}

pub fn get_books_by_author(
    conn: &PgConnection,
    author_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books_authors_link;

    let book_ids = books_authors_link::table
        .filter(books_authors_link::author.eq(author_id))
        .select(books_authors_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}

pub fn get_books_by_category(
    conn: &PgConnection,
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

pub fn get_books_by_format(
    conn: &PgConnection,
    format_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::files;

    let book_ids = files::table
        .filter(files::format.eq(format_id))
        .select(files::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}

pub fn get_books_by_publisher(
    conn: &PgConnection,
    publisher_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books_publishers_link;

    let book_ids = books_publishers_link::table
        .filter(books_publishers_link::publisher.eq(publisher_id))
        .select(books_publishers_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}

pub fn get_books_by_series(
    conn: &PgConnection,
    series_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books_series_link;

    let book_ids = books_series_link::table
        .filter(books_series_link::series.eq(series_id))
        .select(books_series_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}

pub fn get_books_by_tag(
    conn: &PgConnection,
    tag_id: i32,
    query: &GetBooksQuery,
) -> Result<GetBooksResp, Error> {
    use crate::schema::books_tags_link;

    let book_ids = books_tags_link::table
        .filter(books_tags_link::tag.eq(tag_id))
        .select(books_tags_link::book)
        .load::<i32>(conn)?;

    get_books_by_ids(conn, query, &book_ids)
}
