// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[function_component(LeftPanelComponent)]
pub fn left_panel() -> Html {
    let style_str = include_str!("left_panel.css");
    let style_cls = Style::new(style_str).expect("Invalid style file left_panel.css");

    html! {
        <div class={ style_cls }>
            <ul>
                <li><Link<Route> to={ Route::Home }>
                    <i class="bi bi-book"></i>
                    { "Books" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Author }>
                    <i class="bi bi-people"></i>
                    { "Authors" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Category }>
                    <i class="bi bi-inbox"></i>
                    { "Categories" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Tag }>
                    <i class="bi bi-tags"></i>
                    { "Tags" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Publisher }>
                    <i class="bi bi-file-font"></i>
                    { "Publishers" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Series }>
                    <i class="bi bi-grid-3x3-gap"></i>
                    { "Series" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::BooksOfDiscover }>
                    <i class="bi bi-shuffle"></i>
                    { "Discover" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::FileFormat }>
                    <i class="bi bi-file-earmark"></i>
                    { "File Formats" }
                </Link<Route>></li>
            </ul>
            <br />
            <ul>
                <li><Link<Route> to={ Route::UserTag }>
                    <i class="bi bi-tags"></i>
                    { "My Tags" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::BooksOfDownloadHistory }>
                    <i class="bi bi-download"></i>
                    { "My Downloads" }
                </Link<Route>></li>
            </ul>
        </div>
    }
}
