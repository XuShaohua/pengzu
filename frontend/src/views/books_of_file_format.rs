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
use crate::services::file_formats::{fetch_books_by_file_format, fetch_file_format};
use crate::views::util;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub format_id: i32,
}

#[function_component(BooksOfFileFormatComponent)]
pub fn books_of_file_format(props: &Props) -> Html {
    util::set_document_title(&format!("Format: {}", props.format_id));

    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap_or_default();
    let book_list = {
        let format_id = props.format_id;
        let query_clone = query.clone();
        use_async(async move {
            util::scroll_to_top();
            fetch_books_by_file_format(format_id, &query_clone).await
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

    let format_info = {
        let format_id = props.format_id;
        use_async_with_options(
            async move { fetch_file_format(format_id).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    let title_element = format_info.data.as_ref().map_or_else(
        || html! {},
        |format_info| {
            util::set_document_title(&format!("Format: {}", format_info.name));

            html! {
                <h2>{ format!("Books of \"{}\"", format_info.name) }</h2>
            }
        },
    );

    let on_book_filter_change = {
        let query_clone = query.clone();
        let format_id = props.format_id;
        Callback::from(move |order: GetBooksOrder| {
            let new_query = GetBooksQuery {
                order,
                ..query_clone
            };
            let ret =
                navigator.push_with_query(&Route::BooksOfFileFormat { format_id }, &new_query);
            debug_assert!(ret.is_ok());
        })
    };

    let pagination_link = {
        let format_id = props.format_id;
        Callback::from(
            move |(page_id, classes, title): (PageId, &'static str, String)| -> Html {
                let new_query = GetBooksQuery {
                    page: page_id,
                    ..query
                };

                html! {
                    <Link<Route, GetBooksQuery> to={ Route::BooksOfFileFormat { format_id } }
                        query={ Some(new_query) } classes={ classes }>
                        { title }
                    </Link<Route, GetBooksQuery>>
                }
            },
        )
    };

    book_list.data.as_ref().map_or_else(
        || html! { <h2>{ "Books of format" }</h2> },
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
