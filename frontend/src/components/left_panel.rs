// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

pub enum Msg {}

pub struct LeftPanelComponent {}

impl Component for LeftPanelComponent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="left-panel">
                <ul>
                    <li><a href="/">{"Books"}</a></li>
                    <li><a href="/authors">{"Authors"}</a></li>
                    <li><a href="/categories">{"Categories"}</a></li>
                    <li><a href="/tags">{"Tags"}</a></li>
                    <li><a href="/publishers">{"Publishers"}</a></li>
                    <li><a href="/series">{"Series"}</a></li>
                    <li><a href="/discover">{"Discover"}</a></li>
                    <li><a href="/file-formats">{"File Formats"}</a></li>
                    <li><a href="/ratings">{"Ratings"}</a></li>
                </ul>
            </div>
        }
    }
}
