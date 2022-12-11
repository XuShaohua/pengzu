// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books::BookAndAuthors;
use yew::prelude::*;
use yew_router::prelude::{Link, Routable};

use crate::router::Route;
use crate::services::images::get_cover_image_url;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub books: Vec<BookAndAuthors>,
}

fn generate_book_element(book_resp: &BookAndAuthors) -> Html {
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
                <div class="fw-light wrap-text-no-line">
                    <Link<Route> to={ Route::BooksOfAuthor { author_id: author.id } }
                        classes="text-body">
                        { &author.name }
                    </Link<Route>>

                    { delimiter }
                </div >
            }
        })
        .collect::<Html>();

    let cover_url = get_cover_image_url(&book.small_cover);
    let detail_url = Route::BookDetail { book_id: book.id }.to_path();

    html! {
        <div class="col-lg-2 col-sm-4 col-xs-6" key={ book.id }>
            <a href={ detail_url.clone() } class="d-block p-2 mb-3" target="_blank">
                <img src={ cover_url } alt={ book.title.clone() }
                    style="border: 1px solid #fff; box-shadow: 0 5px 8px -6px #777;"
                    width="135" height="200" />
            </a>

            <div class="ms-1">
                <div class="">
                <a href={ detail_url } class="text-body mb-2" target="_blank">
                    <span class="wrap-text-l3" title={ book.title.clone() }>{ &book.title }</span>
                </a>
                </div>

                { authors_element }
            </div>
        </div>
    }
}

#[function_component(BookListComponent)]
pub fn book_list(props: &Props) -> Html {
    let book_elements = props
        .books
        .iter()
        .map(generate_book_element)
        .collect::<Html>();

    html! {
        <div class="d-flex flex-wrap mb-5">
            { book_elements }
        </div>
    }
}
