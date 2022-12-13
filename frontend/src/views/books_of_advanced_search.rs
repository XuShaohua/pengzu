// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::advanced_search::AdvancedSearchQuery;
use shared::books_query::GetBooksOrder;
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, Link};

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::advanced_search::fetch_books_by_advanced_search;
use crate::views::util;

#[function_component(BooksOfAdvancedSearchComponent)]
pub fn books_of_advanced_search() -> Html {
    let location = use_location().unwrap();
    let query = location.query::<AdvancedSearchQuery>().unwrap_or_default();
    let query_desc = query.desc();
    util::set_document_title(&format!("Advanced Search: {}", query_desc));

    let book_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_books_by_advanced_search(&query_clone).await })
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

    let pagination_link = {
        let query_clone = query.clone();
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = AdvancedSearchQuery {
                    page: page_id,
                    ..query_clone.clone()
                };
                html! {
                    <Link<Route, AdvancedSearchQuery> to={ Route::BooksOfAdvancedSearch }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, AdvancedSearchQuery>>
                }
            },
        )
    };

    book_list.data.as_ref().map_or_else(
        || {
            html! {
                <h2>{ "Result of search \"" }{ &query_desc }{ "\"" }</h2>
            }
        },
        |book_list| {
            html! {
                <>
                <h2>{ book_list.page.total }{ " results of search \"" }{ &query_desc }{"\""}</h2>
                <BookFilterComponent onchange={ book_filter_onchange } current_order={ query.order } />
                <BookListComponent books={ book_list.list.clone() } />
                <PaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
