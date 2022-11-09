// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};
use yew_router::history::{History, Location};
use yew_router::hooks::use_history;

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::books::fetch_books_by_series;
use crate::services::series::fetch_series;
use crate::types::books_query::{GetBooksOrder, GetBooksQuery};
use crate::views::util;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub series_id: i32,
}

#[function_component(BooksOfSeriesComponent)]
pub fn books_of_series(props: &Props) -> Html {
    util::set_document_title(&format!("Series: {}", props.series_id));

    let history = use_history().unwrap();
    let location = history.location();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let series_id = props.series_id;
        let query_clone = query.clone();
        use_async(async move { fetch_books_by_series(series_id, &query_clone).await })
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

    let series_info = {
        let series_id = props.series_id;
        use_async_with_options(
            async move { fetch_series(series_id).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let title_element = series_info.data.as_ref().map_or_else(
        || html! {},
        |series_info| {
            util::set_document_title(&format!("Series: {}", series_info.name));

            html! {
                <h2>{ format!("Books of \"{}\"", series_info.name) }</h2>
            }
        },
    );

    let book_filter_onchange = {
        Callback::from(|order: GetBooksOrder| {
            log::info!("new order: {:?}", order);
        })
    };

    let pagination_onclick = {
        let series_id = props.series_id;
        Callback::from(move |page_id: PageId| {
            util::scroll_to_top();

            let new_query = GetBooksQuery {
                page: page_id,
                ..query
            };
            let ret = history.push_with_query(Route::BooksOfSeries { series_id }, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                { title_element }
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
