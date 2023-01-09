// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::recursive_query::RecursiveQuery;
use shared::tags::{NewTag, TagAndBook};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::tags::{delete_tag, fetch_tags};

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub tag: TagAndBook,
}

pub fn generate_edit_tag_list(tag_list: &[TagAndBook]) -> Html {
    html! {
        <ol class="">
        {for tag_list.iter().map(|tag| html!{
            <li class="mb-2" key={ tag.id }>
                <EditTagItemComponent tag={ tag.clone() } />
            </li>
        })}
        </ol>
    }
}

#[function_component(EditTagItemComponent)]
pub fn edit_tag_item(props: &Props) -> Html {
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
        |tag_list| generate_edit_tag_list(&tag_list.list),
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
        let tag_id = tag.id;
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            log::info!("add tag: {}", tag_id);
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
                <button type="button" class="btn btn-success btn-sm" title="Add child tag"
                    onclick={ on_add_button_click }>
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
