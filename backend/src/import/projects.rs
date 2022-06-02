// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use chrono::NaiveDateTime;
use diesel::Queryable;
use serde::Serialize;

#[derive(Debug, Serialize, Queryable)]
pub struct ImportProject {
    pub id: i32,
    pub calibre_path: String,
    pub total: i32,
    pub finished: ok,
    pub worker_pid: Option<i32>,
    pub created: NaiveDateTime,
    pub last_modified: NaiveDateTime,
}
