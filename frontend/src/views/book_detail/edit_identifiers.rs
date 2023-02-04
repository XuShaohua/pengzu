// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::identifiers::IdentifierWithType;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub identifiers: Vec<IdentifierWithType>,
}

#[function_component(EditIdentifiersComponent)]
pub fn edit_identifiers(props: &Props) -> Html {
    let identifiers = &props.identifiers;
    let elements = identifiers
        .iter()
        .map(|identifier| {
            html! {
                <li key={ identifier.id }>
                    <span class="fw-bold me-2">{ identifier.name.to_uppercase() }</span>
                        { &identifier.value }
                </li>
            }
        })
        .collect::<Html>();

    html! {
        <ol>
            { elements }
        </ol>
    }
}
