// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::series::SeriesAndBook;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::views::series::Route;

pub fn generate_series_list(series_list: &[SeriesAndBook]) -> Html {
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
