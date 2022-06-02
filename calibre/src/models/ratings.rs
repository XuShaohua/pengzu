// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use diesel::Queryable;

#[derive(Debug, Queryable)]
pub struct Rating {
    pub id: i32,
    pub rating: i32,
}
