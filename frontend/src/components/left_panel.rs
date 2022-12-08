// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[function_component(LeftPanelComponent)]
pub fn left_panel() -> Html {
    html! {
        <div class="col-sm-2">
            <ul class="nav nav-pills flex-column mb-auto">
                <li class="nav-item">
                    <Link<Route> to={ Route::Home } classes="nav-link active">
                        <i class="bi bi-book pe-2"></i>
                        { "Books" }
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={ Route::Author } classes="nav-link">
                        <i class="bi bi-people pe-2"></i>
                        { "Authors" }
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={ Route::Category } classes="nav-link">
                        <i class="bi bi-inbox pe-2"></i>
                        { "Categories" }
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={ Route::Tag } classes="nav-link">
                        <i class="bi bi-tags pe-2"></i>
                        { "Tags" }
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={ Route::Publisher } classes="nav-link">
                        <i class="bi bi-file-font pe-2"></i>
                        { "Publishers" }
                    </Link<Route>></li>
                <li class="nav-item">
                    <Link<Route> to={ Route::Series } classes="nav-link">
                        <i class="bi bi-grid-3x3-gap pe-2"></i>
                        { "Series" }
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={ Route::BooksOfDiscover } classes="nav-link">
                        <i class="bi bi-shuffle pe-2"></i>
                        { "Discover" }
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={ Route::FileFormat } classes="nav-link">
                        <i class="bi bi-file-earmark pe-2"></i>
                        { "File Formats" }
                    </Link<Route>>
                </li>

                <li class="nav-item">
                    <hr class="ms-3 me-4" />
                </li>

                <li class="nav-item">
                    <Link<Route> to={ Route::UserTag } classes="nav-link">
                        <i class="bi bi-tags pe-2"></i>
                        { "My Tags" }
                    </Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> to={ Route::BooksOfDownloadHistory } classes="nav-link">
                        <i class="bi bi-download pe-2"></i>
                        { "My Downloads" }
                    </Link<Route>>
                </li>
            </ul>
        </div>
    }
}
