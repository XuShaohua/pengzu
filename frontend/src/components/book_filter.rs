// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use stylist::Style;
use yew::prelude::*;

use crate::types::books_query::GetBooksOrder;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub current_order: GetBooksOrder,
    pub onchange: Callback<GetBooksOrder>,
}

#[function_component(BookFilterComponent)]
pub fn book_filter(props: &Props) -> Html {
    let style_str = include_str!("book_filter.css");
    let style_cls = Style::new(style_str).expect("Invalid book_filter.css file");

    let button_onclick = |order: GetBooksOrder| {
        let onchange = props.onchange.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            onchange.emit(order);
        })
    };

    let get_button_cls = {
        let active_order = props.current_order;
        move |order: GetBooksOrder| {
            if order == active_order {
                "btn btn-primary active"
            } else {
                "btn btn-primary"
            }
        }
    };

    html! {
        <div class={ style_cls }>

        <button class={ get_button_cls(GetBooksOrder::IdAsc) }
            title={ "Sort according to book date, newest first" }
            onclick={ button_onclick(GetBooksOrder::IdAsc) }>
            <span class="glyphicon glyphicon-sort-by-order" />
        </button>
        <button class={ get_button_cls(GetBooksOrder::IdDesc) }
            title={ "Sort according to book date, oldest first" }
            onclick={ button_onclick(GetBooksOrder::IdDesc) }>
            <span class="glyphicon glyphicon-sort-by-order-alt" />
        </button>

        <button class={ get_button_cls(GetBooksOrder::TitleAsc) }
            title={ "Sort titles in alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::TitleAsc) }>
            <span class="glyphicon glyphicon-font" />
            <span class="glyphicon glyphicon-sort-by-alphabet" />
        </button>
        <button class={ get_button_cls(GetBooksOrder::TitleDesc) }
            title={ "Sort titles in reverse alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::TitleDesc) }>
            <span class="glyphicon glyphicon-font" />
            <span class="glyphicon glyphicon-sort-by-alphabet-alt" />
        </button>

        <button class={ get_button_cls(GetBooksOrder::AuthorAsc) }
            title={ "Sort authors in alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::AuthorAsc) }>
            <span class="glyphicon glyphicon-user" />
            <span class="glyphicon glyphicon-sort-by-alphabet" />
        </button>
        <button class={ get_button_cls(GetBooksOrder::AuthorDesc) }
            title={ "Sort authors in reverse alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::AuthorDesc) }>
            <span class="glyphicon glyphicon-user" />
            <span class="glyphicon glyphicon-sort-by-alphabet-alt" />
        </button>

        <button class={ get_button_cls(GetBooksOrder::PubdateAsc) }
            title={ "Sort according to publishing date, newest first" }
            onclick={ button_onclick(GetBooksOrder::PubdateAsc) }>
            <span class="glyphicon glyphicon-calender" />
            <span class="glyphicon glyphicon-sort-by-order" />
        </button>
        <button class={ get_button_cls(GetBooksOrder::PubdateDesc) }
            title={ "Sort according to publishing date, oldest first" }
            onclick={ button_onclick(GetBooksOrder::PubdateDesc) }>
            <span class="glyphicon glyphicon-calendar" />
            <span class="glyphicon glyphicon-sort-by-order-alt" />
        </button>

        </div>
    }
}
