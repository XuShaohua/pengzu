// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::categories::{CategoryAndBook, CategoryAndBookList};
use shared::recursive_query::RecursiveQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::router::Route;
use crate::services::categories::fetch_categories;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub category: CategoryAndBook,
}

pub fn generate_category_list(category_list: &CategoryAndBookList) -> Html {
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

    let child_categories = use_async(async move {
        let query = RecursiveQuery {
            parent: parent_id,
            fetch_all: true,
            ..RecursiveQuery::default()
        };
        fetch_categories(&query).await
    });
    let onclick = {
        let child_categories = child_categories.clone();
        Callback::from(move |_event| {
            child_categories.run();
        })
    };

    let child_items = child_categories
        .data
        .as_ref()
        .map_or_else(|| html! {}, generate_category_list);

    html! {
        <>
            <span class="badge">{ category.count }</span>
            <span>{ &category.serial_number }</span>
            <Link<Route> to={ Route::BooksOfCategory { category_id: category.id }}>
                { &category.name }
            </Link<Route>>
            <button {onclick} >{ "˃" }</button>
            { child_items }
        </>
    }
}
