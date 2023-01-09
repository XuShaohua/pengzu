// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod tag_item;

use shared::general_query::GeneralOrder;
use shared::page::PageId;
use shared::recursive_query::RecursiveQuery;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, use_navigator, Link};

use self::tag_item::TagsContainerComponent;
use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::user_tags::fetch_user_tags;
use crate::views::util;

#[function_component(UserTagsComponent)]
pub fn user_tags_page() -> Html {
    util::set_document_title("User Tags");

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<RecursiveQuery>().unwrap_or_default();
    let tag_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_user_tags(&query_clone).await })
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

    let on_filter_change = {
        let query_clone = query.clone();
        Callback::from(move |order: GeneralOrder| {
            let new_query = RecursiveQuery {
                order,
                ..query_clone
            };
            let ret = navigator.push_with_query(&Route::UserTag, &new_query);
            debug_assert!(ret.is_ok());
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
                    <Link<Route, RecursiveQuery> to={ Route::UserTag }
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
                <h2>{ "User Tags" }</h2>
                <GeneralFilterComponent onchange={ on_filter_change } current_order={ query.order } />

                <TagsContainerComponent tag_list={ tag_list.list.clone() } />

                <PaginationComponent  current_page={ tag_list.page.page_num }
                    total_pages={ tag_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
