// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use serde::de::DeserializeOwned;
use serde::Serialize;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

use crate::error::{ErrorKind, FetchError};
use crate::services::auth::get_token;

/// Wrap fetch() api in browser.
///
/// Response body is returned as string.
///
/// Note that response status code is not check currently.
///
/// # Errors
///
/// Returns error if failed to construct request or failed to read response body.
pub async fn request_get<T>(url: &str) -> Result<T, FetchError>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    request("GET", url, ()).await
}

/// Wrap fetch() api in browser, for POST requests.
///
/// Response body is returned as string.
///
/// Request body is serialized json string.
/// And content-type is set to `application/json`.
///
/// Note that response status code is not check currently.
///
/// # Errors
///
/// Returns error if failed to construct request or failed to read response body.
pub async fn request_post<T, B>(url: &str, body: B) -> Result<T, FetchError>
where
    T: DeserializeOwned + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request("POST", url, body).await
}

/// Wrap fetch() api in browser, for PUT requests.
///
/// Response body is returned as string.
///
/// Request body is serialized json string.
/// And content-type is set to `application/json`.
///
/// Note that response status code is not check currently.
///
/// # Errors
///
/// Returns error if failed to construct request or failed to read response body.
pub async fn request_put<T, B>(url: &str, body: B) -> Result<T, FetchError>
where
    T: DeserializeOwned + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    request("PUT", url, body).await
}

/// Wrap fetch() api in browser, for DEELETE requests.
///
/// Response body is returned as string.
///
/// Note that response status code is not check currently.
///
/// # Errors
///
/// Returns error if failed to construct request or failed to read response body.
pub async fn request_delete<T>(url: &str) -> Result<T, FetchError>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    request("DELETE", url, ()).await
}

async fn request<T, B>(method: &str, url: &str, body: B) -> Result<T, FetchError>
where
    T: DeserializeOwned + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let mut opts = RequestInit::new();
    let headers = Headers::new()?;
    if let Some(token) = get_token() {
        headers.set("Authorization", &format!("Bearer {token}"))?;
    }
    headers.set("Content-Type", "application/json")?;
    opts.method(method)
        .mode(RequestMode::Cors)
        .headers(&headers);
    if method == "POST" || method == "PUT" {
        let body = serde_json::to_string(&body)?;
        opts.body(Some(&JsValue::from_str(&body)));
    }

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    if resp.ok() {
        let text = JsFuture::from(resp.text()?).await?;
        let s = text
            .as_string()
            .ok_or_else(|| FetchError::new(ErrorKind::ResponseError))?;
        serde_json::from_str(&s).map_err(Into::into)
    } else {
        log::warn!("http response: {}, url: {}", resp.status(), url);
        let reason = match resp.status() {
            400 => ErrorKind::BadRequest,
            401 => ErrorKind::Unauthorized,
            403 => ErrorKind::Forbidden,
            404 => ErrorKind::NotFound,
            500 => ErrorKind::InternalServerError,
            _ => ErrorKind::ResponseError,
        };

        Err(FetchError::new(reason))
    }
}
