// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::page::PageId;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub current_page: PageId,
    pub total_pages: PageId,
    pub onclick: Callback<PageId>,
}

#[function_component(PaginationComponent)]
pub fn pagination(props: &Props) -> Html {
    let has_previous = props.current_page > 1;
    let has_next = props.current_page < props.total_pages;
    let has_no_page = props.total_pages == 0;

    let mut pages = Vec::new();
    let min_pages = 12;
    if props.total_pages == 1 {
        // Do not add any buttons at all.
    } else if props.total_pages < min_pages {
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
                        <li class="page-item">
                            <span class="page-link disabled">{ ".." }</span>
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
                        "page-item active"
                    } else {
                        "page-item"
                    };

                    html! {
                        <li class={ button_cls }>
                            <a onclick={onclick} href="#" class="page-link">{ page_id }</a>
                        </li>
                    }
                },
            )
        })
        .collect::<Html>();

    let previous_onclick = {
        let previous_page_id = props.current_page - 1;
        let props_onclick = props.onclick.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            props_onclick.emit(previous_page_id);
        })
    };

    let next_onclick = {
        let next_page_id = props.current_page + 1;
        let props_onclick = props.onclick.clone();
        Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            props_onclick.emit(next_page_id);
        })
    };

    let previous_class = if has_no_page {
        "page-item invisible"
    } else if has_previous {
        "page-item"
    } else {
        "page-item disabled"
    };
    let next_class = if has_no_page {
        "page-item invisible"
    } else if has_next {
        "page-item"
    } else {
        "page-item disabled"
    };

    html! {
        <nav aria-label="Pagination">
        <ul class="pagination">
            <li class={ previous_class }>
                <a class="page-link" href="#" onclick={ previous_onclick }>{ "« Previous" }</a>
            </li>

            { buttons }

            <li class={ next_class }>
                <a class="page-link" href="#" onclick={ next_onclick }>{ "Next »" }</a>
            </li>
        </ul>
        </nav>
    }
}
