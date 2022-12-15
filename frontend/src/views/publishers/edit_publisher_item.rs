// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::publishers::PublisherAndBook;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::publishers::delete_publisher;

pub fn generate_edit_publisher_list(publisher_list: &[PublisherAndBook]) -> Html {
    html! {
        <ul class="col-xs-12 col-sm-6 list-unstyled">
            {for publisher_list.iter().map(|publisher| html! {
                <li class="mb-3" key={ publisher.id }>
                    <EditPublisherItemComponent publisher={ publisher.clone() } />
                </li>
            })}
        </ul>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub publisher: PublisherAndBook,
}

#[function_component(EditPublisherItemComponent)]
pub fn edit_publisher_item(props: &Props) -> Html {
    let publisher = &props.publisher;
    let is_deleted = use_state(|| false);

    let delete_publisher_task = {
        let publisher_id = publisher.id;
        use_async(async move {
            if *is_deleted {
                delete_publisher(publisher_id).await
            } else {
                is_deleted.set(true);
                Ok(())
            }
        })
    };
    let on_delete_button_click = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            delete_publisher_task.run();
        })
    };
    let on_edit_button_click = {
        let publisher_id = publisher.id;
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            log::info!("edit publisher: {}", publisher_id);
        })
    };

    html! {
        <>
        <button class="btn btn-secondary btn-sm me-2" onclick={ on_edit_button_click }>{ "Edit" }</button>
        <button class="btn btn-danger btn-sm me-2" onclick={ on_delete_button_click }>{ "Delete" }</button>
        <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ publisher.count }</span>
        <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id } }>
            { &publisher.name }
        </Link<Route>>
        </>
    }
}
