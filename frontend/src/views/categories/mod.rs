// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::categories::fetch_categories;

mod category_item;
use category_item::CategoryItemComponent;

#[function_component(CategoriesComponent)]
pub fn categories() -> Html {
    let _default_parent_id = 0;
    let category_list = use_async_with_options(
        async move { fetch_categories(_default_parent_id).await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(category_list) = &category_list.data {
        return html! {
            <ul>
            {for category_list.list.iter().map(|category| html!{
                <li class="category-item" key={ category.id }>
                <CategoryItemComponent category={ category.clone() } />
                </li>
            })}
            </ul>
        };
    } else {
        return html! {};
    }
}
