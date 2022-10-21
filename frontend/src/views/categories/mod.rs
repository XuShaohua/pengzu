// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::categories::fetch_categories;

mod category_item;
use crate::views::util;
use category_item::generate_category_list;

#[function_component(CategoriesComponent)]
pub fn categories() -> Html {
    util::set_document_title("Categories");

    // TODO(Shaohua): Get category parent id.
    let default_parent_id = 0;
    let category_list = use_async_with_options(
        async move { fetch_categories(default_parent_id).await },
        UseAsyncOptions::enable_auto(),
    );

    category_list.data.as_ref().map_or_else(
        || html! {},
        |category_list| {
            html! {
                <>
                <h2>{ "Categories" }</h2>
                { generate_category_list(category_list) }
                </>
            }
        },
    )
}
