// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::publishers::fetch_publishers;
use crate::views::util;

#[function_component(PublishersComponent)]
pub fn publishers_page() -> Html {
    util::set_document_title("Publishers");

    let publisher_list = use_async_with_options(
        async move { fetch_publishers().await },
        UseAsyncOptions::enable_auto(),
    );

    publisher_list.data.as_ref().map_or_else(
        || html! {},
        |publisher_list| {
            html! {
                <>
                <h2>{ "Publishers" }</h2>
                <ul class="publisher-list">
                {for publisher_list.list.iter().map(|publisher| html!{
                    <li class="publisher-item" key={ publisher.id }>
                        <span class="badge">{ publisher.count }</span>
                        <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id } }>
                            { &publisher.name }
                        </Link<Route>>
                    </li>
                })}
                </ul>
                </>
            }
        },
    )
}
