// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::files::get_file_format_url;
use crate::views::util::to_readable_size;
use shared::files::FileWithPath;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub files: Vec<FileWithPath>,
}

#[function_component(BookFormatsComponent)]
pub fn book_formats(props: &Props) -> Html {
    let files = &props.files;
    let elements = files
        .iter()
        .map(|file| {
            let url = get_file_format_url(file);
            let readable_size = to_readable_size(file.size);
            html! {
                <li>
                    <a class="book-format" target="_blank" href={ url }>
                        { format!("{} ({readable_size})", file.format_name) }
                        <i class="bi bi-download ms-1"></i>
                    </a>
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <ol class="book-formats ms-3">
            { elements }
        </ol>
    }
}
