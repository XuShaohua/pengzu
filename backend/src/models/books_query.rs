// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::pg::Pg;
use diesel::{ExpressionMethods, QueryDsl};
use shared::books_query::GetBooksOrder;

use crate::schema::books;

pub fn sort_books_by_column(order: GetBooksOrder) -> books::BoxedQuery<'static, Pg> {
    let query = books::table.into_boxed();
    match order {
        GetBooksOrder::IdDesc => query.order(books::id.desc()),
        GetBooksOrder::IdAsc => query.order(books::id.asc()),
        GetBooksOrder::TitleDesc => query.order(books::title.desc()),
        GetBooksOrder::TitleAsc => query.order(books::title.asc()),
        GetBooksOrder::AuthorDesc => query.order(books::author_sort.desc()),
        GetBooksOrder::AuthorAsc => query.order(books::author_sort.asc()),
        GetBooksOrder::PubdateDesc => query.order(books::pubdate.desc()),
        GetBooksOrder::PubdateAsc => query.order(books::pubdate.asc()),
    }
}
