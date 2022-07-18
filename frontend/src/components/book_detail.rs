// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::components::models::books_meta::{fetch_book_metadata, BookMetadata};
use crate::components::models::error::FetchError;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(BookMetadata),
    FetchFailed(FetchError),
}

#[derive(Debug, PartialEq, Properties)]
pub struct Prop {
    pub book_id: i32,
}

pub struct BookDetailComponent {
    metadata: Option<BookMetadata>,
}

impl Component for BookDetailComponent {
    type Message = Msg;
    type Properties = Prop;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { metadata: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let book_id = ctx.props().book_id;
                ctx.link().send_future(async move {
                    match fetch_book_metadata(book_id).await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(metadata) => {
                log::info!("metadata: {:#?}", metadata);
                self.metadata = Some(metadata);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch something: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        html! {
            <>
                <button onclick={fetch}>{"Fetch metadata"}</button>
            </>
        }
    }
}
