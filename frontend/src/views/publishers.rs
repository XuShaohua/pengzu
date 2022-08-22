// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::error::FetchError;
use crate::route::Route;
use crate::services::publishers::fetch_publishers;
use crate::types::page::Page;
use crate::types::publishers::{GetPublishersResp, PublisherAndBook};

pub enum Msg {
    Fetch,
    FetchSuccess(GetPublishersResp),
    FetchFailed(FetchError),
}

pub struct PublishersComponent {
    publishers: Vec<PublisherAndBook>,
    page: Option<Page>,
}

fn generate_publisher_element(publisher: &PublisherAndBook) -> Html {
    html! {
        <li class="publisher-item" key={ publisher.id }>
            <span class="badge">{ publisher.count }</span>
            <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id } }>
                { &publisher.name }
            </Link<Route>>
        </li>
    }
}

impl Component for PublishersComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            publishers: vec![],
            page: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    match fetch_publishers().await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(obj) => {
                log::info!("obj: {:#?}", obj);
                self.page = Some(obj.page);
                self.publishers.extend(obj.list);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch publishers: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        let publisher_elements = self
            .publishers
            .iter()
            .map(generate_publisher_element)
            .collect::<Html>();

        html! {
            <>
                <button onclick={ fetch }>{"Fetch publishers"}</button>

                <ul class="publisher-list">
                    { publisher_elements }
                </ul>
            </>
        }
    }
}
