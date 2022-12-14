// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod edit_tag_item;
mod tag_item;

use shared::general_query::GeneralOrder;
use shared::page::PageId;
use shared::recursive_query::RecursiveQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, Link};

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::hooks::use_user_context;
use crate::router::Route;
use crate::services::tags::fetch_tags;
use crate::views::util;
use edit_tag_item::generate_edit_tag_list;
use tag_item::generate_tag_list;

#[function_component(TagsComponent)]
pub fn tags_page() -> Html {
    util::set_document_title("Tags");

    let user_ctx = use_user_context();
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

                {if user_ctx.is_admin() {
                    generate_edit_tag_list(&tag_list.list)
                } else {
                    generate_tag_list(&tag_list.list)
                }}

                <PaginationComponent  current_page={ tag_list.page.page_num }
                    total_pages={ tag_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
