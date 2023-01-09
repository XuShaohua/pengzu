// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::tags::NewTag;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub order_index: i32,
    pub name: String,
    #[prop_or_default]
    pub parent: i32,

    pub cancel_cb: Callback<()>,
    pub ok_cb: Callback<NewTag>,
}

#[function_component(EditTagFormComponent)]
pub fn edit_tag_form(props: &Props) -> Html {
    let name_ref = use_node_ref();

    let on_cancel_clicked = {
        let cancel_cb_clone = props.cancel_cb.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            cancel_cb_clone.emit(());
        })
    };

    let on_form_submit = {
        let order_index = props.order_index;
        let parent = props.parent;
        let name_ref_clone = name_ref.clone();
        let ok_cb_clone = props.ok_cb.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            if let Some(input) = name_ref_clone.cast::<HtmlInputElement>() {
                let name = input.value();
                if !name.is_empty() {
                    let new_tag = NewTag {
                        order_index,
                        name,
                        parent,
                    };
                    ok_cb_clone.emit(new_tag);
                }
            }
        })
    };

    html! {
        <>
        <h3>{ "Edit tag" }</h3>

        <form onsubmit={ on_form_submit }>
            <div>
                <label for="edit-tag-name" class="form-label">{ "Name" }</label>
                <input type="text" id="edit-tag-name" name="name" ref={ name_ref }
                    value={ props.name.clone() } />
            </div>
            <button type="button" onclick={ on_cancel_clicked }>{" Cancel" }</button>
            <button type="submit">{ "Update" }</button>
        </form>
        </>
    }
}
