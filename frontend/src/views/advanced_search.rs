// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

#[function_component(AdvancedSearchComponent)]
pub fn advanced_search() -> Html {
    html! {
        <form class="advanced-search"
            method="GET">
        <div class="form-group">
            <label for="book_title">{ "Book Title" }</label>
            <input id="book_title" class="form-control" name="book_title" type="text" />
        </div>

        <div class="form-group">
            <label for="book_author">{ "Author" }</label>
            <input id="book_author" class="form-control" name="book_author" type="text" />
        </div>

        <div class="form-group">
            <label for="book_publisher">{ "Publisher" }</label>
            <input id="book_publisher" class="form-control" name="book_publishder" type="text" />
        </div>

        <div class="form-group">
            <button type="submit" class="btn btn-default">{ "Search" }</button>
        </div>

        </form>
    }
}
