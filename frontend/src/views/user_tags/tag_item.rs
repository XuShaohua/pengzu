// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::module_name_repetitions)]

use crate::hooks::use_user_context;
use shared::recursive_query::RecursiveQuery;
use shared::user_tags::{NewUserTag, UserTagAndBook};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::user_tags::{add_tag, delete_tag, fetch_user_tags, update_tag};
use crate::views::user_tags::add_tag_modal::AddTagModal;
use crate::views::user_tags::edit_tag_modal::EditTagModal;

const ADD_TAG_MODAL: &str = "add-tag-modal";
const ADD_TAG_MODAL_ID: &str = "#add-tag-modal";
const EDIT_TAG_MODAL: &str = "edit-tag-modal";
const EDIT_TAG_MODAL_ID: &str = "#edit-tag-modal";

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct TagsContainerProps {
    pub tag_list: Vec<UserTagAndBook>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct AddTagReq {
    pub order_index: i32,
    pub parent: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EditTagReq {
    pub id: i32,
    pub order_index: i32,
    pub name: String,
    pub parent: i32,
}

#[function_component(TagsContainerComponent)]
pub fn tags_container(props: &TagsContainerProps) -> Html {
    let new_tag = use_state(NewUserTag::default);
    let tag_id = use_state(|| 0_i32);
    let user_ctx = use_user_context();

    // TODO(Shaohua): Handles return value.
    let add_tag_task = {
        let new_tag_clone = new_tag.clone();
        use_async(async move { add_tag(&new_tag_clone).await })
    };
    let add_tag_cb = {
        let new_tag_clone = new_tag.clone();
        let user_id = user_ctx.id;
        Callback::from(move |new_tag_name: String| {
            new_tag_clone.set(NewUserTag {
                user_id,
                order_index: new_tag_clone.order_index,
                name: new_tag_name,
                parent: new_tag_clone.parent,
            });
            add_tag_task.run();
        })
    };
    let add_tag_req = {
        let new_tag_clone = new_tag.clone();
        let user_id = user_ctx.id;
        Callback::from(move |req: AddTagReq| {
            new_tag_clone.set(NewUserTag {
                user_id,
                order_index: req.order_index,
                name: new_tag_clone.name.clone(),
                parent: req.parent,
            });
        })
    };

    let edit_tag_task = {
        let tag_id_clone = tag_id.clone();
        let new_tag_clone = new_tag.clone();
        use_async(async move { update_tag(*tag_id_clone, &new_tag_clone).await })
    };
    let edit_tag_cb = {
        let new_tag_clone = new_tag.clone();
        let user_id = user_ctx.id;
        Callback::from(move |new_tag_name: String| {
            new_tag_clone.set(NewUserTag {
                user_id,
                order_index: new_tag_clone.order_index,
                name: new_tag_name,
                parent: new_tag_clone.parent,
            });
            edit_tag_task.run();
        })
    };
    let edit_tag_req = {
        let new_tag_clone = new_tag.clone();
        let user_id = user_ctx.id;
        Callback::from(move |req: EditTagReq| {
            tag_id.set(req.id);
            new_tag_clone.set(NewUserTag {
                user_id,
                order_index: req.order_index,
                name: req.name.clone(),
                parent: req.parent,
            });
        })
    };

    let on_add_root_tag_button_click = {
        let new_tag_clone = new_tag.clone();
        Callback::from(move |_event: MouseEvent| {
            new_tag_clone.set(NewUserTag::default());
        })
    };

    html! {
        <>
         <div class="mb-3">
            <button type="button" class="btn btn-primary btn-sm"
                data-bs-toggle="modal" data-bs-target={ ADD_TAG_MODAL_ID }
                onclick={ on_add_root_tag_button_click }
                title="Add root tag">
                { "Add Root Tag" }
                <i class="bi bi-plus"></i>
            </button>
        </div>

        <div class="modal fade" tabindex="-1" id={ ADD_TAG_MODAL }>
            <AddTagModal ok_cb={ add_tag_cb } />
        </div>

        <div class="modal fade" tabindex="-1" id={ EDIT_TAG_MODAL }>
            <EditTagModal name={ new_tag.name.clone() } ok_cb={ edit_tag_cb } />
        </div>

        <TagItemListComponent
            add_tag_req={ add_tag_req }
            edit_tag_req={ edit_tag_req }
            tag_list={ props.tag_list.clone() } />
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TagItemListProps {
    pub tag_list: Vec<UserTagAndBook>,
    pub add_tag_req: Callback<AddTagReq>,
    pub edit_tag_req: Callback<EditTagReq>,
}

#[function_component(TagItemListComponent)]
pub fn tag_item_list(props: &TagItemListProps) -> Html {
    let tag_list = &props.tag_list;

    html! {
        <ol class="">
        {for tag_list.iter().map(|tag| html!{
            <li class="mb-3" key={ tag.id }>
            <UserTagItemComponent
                add_tag_req={ props.add_tag_req.clone() }
                edit_tag_req={ props.edit_tag_req.clone() }
                tag={ tag.clone() } />
            </li>
        })}
        </ol>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TagItemProps {
    pub tag: UserTagAndBook,
    pub add_tag_req: Callback<AddTagReq>,
    pub edit_tag_req: Callback<EditTagReq>,
}

#[function_component(UserTagItemComponent)]
pub fn tag_item(props: &TagItemProps) -> Html {
    let tag = &props.tag;
    let parent_id = tag.id;
    let is_deleted = use_state(|| false);

    let child_tags = use_async(async move {
        let query = RecursiveQuery {
            parent: parent_id,
            fetch_all: true,
            ..RecursiveQuery::default()
        };
        fetch_user_tags(&query).await
    });
    let onclick = {
        let child_tags = child_tags.clone();
        Callback::from(move |_event| {
            child_tags.run();
        })
    };

    let child_items = child_tags.data.as_ref().map_or_else(
        || html! {},
        |tag_list| {
            html! {
                <TagItemListComponent
                    add_tag_req={ props.add_tag_req.clone() }
                    edit_tag_req={ props.edit_tag_req.clone() }
                    tag_list={ tag_list.list.clone() } />
            }
        },
    );

    let delete_tag_task = {
        let tag_id = tag.id;
        use_async(async move {
            if *is_deleted {
                delete_tag(tag_id).await
            } else {
                is_deleted.set(true);
                Ok(())
            }
        })
    };

    let on_add_button_click = {
        let parent = tag.id;
        let order_index = 0;
        let add_tag_req_clone = props.add_tag_req.clone();
        Callback::from(move |_event: MouseEvent| {
            add_tag_req_clone.emit(AddTagReq {
                order_index,
                parent,
            });
        })
    };

    let on_edit_button_click = {
        let old_tag = tag.clone();
        let edit_tag_req_clone = props.edit_tag_req.clone();
        Callback::from(move |_event: MouseEvent| {
            edit_tag_req_clone.emit(EditTagReq {
                id: old_tag.id,
                parent: old_tag.parent,
                order_index: old_tag.order_index,
                name: old_tag.name.clone(),
            });
        })
    };

    let on_delete_button_click = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            delete_tag_task.run();
        })
    };

    html! {
        <>
            <div class="btn-group me-2" role="group">
                <button type="button" class="btn btn-success btn-sm"
                    data-bs-toggle="modal" data-bs-target={ ADD_TAG_MODAL_ID }
                    onclick={ on_add_button_click }
                    title="Add child tag">
                    <i class="bi bi-plus"></i>
                </button>
                <button type="button" class="btn btn-warning btn-sm" title="Edit tag"
                    data-bs-toggle="modal" data-bs-target={ EDIT_TAG_MODAL_ID }
                    onclick={ on_edit_button_click }>
                    <i class="bi bi-pencil"></i>
                </button>
                <button type="button" class="btn btn-danger btn-sm" title="Delete tag"
                    onclick={ on_delete_button_click }>
                    <i class="bi bi-trash3"></i>
                </button>
            </div>

            <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ tag.count }</span>
            <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id }}>
                { &tag.name }
            </Link<Route>>
            <a href="#" {onclick}><i class="bi bi-caret-right"></i></a>
            { child_items }
        </>
    }
}
