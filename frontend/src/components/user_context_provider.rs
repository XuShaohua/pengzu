// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use crate::error::ErrorKind;
use crate::services::auth::{get_token, set_token};
use crate::services::users::get_user_info;
use crate::types::users::UserInfo;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx = use_state(UserInfo::default);
    let current_user = use_async(async move { get_user_info().await });

    {
        let current_user = current_user.clone();
        use_mount(move || {
            if get_token().is_some() {
                current_user.run();
            }
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    user_ctx.set(user_info.clone());
                }
                if let Some(error) = &current_user.error {
                    match error.kind() {
                        ErrorKind::Unauthorized | ErrorKind::Forbidden => set_token(None),
                        _ => (),
                    }
                }
                || ()
            },
            current_user,
        )
    }

    return html! {
        <ContextProvider<UseStateHandle<UserInfo>> context={user_ctx}>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserInfo>>>
    };
}
