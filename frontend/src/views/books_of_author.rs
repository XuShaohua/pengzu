// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::{use_async_with_options, UseAsyncOptions};
use yew_router::history::Location;
use yew_router::hooks::use_location;

use crate::components::book_list::BookListComponent;
use crate::services::authors::fetch_author;
use crate::services::books::fetch_books_by_author;
use crate::types::books::GetBooksQuery;
use crate::views::util;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub author_id: i32,
}

#[function_component(BooksOfAuthorComponent)]
pub fn books_of_author(props: &Props) -> Html {
    util::set_document_title(&format!("Author: {}", props.author_id));

    let location = use_location().unwrap();
    let query = location.query::<GetBooksQuery>().unwrap();

    let author_id = props.author_id;
    let book_list = use_async_with_options(
        async move { fetch_books_by_author(author_id, &query).await },
        UseAsyncOptions::enable_auto(),
    );

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

    book_list.data.as_ref().map_or_else(
        || html! {},
        |book_list| {
            html! {
                <>
                { title_element }
                <BookListComponent books={ book_list.list.clone() } />
                </>
            }
        },
    )
}
