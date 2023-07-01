// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books_query::{GetBooksOrder, GetBooksQuery};
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};
use yew_router::prelude::{use_location, use_navigator, Link};

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::categories::{fetch_books_by_category, fetch_category};
use crate::views::util;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Properties)]
pub struct Props {
    pub category_id: i32,
}

#[function_component(BooksOfCategoryComponent)]
pub fn books_of_category(props: &Props) -> Html {
    util::set_document_title(&format!("Category: {}", props.category_id));

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let category_id = props.category_id;
        let query_clone = query.clone();
        use_async(async move {
            util::scroll_to_top();
            fetch_books_by_category(category_id, &query_clone).await
        })
    };
    {
        let book_list_clone = book_list.clone();
        use_effect_with_deps(
            move |_query_clone| {
                book_list_clone.run();
                || ()
            },
            query.clone(),
        );
    }

    let category_info = {
        let category_id = props.category_id;
        use_async_with_options(
            async move { fetch_category(category_id).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let title_element = category_info.data.as_ref().map_or_else(
        || html! {},
        |category_info| {
            util::set_document_title(&format!("Category: {}", category_info.name));

            html! {
                <h2>{ format!("Books of category \"{}\"", category_info.name) }</h2>
            }
        },
    );

    let on_book_filter_change = {
        let category_id = props.category_id;
        Callback::from(move |order: GetBooksOrder| {
            let new_query = GetBooksQuery { order, ..query };
            let ret =
                navigator.push_with_query(&Route::BooksOfCategory { category_id }, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let pagination_link = {
        let category_id = props.category_id;
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = GetBooksQuery {
                    page: page_id,
                    ..query
                };
                html! {
                    <Link<Route, GetBooksQuery> to={ Route::BooksOfCategory { category_id } }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, GetBooksQuery>>
                }
            },
        )
    };

    book_list.data.as_ref().map_or_else(
        || html! { <h2>{ "Books of category" }</h2> },
        |book_list| {
            html! {
                <>
                { title_element }
                <BookFilterComponent onchange={ on_book_filter_change } current_order={ query.order } />
                <BookListComponent books={ book_list.list.clone() } />
                <PaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    link={ pagination_link } />
                </>
            }
        },
    )
}
