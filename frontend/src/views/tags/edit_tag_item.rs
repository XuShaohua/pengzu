// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::recursive_query::RecursiveQuery;
use shared::tags::{NewTag, TagAndBook};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::tags::{add_tag, delete_tag, fetch_tags, update_tag};
use crate::views::tags::add_tag_modal::AddTagModal;
use crate::views::tags::delete_tag_modal::DeleteTagModal;
use crate::views::tags::edit_tag_modal::EditTagModal;

const ADD_TAG_MODAL: &str = "add-tag-modal";
const ADD_TAG_MODAL_ID: &str = "#add-tag-modal";
const EDIT_TAG_MODAL: &str = "edit-tag-modal";
const EDIT_TAG_MODAL_ID: &str = "#edit-tag-modal";
const DELETE_TAG_MODAL: &str = "delete-tag-modal";
const DELETE_TAG_MODAL_ID: &str = "#delete-tag-modal";

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ItemsContainerProps {
    pub tag_list: Vec<TagAndBook>,
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

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DeleteTagReq {
    pub id: i32,
    pub name: String,
}

#[function_component(EditTagsContainerComponent)]
pub fn edit_tags_container(props: &ItemsContainerProps) -> Html {
    let tag_list = &props.tag_list;
    let new_tag = use_state(NewTag::default);
    let tag_id = use_state(|| 0_i32);

    // TODO(Shaohua): Handles return value.
    let add_tag_task = {
        let new_tag_clone = new_tag.clone();
        use_async(async move { add_tag(&new_tag_clone).await })
    };
    let add_tag_cb = {
        let new_tag_clone = new_tag.clone();
        Callback::from(move |new_tag_name: String| {
            new_tag_clone.set(NewTag {
                order_index: new_tag_clone.order_index,
                name: new_tag_name,
                parent: new_tag_clone.parent,
            });
            add_tag_task.run();
        })
    };
    let add_tag_req = {
        let new_tag_clone = new_tag.clone();
        Callback::from(move |req: AddTagReq| {
            new_tag_clone.set(NewTag {
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
        Callback::from(move |new_tag_name: String| {
            new_tag_clone.set(NewTag {
                order_index: new_tag_clone.order_index,
                name: new_tag_name,
                parent: new_tag_clone.parent,
            });
            edit_tag_task.run();
        })
    };
    let edit_tag_req = {
        let tag_id_clone = tag_id.clone();
        let new_tag_clone = new_tag.clone();
        Callback::from(move |req: EditTagReq| {
            tag_id_clone.set(req.id);
            new_tag_clone.set(NewTag {
                order_index: req.order_index,
                name: req.name.clone(),
                parent: req.parent,
            });
        })
    };

    let delete_tag_task = {
        let tag_id_clone = tag_id.clone();
        use_async(async move { delete_tag(*tag_id_clone).await })
    };
    let delete_tag_cb = {
        Callback::from(move |will_delete: bool| {
            if will_delete {
                delete_tag_task.run();
            }
        })
    };
    let delete_tag_req = {
        let new_tag_clone = new_tag.clone();
        Callback::from(move |req: DeleteTagReq| {
            tag_id.set(req.id);
            new_tag_clone.set(NewTag {
                order_index: new_tag_clone.order_index,
                name: req.name,
                parent: new_tag_clone.parent,
            });
        })
    };

    let on_add_root_tag_button_click = {
        let new_tag_clone = new_tag.clone();
        Callback::from(move |_event: MouseEvent| {
            new_tag_clone.set(NewTag::default());
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

        <div class="modal fade" tabindex="-1" id={ DELETE_TAG_MODAL }>
            <DeleteTagModal name={ new_tag.name.clone() } ok_cb={ delete_tag_cb } />
        </div>

        <EditTagItemListComponent
            add_tag_req={ add_tag_req }
            edit_tag_req={ edit_tag_req }
            delete_tag_req={ delete_tag_req }
            tag_list={ tag_list.clone() } />
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemListProps {
    pub tag_list: Vec<TagAndBook>,
    pub add_tag_req: Callback<AddTagReq>,
    pub edit_tag_req: Callback<EditTagReq>,
    pub delete_tag_req: Callback<DeleteTagReq>,
}

#[function_component(EditTagItemListComponent)]
pub fn edit_tag_item_list(props: &ItemListProps) -> Html {
    let tag_list = &props.tag_list;

    html! {
       <ol class="">
       {for tag_list.iter().map(|tag| html!{
            <li class="mb-2" key={ tag.id }>
                <EditTagItemComponent
                    add_tag_req={ props.add_tag_req.clone() }
                    edit_tag_req={ props.edit_tag_req.clone() }
                    delete_tag_req={ props.delete_tag_req.clone() }
                    tag={ tag.clone() } />
            </li>
        })}
        </ol>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemProps {
    pub tag: TagAndBook,
    pub add_tag_req: Callback<AddTagReq>,
    pub edit_tag_req: Callback<EditTagReq>,
    pub delete_tag_req: Callback<DeleteTagReq>,
}

#[function_component(EditTagItemComponent)]
pub fn edit_tag_item(props: &ItemProps) -> Html {
    let tag = &props.tag;
    let parent_id = tag.id;

    let fetch_child_tags_task = use_async(async move {
        // Always fetch all tags.
        let query = RecursiveQuery {
            parent: parent_id,
            fetch_all: true,
            ..RecursiveQuery::default()
        };
        fetch_tags(&query).await
    });
    let child_items = fetch_child_tags_task.data.as_ref().map_or_else(
        || html! {},
        |tag_list| {
            html! {
                <EditTagItemListComponent
                    add_tag_req={ props.add_tag_req.clone() }
                    edit_tag_req={ props.edit_tag_req.clone() }
                    delete_tag_req={ props.delete_tag_req.clone() }
                    tag_list={ tag_list.list.clone() } />
            }
        },
    );
    let on_tag_click = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            fetch_child_tags_task.run();
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
        let old_tag = tag.clone();
        let delete_tag_req_clone = props.delete_tag_req.clone();
        Callback::from(move |_event: MouseEvent| {
            delete_tag_req_clone.emit(DeleteTagReq {
                id: old_tag.id,
                name: old_tag.name.clone(),
            });
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
                    data-bs-toggle="modal" data-bs-target={ DELETE_TAG_MODAL_ID }
                    onclick={ on_delete_button_click }>
                    <i class="bi bi-trash3"></i>
                </button>
            </div>

            <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ tag.count }</span>
            <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id }}>
                { &tag.name }
            </Link<Route>>

            {
                if tag.children > 0 {
                    html! { <a href="#" onclick={on_tag_click}><i class="bi bi-caret-right"></i></a> }
                } else {
                    html! {}
                }
            }

            { child_items }
        </>
    }
}
