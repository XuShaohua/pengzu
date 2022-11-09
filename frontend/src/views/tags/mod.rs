// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_history, History, Location};

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::tags::fetch_tags;
use crate::types::recursive_query::RecursiveQuery;
use crate::views::util;
use shared::general_query::GeneralOrder;
use shared::page::PageId;

mod tag_item;
use tag_item::generate_tag_list;

#[function_component(TagsComponent)]
pub fn tags_page() -> Html {
    util::set_document_title("Tags");

    let history = use_history().unwrap();
    let location = history.location();
    let query = location.query::<RecursiveQuery>().unwrap_or_default();
    let tag_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_tags(&query_clone).await })
    };
    {
        let tag_list_clone = tag_list.clone();
        use_effect_with_deps(
            move |_query_clone| {
                tag_list_clone.run();
                || ()
            },
            query.clone(),
        );
    }

    let filter_onchange = {
        Callback::from(|order: GeneralOrder| {
            log::info!("new order: {:?}", order);
        })
    };

    let pagination_onclick = {
        let query_clone = query.clone();
        Callback::from(move |page_id: PageId| {
            util::scroll_to_top();

            let new_query = RecursiveQuery {
                page: page_id,
                ..query_clone
            };
            let ret = history.push_with_query(Route::Tag, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    tag_list.data.as_ref().map_or_else(
        || html! {},
        |tag_list| {
            html! {
                <>
                <h2>{ "Tags" }</h2>
                <GeneralFilterComponent onchange={ filter_onchange } current_order={ query.order } />

                { generate_tag_list(tag_list) }

                <PaginationComponent  current_page={ tag_list.page.page_num }
                    total_pages={ tag_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
