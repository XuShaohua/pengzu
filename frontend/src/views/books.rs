// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books_query::{GetBooksOrder, GetBooksQuery};
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::hooks::{use_location, use_navigator};

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::books::fetch_books;
use crate::views::util;

#[function_component(BooksComponent)]
pub fn books() -> Html {
    util::set_document_title("Books");

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_books(&query_clone).await })
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
        let query_clone = query.clone();
        let navigator_clone = navigator.clone();
        Callback::from(move |order: GetBooksOrder| {
            util::scroll_to_top();

            let new_query = GetBooksQuery {
                order,
                ..query_clone
            };
            let ret = navigator_clone.push_with_query(&Route::Book, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let pagination_onclick = Callback::from(move |page_id: PageId| {
        util::scroll_to_top();

        let new_query = GetBooksQuery {
            page: page_id,
            ..query
        };
        let ret = navigator.push_with_query(&Route::Book, &new_query);
        debug_assert!(ret.is_ok());
    });

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                <h2>{ "Books" }</h2>
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
