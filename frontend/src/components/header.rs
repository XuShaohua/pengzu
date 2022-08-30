// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::hooks::use_user_context;
use crate::route::Route;

#[function_component(HeaderComponent)]
pub fn header() -> Html {
    let user_ctx = use_user_context();

    html! {
        <div class="navbar">
            { "Online Library" }
            <div class="user-container">
            if user_ctx.is_login() {
                <span>{ &user_ctx.name }</span>
            } else {
                <Link<Route> to={ Route::Login }>{ "Login" }</Link<Route>>
            }
            </div>
        </div>
    }
}
