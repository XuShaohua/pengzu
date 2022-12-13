// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use crate::components::book_list::BookListComponent;
use crate::services::discover::fetch_books_by_discover;
use crate::views::util;

#[function_component(BooksOfDiscoverComponent)]
pub fn books_of_discover() -> Html {
    let title = "Discover (Random Books)";
    util::set_document_title(title);

    let book_list = use_async(async move {
        util::scroll_to_top();
        fetch_books_by_discover().await
    });
    {
        let book_list_clone = book_list.clone();
        use_mount(move || {
            book_list_clone.run();
        });
    }

    let on_refresh_click = {
        let book_list_clone = book_list.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            book_list_clone.run();
        })
    };

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

                <ul class="mt-3 pagination justify-content-center">
                    <li class="page-item">
                        <a href="#" onclick={ on_refresh_click } class="page-link active">{ "Refresh" }</a>
                    </li>
                </ul>
                </>
            }
        },
    )
}
