// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::components::book_list::BookListComponent;
use crate::services::books::fetch_books_by_series;

#[derive(Debug, Clone, PartialEq, Properties)]
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

    if let Some(book_list) = &book_list.data {
        return html! {
            <BookListComponent books={ book_list.list.clone() } />
        };
    } else {
        return html! {};
    }
}
