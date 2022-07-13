// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

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
    opts.method("GET");
    opts.mode(RequestMode::Cors);
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
