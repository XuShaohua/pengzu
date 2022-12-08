// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::authors::AuthorAndBook;
use shared::general_query::{GeneralOrder, GeneralQuery};
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::hooks::{use_location, use_navigator};
use yew_router::prelude::Link;

use crate::components::general_filter::GeneralFilterComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::authors::fetch_authors;
use crate::views::util;

fn generate_list(author_list: &[AuthorAndBook]) -> Html {
    html! {
        <div class="col-xs-12 col-sm-6">
            {for author_list.iter().map(|author| html! {
                <div class="mb-2" key={ author.id }>
                    <span class="badge rounded-pill d-inline me-2 text-bg-secondary">{ author.count }</span>
                    <Link<Route> to={ Route::BooksOfAuthor { author_id: author.id }}>
                        { &author.name }
                    </Link<Route>>
                </div>
            })}
        </div>
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
        use_async(async move { fetch_authors(&query_clone).await })
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

    let filter_onchange = {
        Callback::from(|order: GeneralOrder| {
            log::info!("new order: {:?}", order);
        })
    };

    let on_pagination_click = {
        let query_clone = query.clone();
        Callback::from(move |page_id: PageId| {
            util::scroll_to_top();

            let new_query = GeneralQuery {
                page: page_id,
                ..query_clone
            };
            let ret = navigator.push_with_query(&Route::Author, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    author_list.data.as_ref().map_or_else(
        || html! {},
        |author_list| {
            let half_list = author_list.list.len() / 2;

            html! {
                <>
                <h2>{ "Authors" }</h2>
                <GeneralFilterComponent onchange={ filter_onchange } current_order={ query.order } />

                <div class="container-fluid">
                    <div class="row">
                        { generate_list(&author_list.list[..half_list]) }
                        { generate_list(&author_list.list[half_list..]) }
                    </div>
                </div>

                <PaginationComponent  current_page={ author_list.page.page_num }
                    total_pages={ author_list.page.total_pages() }
                    onclick={ on_pagination_click } />
                </>
            }
        },
    )
}
