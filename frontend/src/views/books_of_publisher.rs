// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_publisher;
use crate::services::publishers::fetch_publisher;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub publisher_id: i32,
}

#[function_component(BooksOfPublisherComponent)]
pub fn books_of_publisher(props: &Props) -> Html {
    let publisher_id = props.publisher_id;
    let book_list = use_async_with_options(
        async move { fetch_books_by_publisher(publisher_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let publisher_id = props.publisher_id;
    let publisher_info = use_async_with_options(
        async move { fetch_publisher(publisher_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let title_element = if let Some(publisher_info) = &publisher_info.data {
        html! {
            <h2>{ format!("Books of \"{}\"", publisher_info.name) }</h2>
        }
    } else {
        html! {}
    };

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
