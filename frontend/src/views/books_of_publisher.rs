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
use crate::services::books::fetch_books_by_publisher;
use crate::services::publishers::fetch_publisher;
use crate::types::books::{GetBooksOrder, GetBooksQuery};
use crate::types::page::PageId;
use crate::views::util;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub publisher_id: i32,
}

#[function_component(BooksOfPublisherComponent)]
pub fn books_of_publisher(props: &Props) -> Html {
    util::set_document_title(&format!("Publisher: {}", props.publisher_id));

    let history = use_history().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let publisher_id = props.publisher_id;
        let query_clone = query.clone();
        use_async_with_options(
            async move { fetch_books_by_publisher(publisher_id, &query_clone).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let publisher_info = {
        let publisher_id = props.publisher_id;
        use_async_with_options(
            async move { fetch_publisher(publisher_id).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let title_element = publisher_info.data.as_ref().map_or_else(
        || html! {},
        |publisher_info| {
            util::set_document_title(&format!("Publisher: {}", publisher_info.name));

            html! {
                <h2>{ format!("Books of \"{}\"", publisher_info.name) }</h2>
            }
        },
    );

    let book_filter_onchange = {
        Callback::from(|order: GetBooksOrder| {
            log::info!("new order: {:?}", order);
        })
    };

    let pagination_onclick = {
        let publisher_id = props.publisher_id;
        Callback::from(move |page_id: PageId| {
            let new_query = GetBooksQuery {
                page: page_id,
                ..query
            };
            let ret = history.push_with_query(Route::BooksOfPublisher { publisher_id }, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                { title_element }
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
