// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;

pub struct ImportBook {
    pub id: i32,
    pub library: i32,
    pub calibre_book: i32,
    pub status: u32,
    pub book: Option<i32>,
    pub created: NaiveDateTime,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImportBookStatus {
    Ok = 0,
    BookNotFoundError = 1,
    CoverNotFoundError = 2,
    DuplicateError = 3,
    OtherErrors = 128,
}
