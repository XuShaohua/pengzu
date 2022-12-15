// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::publishers::PublisherAndBook;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

pub fn generate_publisher_list(publisher_list: &[PublisherAndBook]) -> Html {
    html! {
        <ul class="col-xs-12 col-sm-6 list-unstyled">
            {for publisher_list.iter().map(|publisher| html! {
                <li class="mb-3" key={ publisher.id }>
                    <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ publisher.count }</span>
                    <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id } }>
                        { &publisher.name }
                    </Link<Route>>
                </li>
            })}
        </ul>
    }
}
