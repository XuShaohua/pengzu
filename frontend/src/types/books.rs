// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::types::page::{default_page_id, Page, PageId};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct AuthorAndBookId {
    pub id: i32,
    pub name: String,
    pub book: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BookWithCover {
    pub id: i32,
    pub title: String,
    pub path: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub pubdate: Option<NaiveDateTime>,
    pub created: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Book {
    pub book: BookWithCover,
    pub authors: Vec<AuthorAndBookId>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
pub struct BooksList {
    pub page: Page,
    pub list: Vec<Book>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct BookDetail {
    pub id: i32,
    pub title: String,
    pub has_cover: bool,
    pub small_cover: Option<String>,
    pub large_cover: Option<String>,
    pub created: NaiveDateTime,
    pub pubdate: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct GetBooksQuery {
    #[serde(default = "default_page_id")]
    pub page: PageId,
    #[serde(default = "GetBooksOrder::default")]
    pub order: GetBooksOrder,
}

impl Default for GetBooksQuery {
    fn default() -> Self {
        Self {
            page: default_page_id(),
            order: GetBooksOrder::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
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
