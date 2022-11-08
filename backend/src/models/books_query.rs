// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::expression::expression_types::NotSelectable;
use diesel::{BoxableExpression, ExpressionMethods};
use serde::Deserialize;

use crate::models::page::{default_page_id, PageId};
use crate::schema::books;

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum GetBooksOrder {
    IdDesc,
    IdAsc,
    TitleDesc,
    TitleAsc,
    AuthorDesc,
    AuthorAsc,
    PubdateDesc,
    PubdateAsc,
}

impl Default for GetBooksOrder {
    fn default() -> Self {
        Self::IdDesc
    }
}

impl GetBooksOrder {
    #[must_use]
    pub fn get_column(
        self,
    ) -> Box<dyn BoxableExpression<books::dsl::books, diesel::pg::Pg, SqlType = NotSelectable>>
    {
        // FIXME(Shaohua): Return type not match.
        use crate::schema::books::dsl;
        match self {
            Self::IdAsc => Box::new(dsl::id.asc()),
            Self::IdDesc => Box::new(dsl::id.desc()),
            Self::TitleAsc => Box::new(dsl::title.asc()),
            Self::TitleDesc => Box::new(dsl::title.desc()),
            // TODO(Shaohua): Support author sorting.
            Self::AuthorAsc => Box::new(dsl::created.asc()),
            Self::AuthorDesc => Box::new(dsl::created.desc()),
            Self::PubdateAsc => Box::new(dsl::pubdate.asc()),
            Self::PubdateDesc => Box::new(dsl::pubdate.desc()),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct GetBooksQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}
