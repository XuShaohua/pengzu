// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::authors::Author;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
    pub authors: Vec<Author>,
}

#[function_component(EditAuthorsComponent)]
pub fn edit_authors(props: &Props) -> Html {
    let authors = &props.authors;

    authors
        .iter()
        .enumerate()
        .map(|(index, author)| {
            let delimiter = if authors.len() - index > 1 {
                html! { <span>{ " & " }</span> }
            } else {
                html! {}
            };

            html! {
                <>
                <Link<Route> to={ Route::BooksOfAuthor { author_id: author.id } }>
                    { &author.name }
                </Link<Route>>
                { delimiter }
                </>
            }
        })
        .collect::<Html>()
}
