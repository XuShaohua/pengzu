// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::BrowserRouter;

use crate::components::main_content::MainContentComponent;
use crate::components::user_context_provider::UserContextProvider;

#[function_component(AppComponent)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <MainContentComponent />
            </BrowserRouter>
        </UserContextProvider>
    }
}
