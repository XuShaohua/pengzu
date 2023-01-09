// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::publishers::{NewPublisher, PublisherAndBook};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::publishers::{add_publisher, delete_publisher, update_publisher};
use crate::views::publishers::add_publisher_modal::AddPublisherModal;
use crate::views::publishers::edit_publisher_modal::EditPublisherModal;

const ADD_PUBLISHER_MODAL: &str = "add-publisher-modal";
const ADD_PUBLISHER_MODAL_ID: &str = "#add-publisher-modal";
const EDIT_PUBLISHER_MODAL: &str = "edit-publisher-modal";
const EDIT_PUBLISHER_MODAL_ID: &str = "#edit-publisher-modal";

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ContainerProps {
    pub publishers: Vec<PublisherAndBook>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EditPublisherReq {
    pub id: i32,
    pub name: String,
}

#[function_component(EditPublishersContainerComponent)]
pub fn edit_publishers_container(props: &ContainerProps) -> Html {
    let new_publisher = use_state(NewPublisher::default);
    let publisher_id = use_state(|| 0_i32);

    let add_publisher_task = {
        let new_publisher_clone = new_publisher.clone();
        use_async(async move { add_publisher(&new_publisher_clone).await })
    };
    let add_publisher_cb = {
        let new_publisher_clone = new_publisher.clone();
        Callback::from(move |new_publisher_name: String| {
            new_publisher_clone.set(NewPublisher {
                name: new_publisher_name,
            });
            add_publisher_task.run();
        })
    };

    let edit_publisher_task = {
        let publisher_id_clone = publisher_id.clone();
        let new_publisher_clone = new_publisher.clone();
        use_async(async move { update_publisher(*publisher_id_clone, &new_publisher_clone).await })
    };
    let edit_publisher_cb = {
        let new_publisher_clone = new_publisher.clone();
        Callback::from(move |new_publisher_name: String| {
            new_publisher_clone.set(NewPublisher {
                name: new_publisher_name,
            });
            edit_publisher_task.run();
        })
    };
    let edit_publisher_req = {
        let new_publisher_clone = new_publisher.clone();
        Callback::from(move |req: EditPublisherReq| {
            publisher_id.set(req.id);
            new_publisher_clone.set(NewPublisher { name: req.name });
        })
    };

    let on_add_publisher_button_click = {
        let new_publisher_clone = new_publisher.clone();
        Callback::from(move |_event: MouseEvent| {
            new_publisher_clone.set(NewPublisher::default());
        })
    };

    html! {
        <>
        <div class="mb-3">
            <button type="button" class="btn btn-primary btn-sm"
                data-bs-toggle="modal" data-bs-target={ ADD_PUBLISHER_MODAL_ID }
                onclick={ on_add_publisher_button_click }
                title="Add publisher">
                { "Add Publisher" }
                <i class="bi bi-plus"></i>
            </button>
        </div>

        <div class="modal fade" tabindex="-1" id={ ADD_PUBLISHER_MODAL }>
            <AddPublisherModal ok_cb={ add_publisher_cb } />
        </div>

        <div class="modal fade" tabindex="-1" id={ EDIT_PUBLISHER_MODAL }>
            <EditPublisherModal name={ new_publisher.name.clone() } ok_cb={ edit_publisher_cb } />
        </div>

        <EditPublisherItemListComponent
            edit_publisher_req={ edit_publisher_req }
            publishers={ props.publishers.clone() } />
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemListProps {
    pub publishers: Vec<PublisherAndBook>,
    pub edit_publisher_req: Callback<EditPublisherReq>,
}

#[function_component(EditPublisherItemListComponent)]
pub fn edit_publisher_item_list(props: &ItemListProps) -> Html {
    html! {
        <ul class="col-xs-12 col-sm-6 list-unstyled">
            {for props.publishers.iter().map(|publisher| html! {
                <li class="mb-3" key={ publisher.id }>
                    <EditPublisherItemComponent
                        edit_publisher_req={ props.edit_publisher_req.clone() }
                        publisher={ publisher.clone() } />
                </li>
            })}
        </ul>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemProps {
    pub publisher: PublisherAndBook,
    pub edit_publisher_req: Callback<EditPublisherReq>,
}

#[function_component(EditPublisherItemComponent)]
pub fn edit_publisher_item(props: &ItemProps) -> Html {
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

    let on_edit_button_click = {
        let publisher_clone = publisher.clone();
        let edit_publisher_req_clone = props.edit_publisher_req.clone();
        Callback::from(move |_event: MouseEvent| {
            edit_publisher_req_clone.emit(EditPublisherReq {
                id: publisher_clone.id,
                name: publisher_clone.name.clone(),
            });
        })
    };

    let on_delete_button_click = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            delete_publisher_task.run();
        })
    };

    html! {
        <>
        <div class="btn-group me-2" role="group">
            <button type="button" class="btn btn-warning btn-sm" title="Edit publisher"
                data-bs-toggle="modal" data-bs-target={ EDIT_PUBLISHER_MODAL_ID }
                onclick={ on_edit_button_click }>
                <i class="bi bi-pencil"></i>
            </button>
            <button type="button" class="btn btn-danger btn-sm" title="Delete publisher"
                onclick={ on_delete_button_click }>
                <i class="bi bi-trash3"></i>
            </button>
        </div>

        <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ publisher.count }</span>
        <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id } }>
            { &publisher.name }
        </Link<Route>>
        </>
    }
}
