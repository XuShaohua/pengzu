// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub ok_cb: Callback<String>,
}

#[function_component(AddTagModalComponent)]
pub fn add_tag_modal(props: &Props) -> Html {
    let name_ref = use_node_ref();

    let on_add_button_clicked = {
        let name_ref_clone = name_ref.clone();
        let ok_cb_clone = props.ok_cb.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();

            if let Some(input) = name_ref_clone.cast::<HtmlInputElement>() {
                let name = input.value();
                if !name.is_empty() {
                    ok_cb_clone.emit(name);
                    // Clean input value.
                    input.set_value("");
                }
            }
        })
    };

    html! {
        <div class="modal-dialog modal-dialog-centered">
            <div class="modal-content">
                <div class="modal-header">
                    <h1 class="modal-title fs-5">{ "Add new user tag" }</h1>
                    <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>

                <div class="modal-body">
                    <div class="mb-3">
                        <label for="new-user-tag-name" class="form-label">{ "Name" }</label>
                        <input class="form-control" type="text" id="new-user-tag-name" name="name" ref={ name_ref } />
                    </div>
                </div>

                <div class="modal-footer">
                    <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                        { "Close "}
                    </button>
                    <button type="button" class="btn btn-primary"
                        data-bs-dismiss="modal"
                        onclick={ on_add_button_clicked }>
                        { "Add" }
                    </button>
                </div>
            </div>
        </div>
    }
}
