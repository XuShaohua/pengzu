// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::recursive_query::RecursiveQuery;
use shared::tags::{TagAndBook, TagAndBookList};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::tags::fetch_tags;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub tag: TagAndBook,
}

pub fn generate_tag_list(tag_list: &TagAndBookList) -> Html {
    html! {
        <ul class="list-unstyled">
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

    let child_tags = use_async(async move {
        let query = RecursiveQuery {
            parent: parent_id,
            fetch_all: true,
            ..RecursiveQuery::default()
        };
        fetch_tags(&query).await
    });
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
            <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ tag.count }</span>
            <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id }}>
                { &tag.name }
            </Link<Route>>
            {
                if tag.children > 0 {
                    html! { <a href="#" {onclick}><i class="bi bi-caret-right"></i></a> }
                } else {
                    html! {}
                }
            }
            { child_items }
        </>
    }
}
