// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::series::Series;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
    pub series: Option<Series>,
}

#[function_component(EditSeriesComponent)]
pub fn edit_series(props: &Props) -> Html {
    let series = &props.series;
    series.as_ref().map_or_else(
        || html! {<></>},
        |series| {
            html! {
                <Link<Route> to={ Route::BooksOfSeries { series_id: series.id }}
                    classes="col-sm-10">
                    { &series.name }
                </Link<Route>>
            }
        },
    )
}
