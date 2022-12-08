// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::file_formats::fetch_file_formats;
use crate::views::util;

#[function_component(FileFormatsComponent)]
pub fn file_formats_page() -> Html {
    util::set_document_title("File Formats");

    let file_formats = use_async_with_options(
        async move { fetch_file_formats().await },
        UseAsyncOptions::enable_auto(),
    );

    file_formats.data.as_ref().map_or_else(
        || html! {},
        |file_formats| {
            html! {
                <>
                <h2>{ "File Formats" }</h2>

                <ul class="list-unstyled">
                    {for file_formats.list.iter().map(|file_format| html!{
                        <li class="mb-3" key={ file_format.id }>
                            <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ file_format.count }</span>
                            <Link<Route> to={ Route::BooksOfFileFormat{ format_id: { file_format.id }}}>
                                { &file_format.name }
                            </Link<Route>>
                        </li>
                    })}
                </ul>
                </>
            }
        },
    )
}
