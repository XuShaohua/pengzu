// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::PgConnection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Comment {
    book: i32,
    comment: String,
}

pub fn add_comment(conn: &PgConnection, req: &Comment) -> Result<Comment, ()> {
    todo!();
}

pub fn get_comments() {
    todo!();
}

pub fn update_comment() {
    todo!();
}

pub fn delete_comment() {
    todo!();
}
