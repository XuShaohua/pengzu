// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

use super::books::GetBooksResp;
use super::error::FetchError;

pub async fn fetch_authors() -> Result<GetBooksResp, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = "/api/author";
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    let text = text.as_string().unwrap();
    let obj: GetBooksResp = serde_json::from_str(&text).expect("Invalid response");
    Ok(obj)
}
