// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::views::util;
use yew::prelude::*;

#[function_component(UserInfoComponent)]
pub fn user_info() -> Html {
    util::set_document_title("User Info");

    html! {
      <h2>{ "User Info" }</h2>
    }
}
