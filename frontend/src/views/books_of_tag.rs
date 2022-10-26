// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::history::Location;
use yew_router::hooks::use_location;

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_tag;
use crate::services::tags::fetch_tag;
use crate::types::books::GetBooksQuery;
use crate::views::util;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub tag_id: i32,
}

#[function_component(BooksOfTagComponent)]
pub fn books_of_tag(props: &Props) -> Html {
    util::set_document_title(&format!("Tag: {}", props.tag_id));

    let tag_id = props.tag_id;
    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = use_async_with_options(
        async move { fetch_books_by_tag(tag_id, &query).await },
        UseAsyncOptions::enable_auto(),
    );

    let tag_id = props.tag_id;
    let tag_info = use_async_with_options(
        async move { fetch_tag(tag_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let title_element = tag_info.data.as_ref().map_or_else(
        || html! {},
        |tag_info| {
            util::set_document_title(&format!("Tag: {}", tag_info.name));

            html! {
                <h2>{ format!("Books of \"{}\"", tag_info.name) }</h2>
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
