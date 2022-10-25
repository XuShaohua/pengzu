// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::Link;

use crate::router::Route;
use crate::types::books::Book;
use crate::views::util::get_cover_image_url;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub books: Vec<Book>,
}

fn generate_book_element(book_resp: &Book) -> Html {
    let book = &book_resp.book;
    let authors = &book_resp.authors;
    let authors_element = authors
        .iter()
        .enumerate()
        .map(|(index, author)| {
            let delimiter = if authors.len() - index > 1 {
                html! { <span>{ " & " }</span> }
            } else {
                html! {}
            };

            html! {
                <>
                <Link<Route> to={ Route::BooksOfAuthor { author_id: author.id } }>
                    { &author.name }
                </Link<Route>>

                { delimiter }
                </>
            }
        })
        .collect::<Html>();

    html! {
        <div class="book-fluid" key={ book.id }>
            <div class="book-cover">
                <Link<Route> to={ Route::BookDetail { book_id: book.id } }>
                    <img src={ get_cover_image_url(&book.small_cover) } alt={ book.title.clone() } />
                </Link<Route>>
            </div>
            <div class="book-meta">
                <Link<Route> to={ Route::BookDetail { book_id: book.id } }>
                    <span class="book-title" title={ book.title.clone() }>{ &book.title }</span>
                </Link<Route>>

                <div class="book-authors">{ authors_element }</div>
            </div>
        </div>
    }
}

#[function_component(BookListComponent)]
pub fn book_list(props: &Props) -> Html {
    let style_str = include_str!("book_list.css");
    let style_cls = Style::new(style_str).expect("Invalid style file book_list.css");

    let book_elements = props
        .books
        .iter()
        .map(generate_book_element)
        .collect::<Html>();

    html! {
        <div class={ style_cls }>
            { book_elements }
        </div>
    }
}
