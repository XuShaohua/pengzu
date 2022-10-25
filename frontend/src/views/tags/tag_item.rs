// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::tags::fetch_tags;
use crate::types::tags::{TagAndBook, TagList};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub tag: TagAndBook,
}

pub fn generate_tag_list(tag_list: &TagList) -> Html {
    html! {
        <ul>
        {for tag_list.list.iter().map(|tag| html!{
            <li class="tag-item" key={ tag.id }>
            <TagItemComponent tag={ tag.clone() } />
            </li>
        })}
        </ul>
    }
}

#[function_component(TagItemComponent)]
pub fn tag_item(props: &Props) -> Html {
    let tag = &props.tag;
    let parent_id = tag.id;

    let child_tags = { use_async(async move { fetch_tags(parent_id).await }) };
    let onclick = {
        let child_tags = child_tags.clone();
        Callback::from(move |_event| {
            child_tags.run();
        })
    };

    let child_items = child_tags
        .data
        .as_ref()
        .map_or_else(|| html! {}, generate_tag_list);

    html! {
        <>
            <span class="badge">{ tag.count }</span>
            <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id }}>
                { &tag.name }
            </Link<Route>>
            <a href="#" {onclick}>{ "˃" }</a>
            { child_items }
        </>
    }
}
