// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_user_tag;
use crate::services::user_tags::fetch_user_tag;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub tag_id: i32,
}

#[function_component(BooksOfUserTagComponent)]
pub fn books_of_user_tag(props: &Props) -> Html {
    let tag_id = props.tag_id;
    let book_list = use_async_with_options(
        async move { fetch_books_by_user_tag(tag_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let tag_id = props.tag_id;
    let tag_info = use_async_with_options(
        async move { fetch_user_tag(tag_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let title_element = tag_info.data.as_ref().map_or_else(
        || html! {},
        |tag_info| {
            html! {
                <h2>{ format!("Result of \"{}\"", tag_info.name) }</h2>
            }
        },
    );

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                { title_element }
                <BookListComponent books={ book_list.list.clone() } />
                </>
            }
        },
    )
}
