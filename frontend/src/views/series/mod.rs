// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod add_series_modal;
mod edit_series_item;
mod edit_series_modal;
mod series_item;

use shared::general_query::{GeneralOrder, GeneralQuery};
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, use_navigator, Link};

use self::edit_series_item::EditSeriesContainerComponent;
use self::series_item::generate_series_list;
use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::hooks::use_user_context;
use crate::router::Route;
use crate::services::series::fetch_series_list;
use crate::views::util;

#[function_component(SeriesComponent)]
pub fn series_page() -> Html {
    util::set_document_title("Series");

    let user_ctx = use_user_context();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GeneralQuery>().unwrap_or_default();
    let series_list = {
        let query_clone = query.clone();
        use_async(async move {
            util::scroll_to_top();
            fetch_series_list(&query_clone).await
        })
    };
    {
        let series_list_clone = series_list.clone();
        use_effect_with_deps(
            move |_query_clone| {
                series_list_clone.run();
                || ()
            },
            query.clone(),
        );
    }

    let on_filter_change = {
        let query_clone = query.clone();
        Callback::from(move |order: GeneralOrder| {
            let new_query = GeneralQuery {
                order,
                ..query_clone
            };
            let ret = navigator.push_with_query(&Route::Series, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let pagination_link = {
        let query_clone = query.clone();
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = GeneralQuery {
                    page: page_id,
                    ..query_clone
                };
                html! {
                    <Link<Route, GeneralQuery> to={ Route::Series }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, GeneralQuery>>
                }
            },
        )
    };

    series_list.data.as_ref().map_or_else(
        || html! {},
        |series_list| {
            html! {
                <>
                <h2>{ "Series" }</h2>
                <GeneralFilterComponent onchange={ on_filter_change } current_order={ query.order } />

                <div class="container-fluid">
                    <div class="row">
                        {if user_ctx.is_admin() {
                            html! {
                                <EditSeriesContainerComponent series_list={ series_list.list.clone() } />
                            }
                        } else {
                            generate_series_list(&series_list.list)
                        }}
                    </div>
                </div>

                <PaginationComponent  current_page={ series_list.page.page_num }
                    total_pages={ series_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
