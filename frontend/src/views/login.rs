// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::users::LoginForm;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

use crate::hooks::use_user_context;
use crate::services::users::login;
use crate::views::util;

#[function_component(LoginComponent)]
pub fn login_page() -> Html {
    util::set_document_title("Login");

    let user_ctx = use_user_context();
    let login_form = use_state(LoginForm::default);
    let user_login = {
        let login_form = login_form.clone();
        use_async(async move { login(&login_form).await })
    };

    use_effect_with(user_login.clone(), move |user_login| {
        if let Some(user_info) = &user_login.data {
            user_ctx.login(user_info.clone());
        }
        || ()
    });

    let on_form_submit = {
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            user_login.run();
        })
    };

    let on_input_username = {
        let login_form = login_form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form: LoginForm = (*login_form).clone();
            form.username = input.value();
            login_form.set(form);
        })
    };

    let on_input_password = {
        let login_form = login_form.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut form: LoginForm = (*login_form).clone();
            form.password = input.value();
            login_form.set(form);
        })
    };

    html! {
        <div class="container">
        <div class="row justify-content-md-center">

        <h2 class="text-center mt-5 mt-3">{ "Login"}</h2>
        <form class="col-lg-6 col-md-9 col-sm-12" onsubmit={ on_form_submit }>
            <div class="mb-3">
                <label for="username" class="form-label">{ "Username" }</label>
                <input name="username" type="text" class="form-control"
                    oninput={ on_input_username }
                    value={ login_form.username.clone() } />
            </div>

            <div class="mb-3">
                <label for="password" class="form-label">{ "Password" }</label>
                <input name="password" type="password" class="form-control"
                    oninput={ on_input_password }
                    value={ login_form.password.clone() } />
            </div>
            <button type="submit" class="btn btn-primary">{ "Login" }</button>
        </form>

        </div>
        </div>
    }
}
