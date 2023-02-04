// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::publishers::Publisher;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
    pub publisher: Option<Publisher>,
}

#[function_component(EditPublisherComponent)]
pub fn edit_publisher(props: &Props) -> Html {
    let publisher = &props.publisher;
    publisher.as_ref().map_or_else(
        || html! {<></>},
        |publisher| {
            html! {
                <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id }}
                    classes="col-sm-10">
                    { &publisher.name }
                </Link<Route>>
            }
        },
    )
}
