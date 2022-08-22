// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use gloo_storage::Storage;
use serde::de::DeserializeOwned;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

use crate::error::FetchError;

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
    request("GET", url, None).await
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
pub async fn request_post<T>(url: &str, body: Option<&str>) -> Result<T, FetchError>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    request("POST", url, body).await
}

fn get_token() -> Option<String> {
    let storage = gloo_storage::LocalStorage::raw();
    storage.get("TOKEN").unwrap()
}

async fn request<T>(method: &str, url: &str, body: Option<&str>) -> Result<T, FetchError>
where
    T: DeserializeOwned + std::fmt::Debug,
{
    let mut opts = RequestInit::new();
    let headers = Headers::new()?;
    if let Some(token) = get_token() {
        headers.set("Authorization", &format!("Bearer {}", token))?;
    }
    headers.set("Content-Type", "application/json")?;
    opts.method(method)
        .mode(RequestMode::Cors)
        .headers(&headers);
    if let Some(body) = body {
        opts.body(Some(&JsValue::from_str(body)));
    }

    let request = Request::new_with_str_and_init(url, &opts)?;
    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let text = JsFuture::from(resp.text()?).await?;
    text.into_serde().map_err(Into::into)
}
