// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

#[function_component(FooterComponent)]
pub fn footer() -> Html {
    html! {
        <footer class="mt-3 fw-light text-center">
            {"© 2022-2023 Pengzu Library"}
        </footer>
    }
}
