// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::recursive_query::RecursiveQuery;
use shared::tags::{NewTag, TagAndBook};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::tags::{add_tag, delete_tag, fetch_tags};
use crate::views::tags::add_tag_form::AddTagFormComponent;

const ADD_TAG_MODAL: &str = "add-tag-modal";
const ADD_TAG_MODAL_ID: &str = "#add-tag-modal";

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ItemListProps {
    pub tag_list: Vec<TagAndBook>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct TagProp {
    pub parent: i32,
    pub order_index: i32,
}

#[function_component(EditTagItemListComponent)]
pub fn edit_tag_item_list(props: &ItemListProps) -> Html {
    let tag_list = &props.tag_list;
    let new_tag = use_state(NewTag::default);

    // TODO(Shaohua): Handles return value.
    let add_tag_task = {
        let new_tag_clone = new_tag.clone();
        use_async(async move { add_tag(&new_tag_clone).await })
    };

    let add_tag_cb = {
        let new_tag_clone = new_tag.clone();
        Callback::from(move |new_tag_name: String| {
            log::info!("new tag: {new_tag_name:?}");
            new_tag_clone.set(NewTag {
                order_index: new_tag_clone.order_index,
                name: new_tag_name,
                parent: new_tag_clone.parent,
            });
            add_tag_task.run();
        })
    };

    let add_tag_req = {
        Callback::from(move |new_tag_extra: TagProp| {
            new_tag.set(NewTag {
                order_index: new_tag_extra.order_index,
                name: new_tag.name.clone(),
                parent: new_tag_extra.parent,
            });
        })
    };

    html! {
        <>
        <div class="modal fade" tabindex="-1" id={ ADD_TAG_MODAL }>
            <AddTagFormComponent ok_cb={ add_tag_cb } />
        </div>

        <ol class="">
        {for tag_list.iter().map(|tag| html!{
            <li class="mb-2" key={ tag.id }>
                <EditTagItemComponent add_tag_req={ add_tag_req.clone() }
                    tag={ tag.clone() } />
            </li>
        })}
        </ol>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemProps {
    pub tag: TagAndBook,
    pub add_tag_req: Callback<TagProp>,
}

#[function_component(EditTagItemComponent)]
pub fn edit_tag_item(props: &ItemProps) -> Html {
    let tag = &props.tag;
    let parent_id = tag.id;

    let is_deleted = use_state(|| false);
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
                <EditTagItemListComponent tag_list={ tag_list.list.clone() } />
            }
        },
    );
    let on_tag_click = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            fetch_child_tags_task.run();
        })
    };

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
            add_tag_req_clone.emit(TagProp {
                parent,
                order_index,
            });
        })
    };

    let on_edit_button_click = {
        let old_tag = tag.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let new_tag = NewTag {
                order_index: old_tag.order_index,
                name: String::new(),
                parent: old_tag.parent,
            };
            log::info!("edit new tag: {new_tag:?}");
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
