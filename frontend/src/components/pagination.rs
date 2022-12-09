// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::page::PageId;
use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub current_page: PageId,
    pub total_pages: PageId,
    pub link: Callback<(PageId, &'static str, String), Html>,
}

#[function_component(PaginationComponent)]
pub fn pagination(props: &Props) -> Html {
    if props.total_pages == 0 {
        return html! {};
    }

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

    let link_elements = pages
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
                    let button_cls = if page_id == props.current_page {
                        "page-item active"
                    } else {
                        "page-item"
                    };

                    html! {
                        <li class={ button_cls }>
                            { props.link.emit((page_id, "page-link", page_id.to_string()))}
                        </li>
                    }
                },
            )
        })
        .collect::<Html>();

    let previous_element = if props.current_page > 1 {
        html! {
            <li class="page-item">
                { props.link.emit((props.current_page - 1, "page-link", "« Previous".to_owned())) }
            </li>
        }
    } else {
        html! {
            <li class="page-item disabled">
                <span class="page-link disabled">{ "« Previous" }</span>
            </li>
        }
    };
    let next_element = if props.current_page < props.total_pages {
        html! {
            <li class="page-item">
                { props.link.emit((props.current_page + 1, "page-link", "Next »".to_owned())) }
            </li>
        }
    } else {
        html! {
            <li class="page-item disabled">
                <span class="page-link disabled">{ "Next »" }</span>
            </li>
        }
    };

    html! {
        <nav aria-label="Pagination">
        <ul class="pagination justify-content-center">
            { previous_element }
            { link_elements }
            { next_element }
        </ul>
        </nav>
    }
}
