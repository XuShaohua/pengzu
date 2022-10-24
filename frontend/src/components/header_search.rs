// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use web_sys::{FocusEvent, HtmlInputElement};
use yew::prelude::*;

#[function_component(HeaderSearchComponent)]
pub fn header_search() -> Html {
    let input_ref = use_node_ref();

    let search_onsubmit = {
        let input_ref_clone = input_ref.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            if let Some(input) = input_ref_clone.cast::<HtmlInputElement>() {
                let query = input.value();
                log::info!("search query: {}", query);
            }
        })
    };

    let input_onkeydown = {
        let input_ref_clone = input_ref.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.code() != "Enter" {
                return;
            }
            event.prevent_default();

            if let Some(input) = input_ref_clone.cast::<HtmlInputElement>() {
                let query = input.value();
                log::info!("search query: {}", query);
            }
        })
    };

    html! {
       <form class="navbar-search input-group" onsubmit={ search_onsubmit }>
            <input class="form-control"
                ref={input_ref}
                onkeydown={ input_onkeydown }
                type="text"
                placeholder={ "Search Library" } />
            <button class="btn btn-default" type="submit">{ "Search" }</button>
        </form>
    }
}
