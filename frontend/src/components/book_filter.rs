// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books_query::GetBooksOrder;
use stylist::Style;
use yew::prelude::*;

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
            <i class="bi bi-sort-numeric-down"></i>
        </button>
        <button class={ get_button_cls(GetBooksOrder::IdDesc) }
            title={ "Sort according to book date, oldest first" }
            onclick={ button_onclick(GetBooksOrder::IdDesc) }>
            <i class="bi bi-sort-numeric-down-alt"></i>
        </button>

        <button class={ get_button_cls(GetBooksOrder::TitleAsc) }
            title={ "Sort titles in alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::TitleAsc) }>
            <i class="bi bi-sort-alpha-down"></i>
        </button>
        <button class={ get_button_cls(GetBooksOrder::TitleDesc) }
            title={ "Sort titles in reverse alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::TitleDesc) }>
            <i class="bi bi-sort-alpha-down-alt"></i>
        </button>

        <button class={ get_button_cls(GetBooksOrder::AuthorAsc) }
            title={ "Sort authors in alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::AuthorAsc) }>
            <i class="bi bi-person"></i>
            <i class="bi bi-sort-alpha-down-alt"></i>
        </button>
        <button class={ get_button_cls(GetBooksOrder::AuthorDesc) }
            title={ "Sort authors in reverse alphabetical order" }
            onclick={ button_onclick(GetBooksOrder::AuthorDesc) }>
            <i class="bi bi-person"></i>
            <i class="bi bi-sort-alpha-down-alt"></i>
        </button>

        <button class={ get_button_cls(GetBooksOrder::PubdateAsc) }
            title={ "Sort according to publishing date, newest first" }
            onclick={ button_onclick(GetBooksOrder::PubdateAsc) }>
            <i class="bi bi-calendar-date"></i>
            <i class="bi bi-sort-numeric-down"></i>
        </button>
        <button class={ get_button_cls(GetBooksOrder::PubdateDesc) }
            title={ "Sort according to publishing date, oldest first" }
            onclick={ button_onclick(GetBooksOrder::PubdateDesc) }>
            <i class="bi bi-calendar-date"></i>
            <i class="bi bi-sort-numeric-down-alt"></i>
        </button>

        </div>
    }
}
