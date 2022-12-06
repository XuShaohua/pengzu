// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::views::book_detail::Route;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub previous_book: Option<i32>,
    pub next_book: Option<i32>,
}

#[function_component(NavigationComponent)]
pub fn navigation(props: &Props) -> Html {
    let previous_button = props.previous_book.map_or_else(
        || html! {},
        |book_id| {
            html! {
                <div class="previous-button">
                    <Link<Route> to={ Route::BookDetail { book_id } }>{ "« Previous" }</Link<Route>>
                </div>
            }
        },
    );
    let next_button = props.next_book.map_or_else(
        || html! {},
        |book_id| {
            html! {
                <div class="next-button">
                    <Link<Route> to={ Route::BookDetail { book_id } }>{ "Next »" }</Link<Route>>
                </div>
            }
        },
    );

    html! {
        <div class="button-group">
            { previous_button }
            { next_button }
        </div>
    }
}
