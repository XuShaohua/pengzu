// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books_query::GetBooksOrder;
use shared::page::PageId;
use shared::simple_search::SimpleSearchQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::hooks::{use_location, use_navigator};

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::simple_search::fetch_books_by_simple_search;
use crate::views::util;

#[function_component(BooksOfSimpleSearchComponent)]
pub fn books_of_simple_search() -> Html {
    util::set_document_title("Search");

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<SimpleSearchQuery>().unwrap_or_default();
    let keyword = query.query.clone();

    let book_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_books_by_simple_search(&query_clone).await })
    };

    {
        let book_list_clone = book_list.clone();
        use_effect_with_deps(
            move |_query_clone| {
                book_list_clone.run();
                || ()
            },
            query.clone(),
        );
    }

    let book_filter_onchange = {
        Callback::from(|order: GetBooksOrder| {
            log::info!("new order: {:?}", order);
        })
    };

    let pagination_onclick = {
        let query_clone = query.clone();
        Callback::from(move |page_id: PageId| {
            util::scroll_to_top();

            let new_query = SimpleSearchQuery {
                page: page_id,
                ..query_clone.clone()
            };
            let ret = navigator.push_with_query(&Route::BooksOfSimpleSearch, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

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
                <BookFilterComponent onchange={ book_filter_onchange } current_order={ query.order } />
                <BookListComponent books={ book_list.list.clone() } />
                <PaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
