// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::file_formats::{fetch_file_formats, FileFormatAndBook, GetFileFormatsResp};
use crate::types::error::FetchError;
use crate::types::page::Page;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(GetFileFormatsResp),
    FetchFailed(FetchError),
}

pub struct FileFormatsComponent {
    formats: Vec<FileFormatAndBook>,
    page: Option<Page>,
}

impl Component for FileFormatsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            formats: vec![],
            page: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                ctx.link().send_future(async {
                    match fetch_file_formats().await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(obj) => {
                log::info!("obj: {:#?}", obj);
                self.page = Some(obj.page);
                self.formats.extend(obj.list);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch formats: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        html! {
            <div>
                <button onclick={fetch}>{"Fetch file formats"}</button>
            </div>
        }
    }
}
