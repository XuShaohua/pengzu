// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::hooks::use_user_context;
use crate::services::users::login;
use crate::types::users::LoginForm;

#[function_component(LoginComponent)]
pub fn login_page() -> Html {
    let user_ctx = use_user_context();
    let login_form = use_state(LoginForm::default);
    let user_login = {
        let login_form = login_form.clone();
        use_async(async move { login(&login_form).await })
    };

    use_effect_with_deps(
        move |user_login| {
            if let Some(user_info) = &user_login.data {
                user_ctx.login(user_info.clone())
            }
            || ()
        },
        user_login.clone(),
    );

    let onsubmit = {
        let user_login = user_login.clone();
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_login.run();
        })
    };

    let oninput_username = {
        let login_form = login_form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form: LoginForm = (*login_form).clone();
            form.username = input.value();
            login_form.set(form);
        })
    };

    let oninput_password = {
        let login_form = login_form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form: LoginForm = (*login_form).clone();
            form.password = input.value();
            login_form.set(form);
        })
    };

    html! {
        <div class="login-form">
            <form {onsubmit}>
                <div class="form-item">
                    <label for="username">{ "Username" }</label>
                    <input name="username" type="text"
                        oninput={ oninput_username }
                        value={ login_form.username.clone() } />
                </div>
                <div class="form-item">
                    <label for="password">{ "Password" }</label>
                    <input name="password" type="password"
                        oninput={ oninput_password }
                        value={ login_form.password.clone() } />
                </div>
                <button>{ "Login" }</button>
            </form>
        </div>
    }
}
