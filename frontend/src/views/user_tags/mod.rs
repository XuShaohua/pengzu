// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::user_tags::fetch_user_tags;
use crate::views::util;

mod tag_item;
use tag_item::generate_tag_list;

#[function_component(UserTagsComponent)]
pub fn user_tags_page() -> Html {
    util::set_document_title("User Tags");

    let default_parent_tag_id = 0;
    let tag_list = use_async_with_options(
        async move { fetch_user_tags(default_parent_tag_id).await },
        UseAsyncOptions::enable_auto(),
    );

    tag_list
        .data
        .as_ref()
        .map_or_else(|| html! {}, generate_tag_list)
}
