// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books_query::{GetBooksOrder, GetBooksQuery};
use shared::page::PageId;
use yew::prelude::*;
use yew_hooks::{use_async, use_async_with_options, UseAsyncOptions};
use yew_router::hooks::{use_location, use_navigator};

use crate::components::book_filter::BookFilterComponent;
use crate::components::book_list::BookListComponent;
use crate::components::pagination::PaginationComponent;
use crate::router::Route;
use crate::services::authors::{fetch_author, fetch_books_by_author};
use crate::views::util;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub author_id: i32,
}

#[function_component(BooksOfAuthorComponent)]
pub fn books_of_author(props: &Props) -> Html {
    util::set_document_title(&format!("Author: {}", props.author_id));

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let author_id = props.author_id;
        let query_clone = query.clone();
        use_async(async move { fetch_books_by_author(author_id, &query_clone).await })
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

    let author_id = props.author_id;
    let author_info = use_async_with_options(
        async move { fetch_author(author_id).await },
        UseAsyncOptions::enable_auto(),
    );

    let title_element = author_info.data.as_ref().map_or_else(
        || html! {},
        |author_info| {
            util::set_document_title(&format!("Author: {}", author_info.name));

            html! {
                <h2>{ format!("Books of \"{}\"", author_info.name) }</h2>
            }
        },
    );

    let on_book_filter_change = {
        let query_clone = query.clone();
        let navigator_clone = navigator.clone();
        Callback::from(move |order: GetBooksOrder| {
            util::scroll_to_top();

            let new_query = GetBooksQuery {
                order,
                ..query_clone
            };
            let ret =
                navigator_clone.push_with_query(&Route::BooksOfAuthor { author_id }, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let on_pagination_click = {
        let author_id = props.author_id;
        Callback::from(move |page_id: PageId| {
            util::scroll_to_top();

            let new_query = GetBooksQuery {
                page: page_id,
                ..query
            };
            let ret = navigator.push_with_query(&Route::BooksOfAuthor { author_id }, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                { title_element }
                <BookFilterComponent onchange={ on_book_filter_change } current_order={ query.order } />
                <BookListComponent books={ book_list.list.clone() } />
                <PaginationComponent current_page={ book_list.page.page_num }
                    total_pages={ book_list.page.total_pages() }
                    onclick={ on_pagination_click } />
                </>
            }
        },
    )
}
