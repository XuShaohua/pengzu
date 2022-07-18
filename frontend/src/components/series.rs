// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::components::models::error::FetchError;
use crate::components::models::page::Page;
use crate::components::models::series::{fetch_series, GetSeriesResp, SeriesAndBook};

pub enum Msg {
    Fetch,
    FetchSuccess(GetSeriesResp),
    FetchFailed(FetchError),
}

pub struct SeriesComponent {
    series: Vec<SeriesAndBook>,
    page: Option<Page>,
}

fn generate_series_element(series: &SeriesAndBook) -> Html {
    html! {
        <li class="series-item" key={ series.id }>
            <span class="badge">{ series.count }</span>
            <a href={ format!("/series/books/{}", series.id) } target="_blank" title={ series.name.clone() }>
                { series.name.clone() }
            </a>
        </li>
    }
}

impl Component for SeriesComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            series: vec![],
            page: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    match fetch_series().await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(obj) => {
                log::info!("obj: {:#?}", obj);
                self.page = Some(obj.page);
                self.series.extend(obj.list);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch series: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        let series_elements = self
            .series
            .iter()
            .map(generate_series_element)
            .collect::<Html>();

        html! {
            <>
                <button onclick={fetch}>{"Fetch series"}</button>

                <ul class="series-list">
                    { series_elements }
                </ul>
            </>
        }
    }
}
