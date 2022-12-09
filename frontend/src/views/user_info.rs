// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};

use crate::services::users::get_user_info;
use crate::views::util;

#[function_component(UserInfoComponent)]
pub fn user_info() -> Html {
    util::set_document_title("User Info");

    let user_info = use_async_with_options(
        async move { get_user_info().await },
        UseAsyncOptions::enable_auto(),
    );

    user_info.data.as_ref().map_or_else(
        || {
            html! {
                 <h2>{ "User Info" }</h2>
            }
        },
        |user_info| {
            html! {
                <>
                <h2>{ "User Info" }</h2>
                <div>
                    <span class="fw-bold text-end me-2">{ "Username:" }</span>
                    <span>{ &user_info.name }</span>
                </div>
                <div>
                    <span class="fw-bold text-end me-2">{ "Email:" }</span>
                    <span>{ &user_info.email }</span>
                </div>
                <div>
                    <span class="fw-bold text-end me-2">{ "Role:" }</span>
                    <span>{ user_info.role }</span>
                </div>
                </>
            }
        },
    )
}
