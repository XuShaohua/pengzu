// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::SqliteConnection;

use crate::error::Error;

pub fn get_book_hash(_conn: &SqliteConnection, _book_id: i32) -> Result<String, Error> {
    todo!()
}
