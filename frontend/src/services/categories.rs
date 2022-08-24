// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::error::FetchError;
use crate::services::fetch::request_get;
use crate::types::categories::CategoryList;

/// Returns category list.
///
/// # Errors
///
/// Returns error if server fails.
pub async fn fetch_categories(parent_category_id: i32) -> Result<CategoryList, FetchError> {
    let url = format!("/api/category?parent={}", parent_category_id);
    request_get(&url).await
}