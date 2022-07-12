// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::components::models::books::{fetch_books, BookResp, GetBooksResp};
use crate::components::models::error::FetchError;
use crate::components::models::page::Page;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(GetBooksResp),
    FetchFailed(FetchError),
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
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    match fetch_books().await {
                        Ok(text) => Msg::FetchSuccess(text),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(obj) => {
                log::info!("obj: {:#?}", obj);
                self.page = Some(obj.page);
                self.books.extend(obj.list);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch books: {:?}", err);
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
