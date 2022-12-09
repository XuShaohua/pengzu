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

const MIN_PAGES: PageId = 12;
const PER_SIDE: PageId = 5;

#[function_component(PaginationComponent)]
pub fn pagination(props: &Props) -> Html {
    if props.total_pages == 0 {
        return html! {};
    }

    let mut pages = Vec::new();
    if props.total_pages == 1 {
        // Do not add any links at all.
    } else if props.total_pages < MIN_PAGES {
        // Show all links.
        for i in 1..=props.total_pages {
            pages.push(Some(i));
        }
    } else {
        let right_page = props.total_pages - PER_SIDE;
        if props.current_page < PER_SIDE {
            for i in 1..=PER_SIDE {
                pages.push(Some(i));
            }

            pages.push(None);
            pages.push(Some(props.total_pages));
        } else if props.current_page > right_page {
            pages.push(Some(1));
            pages.push(None);
            for i in right_page..=props.total_pages {
                pages.push(Some(i));
            }
        } else {
            pages.push(Some(1));
            pages.push(None);
            for i in props.current_page - 2..=props.current_page + 2 {
                pages.push(Some(i));
            }
            pages.push(None);
            pages.push(Some(props.total_pages));
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
                    let link_cls = if page_id == props.current_page {
                        "page-link active"
                    } else {
                        "page-link"
                    };

                    html! {
                        <li class="page-item">
                            { props.link.emit((page_id, link_cls, page_id.to_string()))}
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
            <li class="page-item">
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
            <li class="page-item">
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
