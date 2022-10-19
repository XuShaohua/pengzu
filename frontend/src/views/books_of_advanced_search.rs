// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_advanced_search;
use crate::types::advanced_search::AdvancedSearchQuery;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub query: AdvancedSearchQuery,
    pub page_id: i32,
}

#[function_component(BooksOfAdvancedSearchComponent)]
pub fn books_of_advanced_search(props: &Props) -> Html {
    let page_id = 1;
    let query = props.query.clone();
    let book_list = use_async_with_options(
        async move { fetch_books_by_advanced_search(&query, page_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let title_element = html! {
        <h2>{ format!("Books of \"{}\"", props.query.desc()) }</h2>
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
