// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::router::Route;
use shared::books_query::GetBooksQuery;
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, Link};

use crate::components::book_list::BookListComponent;
use crate::components::pagination::PaginationComponent;
use crate::services::download_history::fetch_books_by_download_history;
use crate::views::util;

#[function_component(BooksOfDownloadHistoryComponent)]
pub fn books_of_user_tag() -> Html {
    util::set_document_title("Download History");

    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_books_by_download_history(&query_clone).await })
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

    let pagination_link = {
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = GetBooksQuery {
                    page: page_id,
                    ..query
                };
                html! {
                    <Link<Route, GetBooksQuery> to={ Route::BooksOfDownloadHistory }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, GetBooksQuery>>
                }
            },
        )
    };

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                <h2>{ "Download history" }</h2>
                <BookListComponent books={ book_list.list.clone() } />
                <PaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
