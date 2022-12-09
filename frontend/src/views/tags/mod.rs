// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::general_query::GeneralOrder;
use shared::page::PageId;
use shared::recursive_query::RecursiveQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, Link};

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::tags::fetch_tags;
use crate::views::util;

mod tag_item;
use tag_item::generate_tag_list;

#[function_component(TagsComponent)]
pub fn tags_page() -> Html {
    util::set_document_title("Tags");

    let location = use_location().unwrap();
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

    let pagination_link = {
        let query_clone = query.clone();
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = RecursiveQuery {
                    page: page_id,
                    ..query_clone
                };
                html! {
                    <Link<Route, RecursiveQuery> to={ Route::Tag }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, RecursiveQuery>>
                }
            },
        )
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
                    link={ pagination_link } />
                </>
            }
        },
    )
}
