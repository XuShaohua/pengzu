// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books;

#[function_component(BooksComponent)]
pub fn books() -> Html {
    let book_list = use_async_with_options(
        async move { fetch_books().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(book_list) = &book_list.data {
        return html! {
            <BookListComponent books={ book_list.list.clone() } />
        };
    } else {
        return html! {};
    }
}
