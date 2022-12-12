// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod category_item;

use shared::recursive_query::RecursiveQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_location;

use crate::services::categories::fetch_categories;
use crate::views::util;
use category_item::generate_category_list;

#[function_component(CategoriesComponent)]
pub fn categories() -> Html {
    util::set_document_title("Categories");

    let location = use_location().unwrap();
    let query = location.query::<RecursiveQuery>().unwrap_or_default();
    let category_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_categories(&query_clone).await })
    };
    {
        let category_list_clone = category_list.clone();
        use_effect_with_deps(
            move |_query_clone| {
                category_list_clone.run();
                || ()
            },
            query,
        );
    }

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
