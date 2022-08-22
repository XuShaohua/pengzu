// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::error::FetchError;
use crate::services::file_formats::fetch_file_formats;
use crate::types::file_formats::{FileFormatAndBook, FileFormatList};
use crate::types::page::Page;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(FileFormatList),
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
