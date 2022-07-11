// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared_models::books::{BookResp, GetBooksResp};
use shared_models::page::Page;
use std::error;
use std::fmt;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FetchError {
    err: JsValue,
}
impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.err, f)
    }
}
impl error::Error for FetchError {}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        Self { err: value }
    }
}

#[derive(Debug, PartialEq)]
pub enum FetchState {
    Success(String),
    Failed(FetchError),
}

async fn fetch_books(url: &str) -> Result<String, FetchError> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let request = Request::new_with_str_and_init(url, &opts)?;

    let window = gloo_utils::window();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into().unwrap();

    let text = JsFuture::from(resp.text()?).await?;
    let text = text.as_string().unwrap();
    Ok(text)
}

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    SetFetchState(FetchState),
}

pub struct BooksComponent {
    books: Vec<BookResp>,
    page: Option<Page>,
}

impl Component for BooksComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            books: Vec::new(),
            page: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let url = "/api/book";
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    match fetch_books(url).await {
                        Ok(text) => Msg::SetFetchState(FetchState::Success(text)),
                        Err(err) => Msg::SetFetchState(FetchState::Failed(err)),
                    }
                });
                false
            }
            Msg::SetFetchState(state) => {
                match state {
                    FetchState::Success(text) => {
                        let obj: GetBooksResp =
                            serde_json::from_str(&text).expect("Invalid response");
                        log::info!("obj: {:#?}", obj);
                        self.page = Some(obj.page);
                        self.books.extend(obj.list);
                    }
                    FetchState::Failed(err) => {
                        log::warn!("failed to fetch books: {:?}", err);
                    }
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        html! {
            <div>
                <button onclick={fetch}>{"Fetch books"}</button>
            </div>
        }
    }
}
