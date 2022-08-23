// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use web_sys::Url;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    footer::FooterComponent, header::HeaderComponent, left_panel::LeftPanelComponent,
    login::LoginComponent,
};
use crate::route::{switch_route, Route};

fn is_login_page() -> bool {
    let window = gloo_utils::window();
    let location = window.location();
    let href = match location.href() {
        Ok(href) => href,
        Err(_err) => return false,
    };
    let url = match Url::new(&href) {
        Ok(url) => url,
        Err(_err) => return false,
    };
    let pathname = url.pathname();
    pathname == "/login"
}

#[function_component(AppComponent)]
pub fn app() -> Html {
    // TODO(Shaohua): Remove
    if is_login_page() {
        return html! {
            <LoginComponent />
        };
    }

    html! {
        <>
            <BrowserRouter>
                <HeaderComponent />
                <div class="container-fluid">
                    <div class="row-fluid">
                        <LeftPanelComponent />
                        <div class="content-area">
                            <Switch<Route> render={Switch::render(switch_route)} />
                        </div>
                    </div>
                </div>
                <FooterComponent />
            </BrowserRouter>
        </>
    }
}
