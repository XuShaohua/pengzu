// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::series::{NewSeries, SeriesAndBook};
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::Link;

use crate::services::series::{add_series, delete_series, update_series};
use crate::views::series::add_series_modal::AddSeriesModal;
use crate::views::series::delete_series_modal::DeleteSeriesModal;
use crate::views::series::edit_series_modal::EditSeriesModal;
use crate::views::series::Route;

const ADD_SERIES_MODAL: &str = "add-series-modal";
const ADD_SERIES_MODAL_ID: &str = "#add-series-modal";
const EDIT_SERIES_MODAL: &str = "edit-series-modal";
const EDIT_SERIES_MODAL_ID: &str = "#edit-series-modal";
const DELETE_SERIES_MODAL: &str = "delete-series-modal";
const DELETE_SERIES_MODAL_ID: &str = "#delete-series-modal";

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ContainerProps {
    pub series_list: Vec<SeriesAndBook>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct EditSeriesReq {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct DeleteSeriesReq {
    pub id: i32,
    pub name: String,
}

#[function_component(EditSeriesContainerComponent)]
pub fn edit_series_container(props: &ContainerProps) -> Html {
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
        let series_id_clone = series_id.clone();
        let new_series_clone = new_series.clone();
        Callback::from(move |req: EditSeriesReq| {
            series_id_clone.set(req.id);
            new_series_clone.set(NewSeries { name: req.name });
        })
    };

    let delete_series_task = {
        let series_id_clone = series_id.clone();
        use_async(async move { delete_series(*series_id_clone).await })
    };
    let delete_series_cb = {
        Callback::from(move |will_delete: bool| {
            if will_delete {
                delete_series_task.run();
            }
        })
    };
    let delete_series_req = {
        let new_series_clone = new_series.clone();
        Callback::from(move |req: DeleteSeriesReq| {
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

    let half_list = (props.series_list.len() + 1) / 2;

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

        <div class="modal fade" tabindex="-1" id={ DELETE_SERIES_MODAL }>
            <DeleteSeriesModal name={ new_series.name.clone() } ok_cb={ delete_series_cb } />
        </div>

        <EditSeriesItemListComponent
            edit_series_req={ edit_series_req.clone() }
            delete_series_req={ delete_series_req.clone() }
            series_list={ props.series_list[..half_list].to_vec() } />

        <EditSeriesItemListComponent
            edit_series_req={ edit_series_req }
            delete_series_req={ delete_series_req }
            series_list={ props.series_list[half_list..].to_vec() } />
        </>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemListProps {
    pub series_list: Vec<SeriesAndBook>,
    pub edit_series_req: Callback<EditSeriesReq>,
    pub delete_series_req: Callback<DeleteSeriesReq>,
}

#[function_component(EditSeriesItemListComponent)]
pub fn edit_series_item_list(props: &ItemListProps) -> Html {
    let series_list = &props.series_list;
    html! {
        <ul class="col-xs-12 col-sm-6 list-unstyled">
        {for series_list.iter().map(|series| html!{
            <li class="mb-2" key={ series.id }>
                <EditSeriesItemComponent
                    edit_series_req={ props.edit_series_req.clone() }
                    delete_series_req={ props.delete_series_req.clone() }
                    series={ series.clone() } />
            </li>
        })}
        </ul>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ItemProps {
    pub series: SeriesAndBook,
    pub edit_series_req: Callback<EditSeriesReq>,
    pub delete_series_req: Callback<DeleteSeriesReq>,
}

#[function_component(EditSeriesItemComponent)]
pub fn edit_series_item(props: &ItemProps) -> Html {
    let series = &props.series;

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
        let series_clone = series.clone();
        let delete_series_req_clone = props.delete_series_req.clone();
        Callback::from(move |_event: MouseEvent| {
            delete_series_req_clone.emit(DeleteSeriesReq {
                id: series_clone.id,
                name: series_clone.name.clone(),
            });
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
                data-bs-toggle="modal" data-bs-target={ DELETE_SERIES_MODAL_ID }
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
