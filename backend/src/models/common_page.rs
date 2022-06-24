// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Page {
    pub page_num: i64,
    pub each_page: i64,
    pub total: i64,
}
