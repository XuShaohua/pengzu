// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::series::{NewSeries, SeriesAndBook};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::services::series::{add_series, delete_series, update_series};
use crate::views::series::add_series_modal::AddSeriesModal;
use crate::views::series::edit_series_modal::EditSeriesModal;
use crate::views::series::Route;

const ADD_SERIES_MODAL: &str = "add-series-modal";
const ADD_SERIES_MODAL_ID: &str = "#add-series-modal";
const EDIT_SERIES_MODAL: &str = "edit-series-modal";
const EDIT_SERIES_MODAL_ID: &str = "#edit-series-modal";

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ContainerProps {
    pub series_list: Vec<SeriesAndBook>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EditSeriesReq {
    pub id: i32,
    pub name: String,
}

#[function_component(EditSeriesContainerComponent)]
pub fn edit_series_container(props: &ContainerProps) -> Html {
    let series_list = &props.series_list;
    let new_series = use_state(NewSeries::default);
    let series_id = use_state(|| 0_i32);

    let add_series_task = {
        let new_series_clone = new_series.clone();
        use_async(async move { add_series(&new_series_clone).await })
    };
    let add_series_cb = {
        let new_series_clone = new_series.clone();
        Callback::from(move |new_series_name: String| {
            new_series_clone.set(NewSeries {
                name: new_series_name,
            });
            add_series_task.run();
        })
    };

    let edit_series_task = {
        let series_id_clone = series_id.clone();
        let new_series_clone = new_series.clone();
        use_async(async move { update_series(*series_id_clone, &new_series_clone).await })
    };
    let edit_series_cb = {
        let new_series_clone = new_series.clone();
        Callback::from(move |new_series_name: String| {
            new_series_clone.set(NewSeries {
                name: new_series_name,
            });
            edit_series_task.run();
        })
    };
    let edit_series_req = {
        let new_series_clone = new_series.clone();
        Callback::from(move |req: EditSeriesReq| {
            series_id.set(req.id);
            new_series_clone.set(NewSeries { name: req.name });
        })
    };

    let on_add_series_button_click = {
        let new_series_clone = new_series.clone();
        Callback::from(move |_event: MouseEvent| {
            new_series_clone.set(NewSeries::default());
        })
    };

    html! {
        <>
        <div class="mb-3">
            <button type="button" class="btn btn-primary btn-sm"
                data-bs-toggle="modal" data-bs-target={ ADD_SERIES_MODAL_ID }
                onclick={ on_add_series_button_click }
                title="Add series">
                { "Add Series" }
                <i class="bi bi-plus"></i>
            </button>
        </div>

        <div class="modal fade" tabindex="-1" id={ ADD_SERIES_MODAL }>
            <AddSeriesModal ok_cb={ add_series_cb } />
        </div>

        <div class="modal fade" tabindex="-1" id={ EDIT_SERIES_MODAL }>
            <EditSeriesModal name={ new_series.name.clone() } ok_cb={ edit_series_cb } />
        </div>

        <ul class="col-xs-12 col-sm-6 list-unstyled">
        {for series_list.iter().map(|series| html!{
            <li class="mb-2" key={ series.id }>
                <EditSeriesItemComponent
                    edit_series_req={ edit_series_req.clone() }
                    series={ series.clone() } />
            </li>
        })}
        </ul>
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemProps {
    pub series: SeriesAndBook,
    pub edit_series_req: Callback<EditSeriesReq>,
}

#[function_component(EditSeriesItemComponent)]
pub fn edit_series_item(props: &ItemProps) -> Html {
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

    let on_edit_button_click = {
        let series_clone = series.clone();
        let edit_series_req_clone = props.edit_series_req.clone();
        Callback::from(move |_event: MouseEvent| {
            edit_series_req_clone.emit(EditSeriesReq {
                id: series_clone.id,
                name: series_clone.name.clone(),
            });
        })
    };

    let on_delete_button_click = {
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            delete_series_task.run();
        })
    };

    html! {
        <>
        <div class="btn-group me-2" role="group">
            <button type="button" class="btn btn-warning btn-sm" title="Edit series"
                data-bs-toggle="modal" data-bs-target={ EDIT_SERIES_MODAL_ID }
                onclick={ on_edit_button_click }>
                <i class="bi bi-pencil"></i>
            </button>
            <button type="button" class="btn btn-danger btn-sm" title="Delete series"
                onclick={ on_delete_button_click }>
                <i class="bi bi-trash3"></i>
            </button>
        </div>

        <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ series.count }</span>
        <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>
            { &series.name }
        </Link<Route>>
        </>
    }
}
