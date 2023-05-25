// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::tags::Tag;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
    pub tags: Vec<Tag>,
}

#[function_component(EditTagsComponent)]
pub fn edit_tags(props: &Props) -> Html {
    let tags = &props.tags;
    tags
        .iter()
        .enumerate()
        .map(|(index, tag)| {
            let delimiter = if tags.len() - index > 1 {
                html! { <span>{ " & " }</span> }
            } else {
                html! {}
            };
            html! {
                <span key={ tag.id }>
                    <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id } }>{ &tag.name } </Link<Route>>
                    { delimiter }
                </span>
            }
        })
        .collect::<Html>()
}
