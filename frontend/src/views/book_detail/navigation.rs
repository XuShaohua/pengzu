// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::views::book_detail::Route;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub previous_book: Option<i32>,
    pub next_book: Option<i32>,
}

#[function_component(NavigationComponent)]
pub fn navigation(props: &Props) -> Html {
    let previous_button = props.previous_book.map_or_else(
        || html! { <span class="page-link disabled">{ "« Previous" }</span> },
        |book_id| {
            html! {
                <Link<Route> to={ Route::BookDetail { book_id } } classes="page-link">
                    { "« Previous" }
                </Link<Route>>
            }
        },
    );
    let next_button = props.next_book.map_or_else(
        || html! { <span class="page-link disabled">{ "Next »" }</span> },
        |book_id| {
            html! {
                <Link<Route> to={ Route::BookDetail { book_id } } classes="page-link" >
                    { "Next »" }
                </Link<Route>>
            }
        },
    );

    html! {
        <ul class="pagination justify-content-center">
            <li class="page-item">
                { previous_button }
            </li>
            <li class="page-item">
                { next_button }
            </li>
        </ul>
    }
}
