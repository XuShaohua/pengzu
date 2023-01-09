// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use shared::recursive_query::RecursiveQuery;
use shared::user_tags::UserTagAndBook;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::user_tags::fetch_user_tags;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct TagsContainerProps {
    pub tag_list: Vec<UserTagAndBook>,
}

#[function_component(TagsContainerComponent)]
pub fn tags_container(props: &TagsContainerProps) -> Html {
    html! {
        <>
        <TagItemListComponent tag_list={ props.tag_list.clone() } />
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct TagItemListProps {
    pub tag_list: Vec<UserTagAndBook>,
}

#[function_component(TagItemListComponent)]
pub fn tag_item_list(props: &TagItemListProps) -> Html {
    let tag_list = &props.tag_list;

    html! {
        <ul class="list-unstyled">
        {for tag_list.iter().map(|tag| html!{
            <li class="mb-3" key={ tag.id }>
            <UserTagItemComponent tag={ tag.clone() } />
            </li>
        })}
        </ul>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct TagItemProps {
    pub tag: UserTagAndBook,
}

#[function_component(UserTagItemComponent)]
pub fn tag_item(props: &TagItemProps) -> Html {
    let tag = &props.tag;
    let parent_id = tag.id;

    let child_tags = use_async(async move {
        let query = RecursiveQuery {
            parent: parent_id,
            fetch_all: true,
            ..RecursiveQuery::default()
        };
        fetch_user_tags(&query).await
    });
    let onclick = {
        let child_tags = child_tags.clone();
        Callback::from(move |_event| {
            child_tags.run();
        })
    };

    let child_items = child_tags.data.as_ref().map_or_else(
        || html! {},
        |tag_list| {
            html! {
                <TagItemListComponent tag_list={ tag_list.list.clone() } />
            }
        },
    );

    html! {
        <>
            <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ tag.count }</span>
            <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id }}>
                { &tag.name }
            </Link<Route>>
            <a href="#" {onclick}><i class="bi bi-caret-right"></i></a>
            { child_items }
        </>
    }
}
