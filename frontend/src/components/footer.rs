// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use stylist::Style;
use yew::prelude::*;

#[function_component(FooterComponent)]
pub fn footer() -> Html {
    let style_str = include_str!("footer.css");
    let style_cls = Style::new(style_str).expect("Invalid style file footer.css");

    html! {
        <footer class={ style_cls }>
            {"© 2022 Shaohua"}
        </footer>
    }
}
