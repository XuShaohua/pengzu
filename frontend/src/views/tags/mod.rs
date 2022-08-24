// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::tags::fetch_tags;

mod tag_item;
use tag_item::TagItemComponent;

#[function_component(TagsComponent)]
pub fn tags_page() -> Html {
    let default_parent_tag_id = 0;
    let tag_list = use_async_with_options(
        async move { fetch_tags(default_parent_tag_id).await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(tag_list) = &tag_list.data {
        return html! {
            <ul>
            {for tag_list.list.iter().map(|tag| html!{
                <li class="tag-item" key={ tag.id }>
                <TagItemComponent tag={ tag.clone() } />
                </li>
            })}
            </ul>
        };
    } else {
        return html! {};
    }
}
