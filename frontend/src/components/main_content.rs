// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by General Public License that can be found
// in the LICENSE file.

use yew::prelude::*;
use yew_router::Switch;

use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;
use crate::components::left_panel::LeftPanelComponent;
use crate::hooks::use_user_context;
use crate::router::{switch_route, Route};
use crate::views::login::LoginComponent;

#[function_component(MainContentComponent)]
pub fn main_content() -> Html {
    let user_ctx = use_user_context();

    html! {

        if user_ctx.is_login() {
            <>
            <HeaderComponent />

            <div class="container-fluid">
                <LeftPanelComponent />
                <div class="content-area">
                    <Switch<Route> render={ switch_route } />
                </div>
            </div>

            <FooterComponent/>
            </>
        } else {
            <LoginComponent />
        }
    }
}
