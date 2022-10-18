// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_series;
use crate::services::series::fetch_series;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub series_id: i32,
}

#[function_component(BooksOfSeriesComponent)]
pub fn books_of_series(props: &Props) -> Html {
    let series_id = props.series_id;
    let book_list = use_async_with_options(
        async move { fetch_books_by_series(series_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let series_id = props.series_id;
    let series_info = use_async_with_options(
        async move { fetch_series(series_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let title_element = series_info.data.as_ref().map_or_else(
        || html! {},
        |series_info| {
            html! {
                <h2>{ format!("Books of \"{}\"", series_info.name) }</h2>
            }
        },
    );

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
