// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::users::{NewUserReq, UserRole};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};

use crate::services::users::{add_user, fetch_users};
use crate::views::util;

#[function_component(UsersComponent)]
pub fn users_page() -> Html {
    util::set_document_title("Users");

    let user_list = use_async_with_options(
        async move { fetch_users().await },
        UseAsyncOptions::enable_auto(),
    );

    let username_ref = use_node_ref();
    let email_ref = use_node_ref();
    let password_ref = use_node_ref();

    let new_user_form = use_state(NewUserReq::default);
    let new_user_request = {
        let new_user_form_clone = new_user_form.clone();
        use_async(async move {
            // TODO(Shaohua): Clear form
            // TODO(Shaohua): Reload users list
            add_user(&new_user_form_clone).await
        })
    };

    let form_onsubmit = {
        let username_ref_clone = username_ref.clone();
        let email_ref_clone = email_ref.clone();
        let password_ref_clone = password_ref.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let mut form: NewUserReq = (*new_user_form).clone();
            form.role = UserRole::User;
            if let Some(input) = username_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    form.name = value;
                    form.display_name = form.name.clone();
                }
            }

            if let Some(input) = email_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    form.email = value;
                }
            }

            if let Some(input) = password_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    form.password = value;
                }
            }

            new_user_form.set(form);
            new_user_request.run();
        })
    };

    user_list.data.as_ref().map_or_else(
        || html! {},
        |user_list| {
            html! {
                <>
                <h2>{ "Users" }</h2>
                <ul>
                {for user_list.iter().map(|user_info| html!{
                    <li key={ user_info.id }>
                    <span>{ &user_info.name }</span>
                    </li>
                })}
                </ul>

                <form onsubmit={ form_onsubmit }>
                    <div>
                    <label for="new-username" class="form-label">{ "Username" }</label>
                    <input id="new-username" class="form-control"
                        ref={ username_ref }
                        type="text" name="username" />
                    </div>

                    <div>
                    <label for="new-email" class="form-label">{ "Email" }</label>
                    <input id="new-email" class="form-control"
                        ref={ email_ref }
                        type="email" name="email" />
                    </div>

                    <div>
                    <label for="new-password" class="form-label">{ "Password" }</label>
                    <input id="new-password" class="form-control"
                        ref={ password_ref }
                        type="password" name="password" />
                    </div>

                    <button class="btn btn-primary" type="submit">{ "Add" }</button>
                </form>
                </>
            }
        },
    )
}
