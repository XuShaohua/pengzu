// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_history, History, Link, Location};

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::series::fetch_series_list;
use crate::views::util;
use shared::general_query::{GeneralOrder, GeneralQuery};

#[function_component(SeriesComponent)]
pub fn series_page() -> Html {
    util::set_document_title("Series");

    let history = use_history().unwrap();
    let location = history.location();
    let query = location.query::<GeneralQuery>().unwrap_or_default();
    let series_list = {
        let query_clone = query.clone();
        use_async(async move { fetch_series_list(&query_clone).await })
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
            let ret = history.push_with_query(Route::Series, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    series_list.data.as_ref().map_or_else(
        || html! {},
        |series_list| {
            html! {
                <>
                <h2>{ "Series" }</h2>
                <GeneralFilterComponent onchange={ filter_onchange } current_order={ query.order } />

                <ul>
                {for series_list.list.iter().map(|series| html!{
                    <li class="series-item" key={ series.id }>
                    <span class="badge">{ series.count }</span>
                    <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>
                    { &series.name }
                    </Link<Route>>
                    </li>
                })}
                </ul>
                <PaginationComponent  current_page={ series_list.page.page_num }
                    total_pages={ series_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
