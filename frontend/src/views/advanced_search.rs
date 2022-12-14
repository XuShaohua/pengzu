// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::advanced_search::AdvancedSearchQuery;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;

use crate::router::Route;
use crate::views::util;

#[function_component(AdvancedSearchComponent)]
pub fn advanced_search() -> Html {
    util::set_document_title("Advanced Search");

    let navigator = use_navigator().unwrap();
    let title_input_ref = use_node_ref();
    let author_input_ref = use_node_ref();
    let publisher_input_ref = use_node_ref();
    let isbn_input_ref = use_node_ref();

    let on_search_submit = {
        let title_input_ref_clone = title_input_ref.clone();
        let author_input_ref_clone = author_input_ref.clone();
        let publisher_input_ref_clone = publisher_input_ref.clone();
        let isbn_input_ref_clone = isbn_input_ref.clone();

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
            if let Some(input) = isbn_input_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    query.isbn = Some(value);
                }
            }

            let ret = navigator.push_with_query(&Route::BooksOfAdvancedSearch, &query);
            debug_assert!(ret.is_ok());
        })
    };

    html! {
        <>
        <h2>{ "Advanced Search" }</h2>

        <div class="container-fluid">
        <form onsubmit={ on_search_submit } class="col-lg-6 col-md-9 col-sm-12">
        <div class="mb-3">
            <label for="book-title" class="form-label">{ "Book Title" }</label>
            <input id="book-title"
                class="form-control"
                ref={ title_input_ref }
                name="book_title"
                type="text" />
        </div>

        <div class="mb-3">
            <label for="book-author" class="form-label">{ "Author" }</label>
            <input id="book-author"
                class="form-control"
                ref={ author_input_ref }
                name="book_author"
                type="text" />
        </div>

        <div class="mb-3">
            <label for="book-publisher" class="form-label">{ "Publisher" }</label>
            <input id="book-publisher"
                class="form-control"
                ref={ publisher_input_ref }
                name="book_publisher"
                type="text" />
        </div>

        <div class="mb-3">
            <label for="book-isbn" class="form-label">{ "ISBN" }</label>
            <input id="book-isbn"
                class="form-control"
                ref={ isbn_input_ref }
                name="book_isbn"
                type="text" />
        </div>

        <button type="submit" class="btn btn-primary">{ "Search" }</button>
        </form>
        </div>

        </>
    }
}
