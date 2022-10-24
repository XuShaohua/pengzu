// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub query: String,
}

#[function_component(BooksOfSimpleSearchComponent)]
pub fn books_of_simple_search(props: &Props) -> Html {
    log::info!("query: {}", props.query);

    html! {
        <h1>{ "Simple Search" }</h1>
    }
}
