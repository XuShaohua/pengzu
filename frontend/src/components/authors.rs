// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::components::models::authors::{fetch_authors, AuthorAndBook, GetAuthorsResp};
use crate::components::models::error::FetchError;
use crate::components::models::page::Page;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(GetAuthorsResp),
    FetchFailed(FetchError),
}

pub struct AuthorsComponent {
    authors: Vec<AuthorAndBook>,
    page: Option<Page>,
}

fn generate_author_element(author: &AuthorAndBook) -> Html {
    html! {
        <li class="author-item" key={ author.id }>
            <span class="badge">{ author.count }</span>
            <a href={ format!("/author/books/{}", author.id) } target="_blank" title={ author.name.clone() }>
                { author.name.clone() }
            </a>
        </li>
    }
}

impl Component for AuthorsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            authors: vec![],
            page: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    match fetch_authors().await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(obj) => {
                log::info!("obj: {:#?}", obj);
                self.page = Some(obj.page);
                self.authors.extend(obj.list);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch authors: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        let author_elements = self
            .authors
            .iter()
            .map(generate_author_element)
            .collect::<Html>();

        html! {
            <>
                <button onclick={fetch}>{"Fetch authors"}</button>

                <ul class="author-list">
                    { author_elements }
                </ul>
            </>
        }
    }
}
