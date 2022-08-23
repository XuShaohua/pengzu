// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::Link;

use crate::route::Route;
use crate::services::authors::fetch_authors;

#[function_component(AuthorsComponent)]
pub fn home() -> Html {
    let author_list = {
        use_async_with_options(
            async move { fetch_authors().await },
            UseAsyncOptions::enable_auto(),
        )
    };

    if let Some(author_list) = &author_list.data {
        return html! {
             <ul class="author-list">
                 {for author_list.list.iter().map(|author| html! {
                     <li class="author-item" key={ author.id }>
                     <span class="badge">{ author.count }</span>
                     <Link<Route> to={ Route::BooksOfAuthor { author_id: author.id } } >
                     { &author.name }
                     </Link<Route>>
                     </li>
                 })}
             </ul>
        };
    } else {
        return html! {};
    }
}
