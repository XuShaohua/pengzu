// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;

use crate::services::books_meta::fetch_book_metadata;
use crate::types::books_meta::BookMetadata;
use crate::types::error::FetchError;
use crate::views::util::get_cover_image_url;

#[derive(PartialEq)]
pub enum Msg {
    Fetch,
    FetchSuccess(BookMetadata),
    FetchFailed(FetchError),
}

#[derive(Debug, PartialEq, Properties)]
pub struct Prop {
    pub book_id: i32,
}

pub struct BookDetailComponent {
    metadata: Option<BookMetadata>,
}

fn generate_metadata_element(metadata: &BookMetadata) -> Html {
    let book = &metadata.book;

    let authors = &metadata.authors;
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

    let publisher_element = match &metadata.publisher {
        Some(publisher) => {
            html! {
                <span>
                    { "Publisher: " }
                    <a href={ format!("/publisher/{}", publisher.id) } target="_blank">{ publisher.name.clone() }</a>
                </span>
            }
        }
        None => html! {<></>},
    };

    let published_date_element = if let Some(pubdate) = book.pubdate {
        html! {
            <span>{ format!("Published At: {:?}", pubdate) }</span>
        }
    } else {
        html! {}
    };

    let tags = &metadata.tags;
    let tags_element = tags
        .iter()
        .enumerate()
        .map(|(index, tag)| {
            let delimiter = if tags.len() - index > 1 {
                html! { <span>{ " & " }</span> }
            } else {
                html! {}
            };
            html! {
                <span key={ tag.id }>
                    <a href={ format!("/tag/{}", tag.id) } target="_blank">{ tag.name.clone() }</a>
                    { delimiter }
                </span>
            }
        })
        .collect::<Html>();

    let series_element = if let Some(series) = &metadata.series {
        html! {
            <a href={ format!("/series/{}", series.id) } target="_blank">{ series.name.clone() }</a>
        }
    } else {
        html! {}
    };

    html! {
        <>
            <h3>{ metadata.book.title.clone() }</h3>
            <div class="book-cover">
                <img class="detail-cover" src={ get_cover_image_url(&book.small_cover) } alt={book.title.clone()} />
            </div>
            <div class="book-authors">{ authors_element }</div>
            <div class="book-publishers">{ publisher_element }</div>
            <div class="book-published-date">{ published_date_element }</div>
            <div class="book-tags">{ tags_element }</div>
            <div class="book-series">{ series_element }</div>
        </>
    }
}

impl Component for BookDetailComponent {
    type Message = Msg;
    type Properties = Prop;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { metadata: None }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                let book_id = ctx.props().book_id;
                ctx.link().send_future(async move {
                    match fetch_book_metadata(book_id).await {
                        Ok(obj) => Msg::FetchSuccess(obj),
                        Err(err) => Msg::FetchFailed(err),
                    }
                });
                false
            }
            Msg::FetchSuccess(metadata) => {
                log::info!("metadata: {:#?}", metadata);
                self.metadata = Some(metadata);
                true
            }
            Msg::FetchFailed(err) => {
                log::warn!("failed to fetch something: {:?}", err);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let fetch = ctx.link().callback(|_| Msg::Fetch);

        let book_detail = match &self.metadata {
            Some(metadata) => generate_metadata_element(metadata),
            None => html! { <></> },
        };

        html! {
            <>
                <h2>{ "Book Details" }</h2>
                <button onclick={fetch}>{"Fetch metadata"}</button>

                { book_detail }
            </>
        }
    }
}
