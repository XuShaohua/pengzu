// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::general_query::{GeneralOrder, GeneralQuery};
use shared::page::PageId;
use shared::series::SeriesAndBook;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, Link};

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::series::fetch_series_list;
use crate::views::util;

fn generate_series_list(series_list: &[SeriesAndBook]) -> Html {
    html! {
        <ul class="col-xs-12 col-sm-6 list-unstyled">
        {for series_list.iter().map(|series| html!{
            <li class="mb-3" key={ series.id }>
                <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ series.count }</span>
                <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>
                    { &series.name }
                </Link<Route>>
            </li>
        })}
        </ul>
    }
}

#[function_component(SeriesComponent)]
pub fn series_page() -> Html {
    util::set_document_title("Series");

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

    let filter_onchange = {
        Callback::from(|order: GeneralOrder| {
            log::info!("new order: {:?}", order);
            // TODO(Shaohua):
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
            let half_list = series_list.list.len() / 2;
            html! {
                <>
                <h2>{ "Series" }</h2>
                <GeneralFilterComponent onchange={ filter_onchange } current_order={ query.order } />

                <div class="container-fluid">
                    <div class="row">
                        { generate_series_list(&series_list.list[..half_list]) }
                        { generate_series_list(&series_list.list[half_list..]) }
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
