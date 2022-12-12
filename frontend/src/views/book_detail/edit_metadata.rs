// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::services::books::update_book;
use shared::books::BookUpdateReq;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
    pub title: String,
}

#[function_component(EditMetadataComponent)]
pub fn edit_metadata(props: &Props) -> Html {
    let edit_title_ref = use_node_ref();

    let query = use_state(|| BookUpdateReq {
        id: props.book_id,
        title: props.title.clone(),
    });
    let update_book_wrapper = {
        let query_clone = query.clone();
        // TODO(Shaohua): Show response status.
        use_async(async move { update_book(&query_clone).await })
    };

    let on_form_submit = {
        let edit_title_ref_clone = edit_title_ref.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            let mut query_form = (*query).clone();

            let mut metadata_changed = false;

            if let Some(input) = edit_title_ref_clone.cast::<HtmlInputElement>() {
                let value = input.value();
                if !value.is_empty() && value != query_form.title {
                    query_form.title = value;
                    metadata_changed = true;
                }
            }

            if metadata_changed {
                query.set(query_form);
                update_book_wrapper.run();
            }
        })
    };

    let on_input_focus = {
        let edit_title_ref_clone = edit_title_ref.clone();
        Callback::from(move |_event: FocusEvent| {
            if let Some(input) = edit_title_ref_clone.cast::<HtmlInputElement>() {
                input.select();
            }
        })
    };

    html! {
        <form onsubmit={ on_form_submit }>
            <input type="text" class="form-control"
                onfocus={ on_input_focus }
                ref={ edit_title_ref }
                name="title" value={ props.title.clone() } />
            <button type="submit" class="btn btn-primary">{ "Update" }</button>
        </form>
    }
}
