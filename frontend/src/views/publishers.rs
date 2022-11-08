// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_history, History, Link, Location};

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::publishers::fetch_publishers;
use crate::types::general_query::{GeneralOrder, GeneralQuery};
use crate::types::page::PageId;
use crate::views::util;

#[function_component(PublishersComponent)]
pub fn publishers_page() -> Html {
    util::set_document_title("Publishers");

    let history = use_history().unwrap();
    let location = history.location();
    let query = location.query::<GeneralQuery>().unwrap_or_default();
    let publisher_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_publishers(&query_clone).await })
    };
    {
        let publisher_list_clone = publisher_list.clone();
        use_effect_with_deps(
            move |_query_clone| {
                publisher_list_clone.run();
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

            let new_query = GeneralQuery {
                page: page_id,
                ..query_clone
            };
            let ret = history.push_with_query(Route::Publisher, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    publisher_list.data.as_ref().map_or_else(
        || html! {},
        |publisher_list| {
            html! {
                <>
                <h2>{ "Publishers" }</h2>
                <GeneralFilterComponent onchange={ filter_onchange } current_order={ query.order } />
                <ul class="publisher-list">
                {for publisher_list.list.iter().map(|publisher| html!{
                    <li class="publisher-item" key={ publisher.id }>
                        <span class="badge">{ publisher.count }</span>
                        <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id } }>
                            { &publisher.name }
                        </Link<Route>>
                    </li>
                })}
                </ul>
                <PaginationComponent  current_page={ publisher_list.page.page_num }
                    total_pages={ publisher_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
