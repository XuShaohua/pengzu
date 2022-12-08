// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[function_component(LeftPanelComponent)]
pub fn left_panel() -> Html {
    html! {
        <div class="d-flex flex-column flex-shrink-0" style="width: 180px">
            <ul class="list-unstyled">
                <li class="p-1"><Link<Route> to={ Route::Home }>
                    <i class="bi bi-book pe-1"></i>
                    { "Books" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::Author }>
                    <i class="bi bi-people pe-1"></i>
                    { "Authors" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::Category }>
                    <i class="bi bi-inbox pe-1"></i>
                    { "Categories" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::Tag }>
                    <i class="bi bi-tags pe-1"></i>
                    { "Tags" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::Publisher }>
                    <i class="bi bi-file-font pe-1"></i>
                    { "Publishers" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::Series }>
                    <i class="bi bi-grid-3x3-gap pe-1"></i>
                    { "Series" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::BooksOfDiscover }>
                    <i class="bi bi-shuffle pe-1"></i>
                    { "Discover" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::FileFormat }>
                    <i class="bi bi-file-earmark pe-1"></i>
                    { "File Formats" }
                </Link<Route>></li>
            </ul>
            <hr />
            <ul class="list-unstyled">
                <li class="p-1"><Link<Route> to={ Route::UserTag }>
                    <i class="bi bi-tags pe-1"></i>
                    { "My Tags" }
                </Link<Route>></li>
                <li class="p-1"><Link<Route> to={ Route::BooksOfDownloadHistory }>
                    <i class="bi bi-download pe-1"></i>
                    { "My Downloads" }
                </Link<Route>></li>
            </ul>
        </div>
    }
}
