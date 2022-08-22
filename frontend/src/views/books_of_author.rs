// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::components::book_list::BookListComponent;
use crate::error::FetchError;
use crate::services::books::fetch_books_by_author;
use crate::types::books::{BookResp, GetBooksResp};
use crate::types::page::Page;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(GetBooksResp),
    FetchFailed(FetchError),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Prop {
    pub author_id: i32,
}

pub struct BooksOfAuthorComponent {
    books: Vec<BookResp>,
    page: Option<Page>,
}

impl Component for BooksOfAuthorComponent {
    type Message = Msg;
    type Properties = Prop;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            books: Vec::new(),
            page: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let author_id = ctx.props().author_id;
                ctx.link().send_future(async move {
                    match fetch_books_by_author(author_id).await {
                        Ok(obj) => Msg::FetchSuccess(obj),
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
            <>
                <button onclick={fetch}>{"Fetch books by author"}</button>
                <BookListComponent books={self.books.clone()} />
            </>
        }
    }
}
