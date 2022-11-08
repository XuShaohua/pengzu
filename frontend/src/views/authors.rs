// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use stylist::Style;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::{use_history, History, Link, Location};

use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::authors::fetch_authors;
use crate::types::general_query::GeneralQuery;
use crate::types::page::PageId;
use crate::views::util;

#[function_component(AuthorsComponent)]
pub fn home() -> Html {
    util::set_document_title("Authors");

    let style_str = include_str!("authors.css");
    let style_cls = Style::new(style_str).expect("Invalid style file authors.css");

    let history = use_history().unwrap();
    let location = history.location();
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

    let pagination_onclick = Callback::from(move |page_id: PageId| {
        util::scroll_to_top();

        let new_query = GeneralQuery {
            page: page_id,
            ..query
        };
        let ret = history.push_with_query(Route::Author, &new_query);
        debug_assert!(ret.is_ok());
    });

    author_list.data.as_ref().map_or_else(
        || html! {},
        |author_list| {
            html! {
                <>
                <h2>{ "Authors" }</h2>
                <ul class={ style_cls }>
                {for author_list.list.iter().map(|author| html! {
                    <li class="author-item" key={ author.id }>
                        <span class="badge">{ author.count }</span>
                        <Link<Route> to={ Route::BooksOfAuthor { author_id: author.id } } >
                            { &author.name }
                        </Link<Route>>
                    </li>
                })}
                </ul>
                <PaginationComponent  current_page={ author_list.page.page_num }
                    total_pages={ author_list.page.total_pages() }
                    onclick={ pagination_onclick } />
                </>
            }
        },
    )
}
