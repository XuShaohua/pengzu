// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;

use crate::route::Route;
use crate::services::books_meta::fetch_book_metadata;
use crate::types::books_meta::BookMetadata;
use crate::views::util::get_cover_image_url;

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub book_id: i32,
}

fn generate_metadata_element(metadata: &BookMetadata) -> Html {
    let book = &metadata.book;

    let authors = &metadata.authors;
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

    let publisher_element = match &metadata.publisher {
        Some(publisher) => {
            html! {
                <span>
                    { "Publisher: " }
                    <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id }}>
                        { &publisher.name }
                    </Link<Route>>
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
                    <Link<Route> to={ Route::BooksOfTag { tag_id: tag.id } }>{ &tag.name } </Link<Route>>
                    { delimiter }
                </span>
            }
        })
        .collect::<Html>();

    let series_element = if let Some(series) = &metadata.series {
        html! {
            <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>{ &series.name }</Link<Route>>
        }
    } else {
        html! {}
    };

    html! {
        <>
            <h3>{ &metadata.book.title }</h3>
            <div class="book-cover">
                <img class="detail-cover" src={ get_cover_image_url(&book.small_cover) } alt={ book.title.clone() } />
            </div>
            <div class="book-authors">{ authors_element }</div>
            <div class="book-publishers">{ publisher_element }</div>
            <div class="book-published-date">{ published_date_element }</div>
            <div class="book-tags">{ tags_element }</div>
            <div class="book-series">{ series_element }</div>
        </>
    }
}

#[function_component(BookDetailComponent)]
pub fn book_detail(props: &Props) -> Html {
    let book_metadata = {
        let book_id = props.book_id;
        use_async_with_options(
            async move { fetch_book_metadata(book_id).await },
            UseAsyncOptions::enable_auto(),
        )
    };

    if let Some(book_metadata) = &book_metadata.data {
        return generate_metadata_element(book_metadata);
    } else {
        return html! {};
    }
}
