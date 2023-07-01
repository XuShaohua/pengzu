// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod add_publisher_modal;
mod delete_publisher_modal;
mod edit_publisher_item;
mod edit_publisher_modal;
mod publisher_item;

use shared::general_query::{GeneralOrder, GeneralQuery};
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, use_navigator, Link};

use self::edit_publisher_item::EditPublishersContainerComponent;
use self::publisher_item::generate_publisher_list;
use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::hooks::use_user_context;
use crate::router::Route;
use crate::services::publishers::fetch_publishers;
use crate::views::util;

#[function_component(PublishersComponent)]
pub fn publishers_page() -> Html {
    util::set_document_title("Publishers");

    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GeneralQuery>().unwrap_or_default();
    let publisher_list = {
        let query_clone = query.clone();
        use_async(async move {
            util::scroll_to_top();
            fetch_publishers(&query_clone).await
        })
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

    let on_filter_change = {
        Callback::from(move |order: GeneralOrder| {
            let new_query = GeneralQuery { order, ..query };
            let ret = navigator.push_with_query(&Route::Publisher, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let pagination_link = {
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = GeneralQuery {
                    page: page_id,
                    ..query
                };
                html! {
                    <Link<Route, GeneralQuery> to={ Route::Publisher }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, GeneralQuery>>
                }
            },
        )
    };

    publisher_list.data.as_ref().map_or_else(
        || html! {},
        |publisher_list| {
            html! {
                <>
                <h2>{ "Publishers" }</h2>
                <GeneralFilterComponent onchange={ on_filter_change } current_order={ query.order } />

                <div class="container-fluid">
                    <div class="row">
                       {if user_ctx.is_admin() {
                            html!{
                                <EditPublishersContainerComponent publishers={ publisher_list.list.clone() } />
                            }
                        } else {
                            generate_publisher_list(&publisher_list.list)
                        }}
                    </div>
                </div>

                <PaginationComponent  current_page={ publisher_list.page.page_num }
                    total_pages={ publisher_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
