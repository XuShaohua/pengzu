// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::history::Location;
use yew_router::hooks::use_location;

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_advanced_search;
use crate::types::advanced_search::AdvancedSearchQuery;
use crate::views::util;

#[function_component(BooksOfAdvancedSearchComponent)]
pub fn books_of_advanced_search() -> Html {
    let location = use_location().unwrap();
    let query = location.query::<AdvancedSearchQuery>().unwrap_or_default();

    let query_desc = query.desc();
    util::set_document_title(&format!("Advanced Search: {}", query_desc));

    let book_list = use_async_with_options(
        async move { fetch_books_by_advanced_search(&query).await },
        UseAsyncOptions::enable_auto(),
    );

    book_list.data.as_ref().map_or_else(
        || {
            html! {
                <h2>{ "Result for \"" }{ &query_desc }{ "\"" }</h2>
            }
        },
        |book_list| {
            html! {
                <>
                <h2>{ book_list.page.total }{ " Results for \"" }{ &query_desc }{"\""}</h2>
                <BookListComponent books={ book_list.list.clone() } />
                </>
            }
        },
    )
}
