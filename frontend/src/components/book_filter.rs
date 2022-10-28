// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::types::books::GetBooksOrder;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub onchange: Callback<GetBooksOrder>,
}

#[function_component(BookFilterComponent)]
pub fn book_filter(props: &Props) -> Html {
    let button_onclick = |order: GetBooksOrder| {
        let onchange = props.onchange.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            onchange.emit(order);
        })
    };

    html! {
        <div>

        <button class="btn btn-primary"
            title={ "Sort according to book date, newest first" }
            onclick={ button_onclick(GetBooksOrder::IdAsc) }>
            <span class="glyphicon glyphicon-sort-by-order" />
        </button>
        <button class="btn btn-primary"
            title={ "Sort according to book date, oldest first" }
            onclick={ button_onclick(GetBooksOrder::IdDesc) }>
            <span class="glyphicon glyphicon-sort-by-order-alt" />
        </button>

        <button class="btn btn-primary"
            title={ "Sort titles in alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::TitleAsc) }>
            <span class="glyphicon glyphicon-font" />
            <span class="glyphicon glyphicon-sort-by-alphabet" />
        </button>
        <button class="btn btn-primary"
            title={ "Sort titles in reverse alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::TitleAsc) }>
            <span class="glyphicon glyphicon-font" />
            <span class="glyphicon glyphicon-sort-by-alphabet-alt" />
        </button>

        <button class="btn btn-primary"
            title={ "Sort authors in alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::AuthorAsc) }>
            <span class="glyphicon glyphicon-user" />
            <span class="glyphicon glyphicon-sort-by-alphabet" />
        </button>
        <button class="btn btn-primary"
            title={ "Sort authors in reverse alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::AuthorDesc) }>
            <span class="glyphicon glyphicon-user" />
            <span class="glyphicon glyphicon-sort-by-alphabet-alt" />
        </button>

        <button class="btn btn-primary"
            title={ "Sort according to publishing date, newest first" }
            onclick={ button_onclick(GetBooksOrder::PubdateAsc) }>
            <span class="glyphicon glyphicon-calender" />
            <span class="glyphicon glyphicon-sort-by-order" />
        </button>
        <button class="btn btn-primary"
            title={ "Sort according to publishing date, oldest first" }
            onclick={ button_onclick(GetBooksOrder::PubdateDesc) }>
            <span class="glyphicon glyphicon-calendar" />
            <span class="glyphicon glyphicon-sort-by-order-alt" />
        </button>

        </div>
    }
}
