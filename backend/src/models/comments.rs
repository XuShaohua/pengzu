// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddComment {
    book: i32,
    comment: String,
}

pub fn add_comment(_req: AddComment) {
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
