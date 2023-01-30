// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub book: i32,
}

#[function_component(SearchTagComponent)]
pub fn search_tag(props: &Props) -> Html {
    log::info!("book id: {}", props.book);

    let oninput_cb = {
        Callback::from(move |event: InputEvent| {
            let input: HtmlInputElement = event.target_unchecked_into();
            let text = input.value();
            log::info!("value: {text}");
        })
    };

    html! {
        <div class="d-inline">
            <input type="text" oninput={ oninput_cb } />
        </div>
    }
}
