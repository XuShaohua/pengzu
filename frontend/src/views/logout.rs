// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::hooks::use_user_context;

#[function_component(LogoutComponent)]
pub fn logout_page() -> Html {
    let user_ctx = use_user_context();
    user_ctx.logout();

    // Redirect to login page.

    html! {
        <>
        </>
    }
}
