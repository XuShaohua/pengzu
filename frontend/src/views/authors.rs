// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::authors::AuthorAndBook;
use shared::general_query::{GeneralOrder, GeneralQuery};
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_location, use_navigator, Link};

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::authors::fetch_authors;
use crate::views::util;

fn generate_author_list(author_list: &[AuthorAndBook]) -> Html {
    html! {
        <ul class="col-xs-12 col-sm-6 list-unstyled">
            {for author_list.iter().map(|author| html! {
                <li class="mb-2" key={ author.id }>
                    <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ author.count }</span>
                    <Link<Route> to={ Route::BooksOfAuthor { author_id: author.id }}>
                        { &author.name }
                    </Link<Route>>
                </li>
            })}
        </ul>
    }
}

#[function_component(AuthorsComponent)]
pub fn home() -> Html {
    util::set_document_title("Authors");

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GeneralQuery>().unwrap_or_default();
    let author_list = {
        let query_clone = query.clone();
        use_async(async move {
            util::scroll_to_top();
            fetch_authors(&query_clone).await
        })
    };
    {
        let author_list_clone = author_list.clone();
        use_effect_with_deps(
            move |_query_clone| {
                author_list_clone.run();
                || ()
            },
            query.clone(),
        );
    }

    let on_filter_change = {
        let query_clone = query.clone();
        Callback::from(move |order: GeneralOrder| {
            let new_query = GeneralQuery {
                order,
                ..query_clone
            };
            let ret = navigator.push_with_query(&Route::Author, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let pagination_link = {
        let query_clone = query.clone();
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = GeneralQuery {
                    page: page_id,
                    ..query_clone
                };
                html! {
                    <Link<Route, GeneralQuery> to={ Route::Author }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, GeneralQuery>>
                }
            },
        )
    };

    author_list.data.as_ref().map_or_else(
        || html! {},
        |author_list| {
            let half_list = (author_list.list.len() + 1) / 2;

            html! {
                <>
                <h2>{ "Authors" }</h2>
                <GeneralFilterComponent onchange={ on_filter_change } current_order={ query.order } />

                <div class="container-fluid">
                    <div class="row">
                        { generate_author_list(&author_list.list[..half_list]) }
                        { generate_author_list(&author_list.list[half_list..]) }
                    </div>
                </div>

                <PaginationComponent  current_page={ author_list.page.page_num }
                    total_pages={ author_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
