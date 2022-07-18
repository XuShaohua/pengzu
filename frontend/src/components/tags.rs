// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::components::models::error::FetchError;
use crate::components::models::page::Page;
use crate::components::models::tags::{fetch_tags, GetTagsResp, TagAndBook};
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(GetTagsResp),
    FetchFailed(FetchError),
}

pub struct TagsComponent {
    tags: Vec<TagAndBook>,
    page: Option<Page>,
}

fn generate_tag_element(tag: &TagAndBook) -> Html {
    html! {
        <li class="tag-item" key={ tag.id }>
            <span class="badge">{ tag.count }</span>
            <a href={ format!("/tag/books/{}", tag.id) } target="_blank" title={ tag.name.clone() }>
                { tag.name.clone() }
            </a>
        </li>
    }
}

impl Component for TagsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            tags: vec![],
            page: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    match fetch_tags().await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(obj) => {
                log::info!("obj: {:#?}", obj);
                self.page = Some(obj.page);
                self.tags.extend(obj.list);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch tags: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        let tag_elements = self.tags.iter().map(generate_tag_element).collect::<Html>();

        html! {
            <>
                <button onclick={fetch}>{"Fetch tags"}</button>

                <ul class="tag-list">
                    { tag_elements }
                </ul>
            </>
        }
    }
}
