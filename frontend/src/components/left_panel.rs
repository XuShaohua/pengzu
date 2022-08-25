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
                <li><Link<Route> to={ Route::Home }>{ "Books" }</Link<Route>></li>
                <li><Link<Route> to={ Route::Author }>{ "Authors" }</Link<Route>></li>
                <li><Link<Route> to={ Route::Category }>{ "Categories" }</Link<Route>></li>
                <li><Link<Route> to={ Route::Tag }>{ "Tags" }</Link<Route>></li>
                <li><Link<Route> to={ Route::Publisher }>{ "Publishers" }</Link<Route>></li>
                <li><Link<Route> to={ Route::Series }>{ "Series" }</Link<Route>></li>
                <li><Link<Route> to={ Route::Discover }>{ "Discover" }</Link<Route>></li>
                <li><Link<Route> to={ Route::FileFormat }>{ "File Formats" }</Link<Route>></li>
                <li><Link<Route> to={ Route::Rating }>{ "Ratings" }</Link<Route>></li>
            </ul>
            <br />
            <ul>
                <li><Link<Route> to={ Route::UserTag }>{ "My Tags" }</Link<Route>></li>
            </ul>
        </div>
    }
}
