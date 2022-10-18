// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_file_format;
use crate::services::file_formats::fetch_file_format;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub format_id: i32,
}

#[function_component(BooksOfFileFormatComponent)]
pub fn books_of_file_format(props: &Props) -> Html {
    let format_id = props.format_id;
    let book_list = use_async_with_options(
        async move { fetch_books_by_file_format(format_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let format_id = props.format_id;
    let format_info = use_async_with_options(
        async move { fetch_file_format(format_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let title_element = format_info.data.as_ref().map_or_else(
        || html! {},
        |format_info| {
            html! {
                <h2>{ format!("Books of \"{}\"", format_info.name) }</h2>
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
