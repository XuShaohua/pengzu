// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::series::delete_series;
use shared::series::SeriesAndBook;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::views::series::Route;

pub fn generate_edit_series_list(series_list: &[SeriesAndBook]) -> Html {
    html! {
        <ul class="col-xs-12 col-sm-6 list-unstyled">
        {for series_list.iter().map(|series| html!{
            <li class="mb-2" key={ series.id }>
                <EditSeriesItemComponent series={ series.clone() } />
            </li>
        })}
        </ul>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub series: SeriesAndBook,
}

#[function_component(EditSeriesItemComponent)]
pub fn edit_series_item(props: &Props) -> Html {
    let series = &props.series;
    let is_deleted = use_state(|| false);

    let delete_series_task = {
        let series_id = series.id;
        use_async(async move {
            if *is_deleted {
                delete_series(series_id).await
            } else {
                is_deleted.set(true);
                Ok(())
            }
        })
    };
    let on_delete_button_click = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            delete_series_task.run();
        })
    };
    let on_edit_button_click = {
        let series_id = series.id;
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            log::info!("edit series: {}", series_id);
        })
    };

    html! {
        <>
        <button class="btn btn-secondary btn-sm me-2" onclick={ on_edit_button_click }>{ "Edit" }</button>
        <button class="btn btn-danger btn-sm me-2" onclick={ on_delete_button_click }>{ "Delete" }</button>
        <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ series.count }</span>
        <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>
            { &series.name }
        </Link<Route>>
        </>
    }
}
