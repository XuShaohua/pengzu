// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::components::models::error::FetchError;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess,
    FetchFailed(FetchError),
}

pub struct LoginComponent {}

impl Component for LoginComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => false,
            Msg::FetchSuccess => true,
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch something: {:?}", err);
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="login-form">
                <form>
                    <div class="form-item">
                    <label for="username">{ "Username" }</label>
                    <input name="username" type="text" />
                    </div>
                    <div class="form-item">
                    <label for="password">{ "Password" }</label>
                    <input name="password" type="password" />
                    </div>
                    <button>{ "Login" }</button>
                </form>
            </div>
        }
    }
}
