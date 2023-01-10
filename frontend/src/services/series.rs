// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthorsList;
use shared::books_query::GetBooksQuery;
use shared::general_query::GeneralQuery;
use shared::series::{NewSeries, Series, SeriesAndBookList};

use crate::error::FetchError;
use crate::services::fetch::{request_delete, request_get, request_post, request_put};

/// Get series list.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_series_list(query: &GeneralQuery) -> Result<SeriesAndBookList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = ["/api/series", &query_str].join("?");
    request_get(&url).await
}

/// Get series info.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_series(series_id: i32) -> Result<Series, FetchError> {
    let url = format!("/api/series/{series_id}");
    request_get(&url).await
}

/// Get book list of specific series `series_id`.
///
/// # Errors
/// Returns error if server fails.
pub async fn fetch_books_by_series(
    series_id: i32,
    query: &GetBooksQuery,
) -> Result<BookAndAuthorsList, FetchError> {
    let query_str = serde_urlencoded::to_string(query)?;
    let url = format!("/api/series/books/{series_id}?{query_str}");
    request_get(&url).await
}

/// Add a new series record.
///
/// # Errors
/// Returns error if server error or `new_series` is invalid.
pub async fn add_series(new_series: &NewSeries) -> Result<Series, FetchError> {
    let url = "/api/series";
    request_post(url, new_series).await
}

/// Update fields of existing series.
///
/// # Errors
/// Returns error if:
/// - Server error
/// - Invalid series id
/// - Invalid fields in `new_series`
pub async fn update_series(series_id: i32, new_series: &NewSeries) -> Result<(), FetchError> {
    let url = format!("/api/series/{series_id}");
    request_put(&url, new_series).await
}

/// Delete specific series.
///
/// Note that this api can be trigger by admin only.
///
/// # Errors
/// Returns error if server fails.
pub async fn delete_series(series_id: i32) -> Result<(), FetchError> {
    let url = format!("/api/series/{series_id}");
    request_delete(&url).await
}
