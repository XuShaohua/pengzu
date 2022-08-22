// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::components::util::get_cover_image_url;
use crate::services::books::BookResp;

#[derive(PartialEq)]
pub enum Msg {}

#[derive(Debug, PartialEq, Properties)]
pub struct Prop {
    pub books: Vec<BookResp>,
}

pub struct BookListComponent {}

fn generate_book_element(book_resp: &BookResp) -> Html {
    let book = &book_resp.book;
    let authors = &book_resp.authors;
    let authors_element = authors
        .iter()
        .enumerate()
        .map(|(index, author)| {
            let delimiter = if authors.len() - index > 1 {
                html!{ <span>{ " & " }</span> }
            } else {
                html!{}
            };

            html! {
                <>
                <a key={ author.id } href={ format!("/author/books/{:?}", author.id) } target="_blank">
                    { author.name.clone() }
                </a>

                { delimiter }
                </>
            }
        })
        .collect::<Html>();

    html! {
        <div class="book-fluid" key={ book.id }>
            <div class="book-cover">
                <a href={ format!("/book/{}", book.id) } target="_blank">
                    <img src={ get_cover_image_url(&book.small_cover) } alt={ book.title.clone() } />
                </a>
            </div>
            <div class="book-meta">
                <a href={ format!("/book/{}", book.id) } target="_blank">
                    <span class="book-title" title={ book.title.clone() }>{ book.title.clone() }</span>
                </a>

                <div class="book-authors">{ authors_element }</div>
            </div>
        </div>
    }
}

impl Component for BookListComponent {
    type Message = Msg;
    type Properties = Prop;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let book_elements = ctx
            .props()
            .books
            .iter()
            .map(generate_book_element)
            .collect::<Html>();

        html! {
            <div class="book-list">
                { book_elements }
            </div>
        }
    }
}
