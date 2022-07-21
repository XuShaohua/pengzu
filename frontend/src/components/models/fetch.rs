// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Headers, Request, RequestInit, RequestMode, Response};

use crate::components::models::error::{ErrorKind, FetchError};

/// Wrap fetch() api in browser.
///
/// Response body is returned as string.
///
/// Note that response status code is not check currently.
///
/// # Errors
///
/// Returns error if failed to construct request or failed to read response body.
pub async fn fetch(url: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET").mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let text = JsFuture::from(resp.text()?).await?;
    text.as_string().ok_or_else(|| {
        FetchError::from_string(
            ErrorKind::ResponseError,
            format!("Failed to read response body as text in: {:?}", url),
        )
    })
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
pub async fn fetch_post(url: &str, body: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    let headers = Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    opts.method("POST")
        .body(Some(&JsValue::from_str(body)))
        .headers(&headers);
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;

    let text = JsFuture::from(resp.text()?).await?;
    text.as_string().ok_or_else(|| {
        FetchError::from_string(
            ErrorKind::ResponseError,
            format!("Failed to read response body as text in: {:?}", url),
        )
    })
}
