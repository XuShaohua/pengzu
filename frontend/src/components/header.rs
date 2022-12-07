// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::let_unit_value)]

use yew::prelude::*;
use yew_router::prelude::Link;

use crate::components::header_search::HeaderSearchComponent;
use crate::hooks::use_user_context;
use crate::router::Route;

#[function_component(HeaderComponent)]
pub fn header() -> Html {
    let user_ctx = use_user_context();

    html! {
        <nav class="navbar navbar-expand-lg bg-light">
        <div class="container-fluid">
            <Link<Route> to={ Route::Home } classes="navbar-brand">
                <img src="/assets/images/pengzu.svg" alt="Logo" width="30" height="24"
                    class="d-inline-block align-text-top" />
                { "Pengzu Library" }
            </Link<Route>>
            <button class="navbar-toggler" type="button" data-bs-toggle="collapse"
                data-bs-target="#header-search-area"
                aria-controls="header-search-area"
                aria-expanded="false" aria-label="Toggle navigation">
                <span class="navbar-toggler-icon"></span>
            </button>

            <div class="collapse navbar-collapse" id="header-search-area">
                <div class="navbar-nav">
                    <HeaderSearchComponent />

                    <Link<Route> to={ Route::AdvancedSearch }
                        classes="nav-link">
                        <span>{ "Advanced Search" }</span>
                    </Link<Route>>
                </div>
            </div>

            <ul class="navbar-nav">
            if user_ctx.is_login() {
                <li class="nav-item">
                    <Link<Route> to={ Route::UserInfo } classes="nav-link">
                        { &user_ctx.name }
                    </Link<Route>></li>
                <li class="nav-item">
                    <Link<Route> to={ Route::Logout } classes="nav-link">
                        { "Logout" }
                    </Link<Route>>
                </li>
            }
            </ul>
        </div>
        </nav>
    }
}
