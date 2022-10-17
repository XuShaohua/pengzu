// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;

#[function_component(LeftPanelComponent)]
pub fn left_panel() -> Html {
    html! {
        <div class="left-panel">
            <ul>
                <li><Link<Route> to={ Route::Home }>
                    <span class="glyphicon glyphicon-book" />
                    { "Books" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Author }>
                    <span class="glyphicon glyphicon-user" />
                    { "Authors" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Category }>
                    <span class="glyphicon glyphicon-inbox" />
                    { "Categories" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Tag }>
                    <span class="glyphicon glyphicon-tags" />
                    { "Tags" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Publisher }>
                    <span class="glyphicon glyphicon-text-size" />
                    { "Publishers" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Series }>
                    <span class="glyphicon glyphicon-th" />
                    { "Series" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Discover }>
                    <span class="glyphicon glyphicon-random" />
                    { "Discover" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::FileFormat }>
                    <span class="glyphicon glyphicon-file" />
                    { "File Formats" }
                </Link<Route>></li>
                <li><Link<Route> to={ Route::Rating }>
                    <span class="glyphicon glyphicon-star-empty" />
                    { "Ratings" }
                </Link<Route>></li>
            </ul>
            <br />
            <ul>
                <li><Link<Route> to={ Route::UserTag }>
                    <span class="glyphicon glyphicon-tag" />
                    { "My Tags" }
                </Link<Route>></li>
            </ul>
        </div>
    }
}
