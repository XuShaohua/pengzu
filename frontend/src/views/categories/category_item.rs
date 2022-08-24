// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::categories::fetch_categories;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::types::categories::{CategoryAndBook, CategoryList};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub category: CategoryAndBook,
}

pub fn generate_category_list(category_list: &CategoryList) -> Html {
    html! {
        <ul class="child-categories">
        {for category_list.list.iter().map(|category| html!{
            <li class="category-item" key={ category.id }>
            <CategoryItemComponent category={ category.clone() } />
            </li>
        })}
        </ul>
    }
}

#[function_component(CategoryItemComponent)]
pub fn category_item(props: &Props) -> Html {
    let category = &props.category;
    let parent_id = category.id;

    let child_categories = { use_async(async move { fetch_categories(parent_id).await }) };
    let onclick = {
        let child_categories = child_categories.clone();
        Callback::from(move |_event| {
            child_categories.run();
        })
    };

    let child_items = if let Some(category_list) = &child_categories.data {
        generate_category_list(category_list)
    } else {
        html! {}
    };

    return html! {
        <>
            <span class="badge">{ category.count }</span>
            <span>{ &category.serial_number }</span>
            <a href="#">{ &category.name }</a>
            <button {onclick} >{ "˃" }</button>
            { child_items }
        </>
    };
}
