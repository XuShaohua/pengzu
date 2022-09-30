// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_author;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub author_id: i32,
}

#[function_component(BooksOfAuthorComponent)]
pub fn books_of_author(props: &Props) -> Html {
    let author_id = props.author_id;
    let book_list = use_async_with_options(
        async move { fetch_books_by_author(author_id).await },
        UseAsyncOptions::enable_auto(),
    );

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <BookListComponent books={ book_list.list.clone() } />
            }
        },
    )
}
