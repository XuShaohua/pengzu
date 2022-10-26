// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::history::Location;
use yew_router::hooks::use_location;

use crate::components::book_list::BookListComponent;
use crate::components::book_pagination::BookPaginationComponent;
use crate::services::books::fetch_books;
use crate::types::books::GetBooksQuery;
use crate::types::page::PageId;
use crate::views::util;

#[function_component(BooksComponent)]
pub fn books() -> Html {
    util::set_document_title("Books");

    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = use_async_with_options(
        async move { fetch_books(&query).await },
        UseAsyncOptions::enable_auto(),
    );

    let pagination_onclick = Callback::from(|page_id: PageId| {
        log::info!("page clicked {}", page_id);
    });

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            log::info!("page: {:?}", book_list.page);
            html! {
                <>
                <h2>{ "Books" }</h2>
                <BookListComponent books={ book_list.list.clone() } />
                <BookPaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
