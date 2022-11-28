// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::advanced_search::AdvancedSearchQuery;
use stylist::Style;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::router::Route;
use crate::views::util;

#[function_component(AdvancedSearchComponent)]
pub fn advanced_search() -> Html {
    util::set_document_title("Advanced Search");

    let style_str = include_str!("advanced_search.css");
    let style_cls = Style::new(style_str).expect("Invalid style file advanced_search.css");

    let navigator = use_navigator().unwrap();
    let title_input_ref = use_node_ref();
    let author_input_ref = use_node_ref();
    let publisher_input_ref = use_node_ref();

    let search_onsubmit = {
        let title_input_ref_clone = title_input_ref.clone();
        let author_input_ref_clone = author_input_ref.clone();
        let publisher_input_ref_clone = publisher_input_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let mut query = AdvancedSearchQuery::default();
            if let Some(input) = title_input_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    query.title = Some(value);
                }
            }
            if let Some(input) = author_input_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    query.author = Some(value);
                }
            }
            if let Some(input) = publisher_input_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    query.publisher = Some(value);
                }
            }

            let ret = navigator.push_with_query(&Route::BooksOfAdvancedSearch, &query);
            debug_assert!(ret.is_ok());
        })
    };

    html! {
        <form class={ style_cls } onsubmit={ search_onsubmit }>
        <div class="form-group">
            <label for="book_title">{ "Book Title" }</label>
            <input id="book_title"
                class="form-control"
                ref={ title_input_ref }
                name="book_title"
                type="text" />
        </div>

        <div class="form-group">
            <label for="book_author">{ "Author" }</label>
            <input id="book_author"
                class="form-control"
                ref={ author_input_ref }
                name="book_author"
                type="text" />
        </div>

        <div class="form-group">
            <label for="book_publisher">{ "Publisher" }</label>
            <input id="book_publisher"
                class="form-control"
                ref={ publisher_input_ref }
                name="book_publisher"
                type="text" />
        </div>

        <div class="form-group">
            <button type="submit" class="btn btn-default">{ "Search" }</button>
        </div>

        </form>
    }
}
