// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::file_formats::fetch_file_formats;

#[function_component(FileFormatsComponent)]
pub fn file_formats_page() -> Html {
    let file_formats = use_async_with_options(
        async move { fetch_file_formats().await },
        UseAsyncOptions::enable_auto(),
    );

    if let Some(file_formats) = &file_formats.data {
        return html! {
            <ul>
                {for file_formats.list.iter().map(|file_format| html!{
                    <li key={ file_format.id }>
                    <span class="badge">{ file_format.count }</span>
                    { &file_format.name }
                    </li>
                })}
            </ul>
        };
    } else {
        return html! {};
    }
}
