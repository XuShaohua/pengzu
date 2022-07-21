// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::models::error::FetchError;
use crate::components::models::users::{login, LoginForm, UserInfo};

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(UserInfo),
    FetchFailed(FetchError),
}

pub struct LoginComponent {
    user_info: Option<UserInfo>,
    username_node: NodeRef,
    password_node: NodeRef,
}

impl Component for LoginComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            user_info: None,
            username_node: Default::default(),
            password_node: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let username_node = self.username_node.cast::<HtmlInputElement>().unwrap();
                let password_node = self.password_node.cast::<HtmlInputElement>().unwrap();
                let username = username_node.value();
                let password = password_node.value();
                let form = LoginForm { username, password };
                ctx.link().send_future(async move {
                    match login(&form).await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(user_info) => {
                log::info!("user info: {:?}", user_info);
                self.user_info = Some(user_info);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch something: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let login = ctx.link().callback(|event: FocusEvent| {
            event.prevent_default();
            Msg::Fetch
        });

        html! {
            <div class="login-form">
                <form onsubmit={ login } method="POST" role="form">
                    <div class="form-item">
                        <label for="username">{ "Username" }</label>
                        <input name="username" type="text" ref={ self.username_node.clone() } />
                    </div>
                    <div class="form-item">
                        <label for="password">{ "Password" }</label>
                        <input name="password" type="password" ref={ self.password_node.clone() } />
                    </div>
                    <button>{ "Login" }</button>
                </form>
            </div>
        }
    }
}
