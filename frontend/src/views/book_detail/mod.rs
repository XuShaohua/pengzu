// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod edit_metadata;
mod navigation;

use shared::authors::Author;
use shared::books_meta::BookMetadata;
use shared::files::FileWithPath;
use shared::tags::Tag;
use stylist::Style;
use yew::prelude::*;
use yew_hooks::prelude::{use_async_with_options, UseAsyncOptions};
use yew_router::prelude::Link;

use crate::router::Route;
use crate::services::books_meta::fetch_book_metadata;
use crate::services::files::get_file_format_url;
use crate::services::images::get_cover_image_url;
use crate::views::util;
use crate::views::util::to_readable_size;
use edit_metadata::EditMetadataComponent;
use navigation::NavigationComponent;
use shared::publishers::Publisher;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
}

fn generate_author_element(authors: &[Author]) -> Html {
    authors
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
        .collect::<Html>()
}

fn generate_tags_element(tags: &[Tag]) -> Html {
    tags
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
        .collect::<Html>()
}

fn generate_formats_element(files: &[FileWithPath]) -> Html {
    files
        .iter()
        .map(|file| {
            let url = get_file_format_url(file);
            let readable_size = to_readable_size(file.size);
            html! {
                <li>
                    <a class="book-format" target="_blank" href={ url }>
                        <i class="bi bi-download"></i>
                        { format!("{} ({})", file.format_name, readable_size) }
                    </a>
                </li>
            }
        })
        .collect::<Html>()
}

fn generate_publisher_element(publisher: &Option<Publisher>) -> Html {
    publisher.as_ref().map_or_else(
        || html! {<></>},
        |publisher| {
            html! {
                <span>
                    { "Publisher: " }
                    <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id }}>
                        { &publisher.name }
                    </Link<Route>>
                </span>
            }
        },
    )
}

fn generate_metadata_element(metadata: &BookMetadata) -> Html {
    let book = &metadata.book;

    let authors_element = generate_author_element(&metadata.authors);
    let publisher_element = generate_publisher_element(&metadata.publisher);
    let tags_element = generate_tags_element(&metadata.tags);
    let formats_element = generate_formats_element(&metadata.files);

    let published_date_element = book.pubdate.as_ref().map_or_else(
        || html! {},
        |pubdate| {
            html! {
                <span>{ format!("Published At: {:?}", pubdate) }</span>
            }
        },
    );

    let series_element = metadata.series.as_ref().map_or_else(|| html!{},
        |series|
        html! {
            <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>{ &series.name }</Link<Route>>
        });

    let style_str = include_str!("book_detail.css");
    let style_cls = Style::new(style_str).expect("Invalid style file book_detail.css");
    let cover_url = get_cover_image_url(&book.small_cover);

    html! {
        <div class={ style_cls }>

            <h2>{ &metadata.book.title }</h2>
            <div class="book-cover">
                <img class="detail-cover" src={ cover_url } alt={ book.title.clone() } />
            </div>
            <div class="book-authors">{ authors_element }</div>
            <div class="book-publishers">{ publisher_element }</div>
            <div class="book-published-date">{ published_date_element }</div>
            <div class="book-tags">{ tags_element }</div>
            <div class="book-series">{ series_element }</div>
            <ul class="book-formats">{ formats_element }</ul>

            <NavigationComponent previous_book={ metadata.previous_book } next_book={ metadata.next_book } />

            <EditMetadataComponent book_id={ book.id } title={ book.title.clone() } />
        </div>
    }
}

#[function_component(BookDetailComponent)]
pub fn book_detail(props: &Props) -> Html {
    util::set_document_title(&format!("Book: {}", props.book_id));

    let book_id = props.book_id;
    let book_metadata = use_async_with_options(
        async move { fetch_book_metadata(book_id).await },
        UseAsyncOptions::enable_auto(),
    );

    book_metadata.data.as_ref().map_or_else(
        || html! {},
        |book_metadata| {
            util::set_document_title(&format!("Book: {}", book_metadata.book.title));

            generate_metadata_element(book_metadata)
        },
    )
}
