// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::history::{History, Location};
use yew_router::hooks::{use_history, use_location};

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::book_pagination::BookPaginationComponent;
use crate::router::Route;
use crate::services::books::fetch_books_by_advanced_search;
use crate::types::advanced_search::AdvancedSearchQuery;
use crate::types::books::GetBooksOrder;
use crate::types::page::PageId;
use crate::views::util;

#[function_component(BooksOfAdvancedSearchComponent)]
pub fn books_of_advanced_search() -> Html {
    let history = use_history().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<AdvancedSearchQuery>().unwrap_or_default();
    let query_desc = query.desc();
    util::set_document_title(&format!("Advanced Search: {}", query_desc));

    let book_list = {
        let query_clone = query.clone();
        use_async_with_options(
            async move { fetch_books_by_advanced_search(&query_clone).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let book_filter_onchange = {
        Callback::from(|order: GetBooksOrder| {
            log::info!("new order: {:?}", order);
        })
    };

    let pagination_onclick = {
        Callback::from(move |page_id: PageId| {
            let new_query = AdvancedSearchQuery {
                page: page_id,
                ..query.clone()
            };
            let ret = history.push_with_query(Route::BooksOfAdvancedSearch, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

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
                <BookFilterComponent onchange={ book_filter_onchange }/>
                <BookListComponent books={ book_list.list.clone() } />
                <BookPaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
