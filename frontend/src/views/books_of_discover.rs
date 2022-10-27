// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_discover;
use crate::views::util;

#[function_component(BooksOfDiscoverComponent)]
pub fn books_of_discover() -> Html {
    let title = "Discover (Random Books)";
    util::set_document_title(title);

    let book_list = use_async_with_options(
        async move { fetch_books_by_discover().await },
        UseAsyncOptions::enable_auto(),
    );

    book_list.data.as_ref().map_or_else(
        || {
            html! {
                <h2>{ title }</h2>
            }
        },
        |book_list| {
            html! {
                <>
                <h2>{ title }</h2>
                <BookListComponent books={ book_list.list.clone() } />
                </>
            }
        },
    )
}
