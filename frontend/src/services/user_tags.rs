// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::user_tags::{UserTag, UserTagList};

use crate::error::FetchError;
use crate::services::fetch::request_get;

/// Returns user tag list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_user_tags(parent_tag_id: i32) -> Result<UserTagList, FetchError> {
    let url = format!("/api/user-tag?parent={}", parent_tag_id);
    request_get(&url).await
}

/// Returns user tag info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_user_tag(tag_id: i32) -> Result<UserTag, FetchError> {
    let url = format!("/api/user-tag/{}", tag_id);
    request_get(&url).await
}
