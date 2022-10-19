// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books;
use crate::services::query::{GetBooksOrder, GetBooksQuery};

#[function_component(BooksComponent)]
pub fn books() -> Html {
    let query = Some(GetBooksQuery {
        page: 2,
        order: GetBooksOrder::IdDesc,
    });
    let book_list = use_async_with_options(
        async move { fetch_books(query).await },
        UseAsyncOptions::enable_auto(),
    );

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                <h2>{ "Books" }</h2>
                <BookListComponent books={ book_list.list.clone() } />
                </>
            }
        },
    )
}
