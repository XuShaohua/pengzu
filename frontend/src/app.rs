// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#![allow(clippy::let_unit_value)]

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::FooterComponent;
use crate::components::header::HeaderComponent;
use crate::components::left_panel::LeftPanelComponent;
use crate::components::user_context_provider::UserContextProvider;
use crate::router::{switch_route, Route};

#[function_component(AppComponent)]
pub fn app() -> Html {
    let style_str = include_str!("app.css");
    let style_cls = Style::new(style_str).expect("Invalid style file app.css");

    html! {
        <UserContextProvider>
            <BrowserRouter>
                <HeaderComponent />
                <div class={ style_cls }>
                    <div class="row-fluid">
                        <LeftPanelComponent />
                        <div class="content-area">
                            <Switch<Route> render={Switch::render(switch_route)} />
                        </div>
                    </div>
                </div>
                <FooterComponent/>
            </BrowserRouter>
        </UserContextProvider>
    }
}
