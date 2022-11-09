// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::simple_search::SimpleSearchQuery;
use std::ops::Deref;
use web_sys::{FocusEvent, HtmlInputElement};
use yew::prelude::*;
use yew_router::history::History;
use yew_router::hooks::use_history;

use crate::router::Route;

#[function_component(HeaderSearchComponent)]
pub fn header_search() -> Html {
    let input_ref = use_node_ref();
    let history = use_history().expect("History object is invalid");
    let query_state = use_state(String::new);

    use_effect_with_deps(
        move |query_state| {
            let query = query_state.deref().clone();
            if !query.is_empty() {
                let query_obj = SimpleSearchQuery {
                    query,
                    ..Default::default()
                };
                let ret = history.push_with_query(Route::BooksOfSimpleSearch, &query_obj);
                debug_assert!(ret.is_ok());
            }
            || ()
        },
        query_state.clone(),
    );

    let search_onsubmit = {
        let input_ref_clone = input_ref.clone();
        let query_state_clone = query_state.clone();
        Callback::from(move |event: FocusEvent| {
            event.prevent_default();
            if let Some(input) = input_ref_clone.cast::<HtmlInputElement>() {
                query_state_clone.set(input.value());
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
                query_state.set(input.value());
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
