// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books_query::GetBooksOrder;
use shared::page::PageId;
use shared::simple_search::SimpleSearchQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, use_navigator, Link};

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

    let on_book_filter_change = {
        let query_clone = query.clone();
        Callback::from(move |order: GetBooksOrder| {
            let new_query = SimpleSearchQuery {
                order,
                ..query_clone.clone()
            };
            let ret = navigator.push_with_query(&Route::BooksOfSimpleSearch, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let pagination_link = {
        let query_clone = query.clone();
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = SimpleSearchQuery {
                    page: page_id,
                    ..query_clone.clone()
                };
                html! {
                    <Link<Route, SimpleSearchQuery> to={ Route::BooksOfSimpleSearch }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, SimpleSearchQuery>>
                }
            },
        )
    };

    let keyword = &query.query;
    book_list.data.as_ref().map_or_else(
        || {
            html! {
                <h2>{ "Result for search \"" }{ &keyword }{ "\""}</h2>
            }
        },
        |book_list| {
            html! {
                <>
                <h2>{ book_list.page.total }{ " results of search \""}{ &keyword }{"\""}</h2>
                <BookFilterComponent onchange={ on_book_filter_change } current_order={ query.order } />
                <BookListComponent books={ book_list.list.clone() } />
                <PaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
