// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookUpdateReq;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
    pub title: String,
}

#[function_component(EditMetadataComponent)]
pub fn edit_metadata(props: &Props) -> Html {
    let edit_title_ref = use_node_ref();

    let on_form_submit = {
        let edit_title_ref_clone = edit_title_ref.clone();
        let book_id = props.book_id;
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let mut query = BookUpdateReq {
                id: book_id,
                ..BookUpdateReq::default()
            };
            if let Some(input) = edit_title_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() {
                    query.title = value;
                }
            }

            // let update_book_wrapper = use_async(async move { update_book(&query).await });
            // update_book_wrapper.run();
        })
    };

    html! {
        <div class="edit-book">
            <form onsubmit={ on_form_submit }>
                <input type="text" class="form-control"
                    ref={ edit_title_ref }
                    name="title" value={ props.title.clone() } />
                <button type="submit" class="btn btn-primary">{ "Update" }</button>
            </form>
        </div>
    }
}
