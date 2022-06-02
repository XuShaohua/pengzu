// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::Error;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ImportLibrary {
    pub id: i32,
    pub calibre_path: String,
    pub total: i32,
    pub finished: bool,
    pub worker_pid: Option<i32>,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}

pub fn add_library(_calibre_path: &str) -> Result<i32, Error> {
    Ok(0)
}
