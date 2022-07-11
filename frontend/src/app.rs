// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    footer::FooterComponent, header::HeaderComponent, left_panel::LeftPanelComponent,
};
use crate::route::{switch_route, Route};

pub enum Msg {}

pub struct AppComponent {}

impl Component for AppComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <BrowserRouter>
                    <HeaderComponent />
                    <div>
                        <LeftPanelComponent />
                        <Switch<Route> render={Switch::render(switch_route)} />
                    </div>
                    <FooterComponent />
                </BrowserRouter>
            </>
        }
    }
}
