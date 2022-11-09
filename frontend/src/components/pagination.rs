// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::page::PageId;
use stylist::Style;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub current_page: PageId,
    pub total_pages: PageId,
    pub onclick: Callback<PageId>,
}

#[function_component(PaginationComponent)]
pub fn pagination(props: &Props) -> Html {
    let style_str = include_str!("pagination.css");
    let style_cls = Style::new(style_str).expect("Invalid style file pagination.css");

    let has_previous = props.current_page > 1;
    let has_next = props.current_page + 1 < props.total_pages;

    let mut pages = Vec::new();
    let min_pages = 12;
    if props.total_pages < min_pages {
        for i in 1..=props.total_pages {
            pages.push(Some(i));
        }
    } else {
        let left_page = 5;
        if props.current_page < left_page && props.total_pages > left_page {
            for i in 1..=left_page {
                pages.push(Some(i));
            }
        }
        pages.push(None);

        let right_page = props.total_pages - left_page;
        for i in right_page..props.total_pages {
            pages.push(Some(i));
        }
    }

    let buttons = pages
        .iter()
        .map(|page_id| {
            page_id.map_or_else(
                || {
                    html! {
                        <li class="page">
                            <button class="btn" disabled={true}>{ ".." }</button>
                        </li>
                    }
                },
                |page_id| {
                    let props_onclick = props.onclick.clone();
                    let onclick = Callback::from(move |event: MouseEvent| {
                        event.prevent_default();
                        props_onclick.emit(page_id);
                    });

                    let button_cls = if page_id == props.current_page {
                        "page active"
                    } else {
                        "page"
                    };

                    html! {
                        <li class={ button_cls }>
                            <button onclick={onclick} class="btn">{ page_id }</button>
                        </li>
                    }
                },
            )
        })
        .collect::<Html>();

    html! {
        <ul class={ style_cls }>
            if has_previous {
                <li class="page previous">
                    <button class="btn">{ "« Previous" }</button>
                </li>
            }

            { buttons }

            if has_next {
                <li class="page next">
                    <button class="btn">{ "Next »" }</button>
                </li>
            }
        </ul>
    }
}
