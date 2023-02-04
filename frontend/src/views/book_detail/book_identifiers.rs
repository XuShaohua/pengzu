// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::identifiers::IdentifierWithType;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub identifiers: Vec<IdentifierWithType>,
}

#[function_component(BookIdentifiersComponent)]
pub fn book_identifiers(props: &Props) -> Html {
    let identifiers = &props.identifiers;
    identifiers
        .iter()
        .enumerate()
        .map(|(index, identifier)| {
            let delimiter = if identifiers.len() - index > 1 {
                html! { <span>{ " & " }</span> }
            } else {
                html! {}
            };
            html! {
                <span key={ identifier.id }>
                    <span title={ identifier.name.clone() }>
                        { &identifier.value }
                    </span>
                    { delimiter }
                </span>
            }
        })
        .collect::<Html>()
}
