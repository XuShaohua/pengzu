// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::*;

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_simple_search;
use crate::types::simple_search::SimpleSearchQuery;

#[function_component(BooksOfSimpleSearchComponent)]
pub fn books_of_simple_search() -> Html {
    let location = use_location().unwrap();
    let query = location
        .query::<SimpleSearchQuery>()
        .expect("Failed to parse query params");

    let keyword = query.query.clone();
    let book_list = use_async_with_options(
        async move { fetch_books_by_simple_search(&query).await },
        UseAsyncOptions::enable_auto(),
    );

    book_list.data.as_ref().map_or_else(
        || {
            html! {
                <h2>{ "Result for \"" }{ &keyword }{ "\""}</h2>
            }
        },
        |book_list| {
            html! {
                <>
                <h2>{ book_list.page.total }{ " Results for \""}{ &keyword }{"\""}</h2>
                <BookListComponent books={ book_list.list.clone() } />
                </>
            }
        },
    )
}
