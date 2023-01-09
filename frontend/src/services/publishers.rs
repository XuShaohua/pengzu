// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::general_query::GeneralQuery;
use shared::publishers::{NewPublisher, Publisher, PublisherAndBookList};

use crate::error::FetchError;
use crate::services::fetch::{request_delete, request_get, request_post, request_put};

/// Get publisher list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_publishers(query: &GeneralQuery) -> Result<PublisherAndBookList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/publisher", &query_str].join("?");
    request_get(&url).await
}

/// Get publisher info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_publisher(publisher_id: i32) -> Result<Publisher, FetchError> {
    let url = format!("/api/publisher/{publisher_id}");
    request_get(&url).await
}

/// Get book list of specific publisher `publisher_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_publisher(
    publisher_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = format!("/api/publisher/books/{publisher_id}?{query_str}");
    request_get(&url).await
}

/// Add a new publisher record.
///
/// # Errors
/// Returns error if server error or `new_publisher` is invalid.
pub async fn add_publisher(new_publisher: &NewPublisher) -> Result<Publisher, FetchError> {
    let url = "/api/publisher";
    request_post(url, new_publisher).await
}

/// Update fields of existing publisher.
///
/// # Errors
/// Returns error if:
/// - Server error
/// - Invalid publisher id
/// - Invalid fields in `new_publisher`
pub async fn update_publisher(
    publisher_id: i32,
    new_publisher: &NewPublisher,
) -> Result<(), FetchError> {
    let url = format!("/api/publisher/{publisher_id}");
    request_put(&url, new_publisher).await
}

/// Delete specific publisher.
///
/// Note that this api can be trigger by admin only.
///
/// # Errors
/// Returns error if server fails.
pub async fn delete_publisher(publisher_id: i32) -> Result<(), FetchError> {
    let url = format!("/api/publisher/{publisher_id}");
    request_delete(&url).await
}
