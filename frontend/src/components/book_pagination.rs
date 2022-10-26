// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use crate::types::page::PageId;
use stylist::Style;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub current_page: PageId,
    pub total_pages: PageId,
    pub onclick: Callback<PageId>,
}

#[function_component(BookPaginationComponent)]
pub fn book_pagination(props: &Props) -> Html {
    log::info!(
        "current page: {}, total pages: {}",
        props.current_page,
        props.total_pages
    );

    let style_str = include_str!("book_pagination.css");
    let style_cls = Style::new(style_str).expect("Invalid style file book_pagination.css");

    html! {
        <div class={ style_cls }>
            <button>{ "Previous" }</button>
            <button>{ props.current_page }</button>
            <button>{ props.total_pages }</button>
            <button>{ "Next" }</button>
        </div>
    }
}
