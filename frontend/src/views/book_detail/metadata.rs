// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::authors::Author;
use shared::books_meta::BookMetadata;
use shared::categories::Category;
use shared::publishers::Publisher;
use shared::tags::Tag;
use shared::user_tags::UserTag;
use yew::prelude::*;
use yew_router::prelude::Link;

use super::book_formats::BookFormatsComponent;
use super::book_identifiers::BookIdentifiersComponent;
use super::navigation::NavigationComponent;
use crate::components::book_cover::BookCover;
use crate::router::Route;

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

fn generate_user_tags_element(tags: &[UserTag]) -> Html {
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
                    <Link<Route> to={ Route::BooksOfUserTag { tag_id: tag.id } }>{ &tag.name } </Link<Route>>
                    { delimiter }
                </span>
            }
        })
        .collect::<Html>()
}

fn generate_publisher_element(publisher: &Option<Publisher>) -> Html {
    publisher.as_ref().map_or_else(
        || html! {<></>},
        |publisher| {
            html! {
                <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id }}
                    classes="col-sm-10">
                    { &publisher.name }
                </Link<Route>>
            }
        },
    )
}

fn generate_categories_element(categories: &[Category]) -> Html {
    categories
        .iter()
        .enumerate()
        .map(|(index, category)| {
            let delimiter = if categories.len() - index > 1 {
                html! { <span>{ " & " }</span> }
            } else {
                html! {}
            };
            html! {
                <span key={ category.id }>
                    <Link<Route> to={ Route::BooksOfCategory { category_id: category.id } }>{ format!("[{}] {}", category.serial_number, category.name) } </Link<Route>>
                    { delimiter }
                </span>
            }
        })
        .collect::<Html>()
}

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    pub metadata: BookMetadata,
}

#[function_component(MetadataComponent)]
pub fn metadata_page(props: &Props) -> Html {
    let metadata = &props.metadata;
    let book = &metadata.book;

    let authors_element = generate_author_element(&metadata.authors);
    let publisher_element = generate_publisher_element(&metadata.publisher);
    let tags_element = generate_tags_element(&metadata.tags);
    let user_tags_element = generate_user_tags_element(&metadata.user_tags);
    let categories_element = generate_categories_element(&metadata.categories);

    let published_date = book.pubdate.as_ref().map_or_else(String::new, |pubdate| {
        pubdate.date().format("%Y-%m-%d").to_string()
    });

    let series_element = metadata.series.as_ref().map_or_else(|| html!{},
                                                              |series|
                                                                  html! {
            <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>{ &series.name }</Link<Route>>
        });

    html! {
        <div class="container">

            <h2>{ &metadata.book.title }</h2>
            <div class="mt-2 mb-2">
                <BookCover url={book.small_cover.clone().unwrap_or_default()} title={ book.title.clone() } />
            </div>

            <div>
                <span class="me-2 text-align-end fw-bold">{ "Authors:" }</span>
                { authors_element }
            </div>

            <div>
                <span class="me-2 fw-bold">{ "Tags:" }</span>
                { tags_element }
            </div>

            <div>
                <span class="me-2 fw-bold">{ "UserTags:" }</span>
                { user_tags_element }
            </div>

            <div>
                <span class="me-2 fw-bold">{ "Categories:" }</span>
                { categories_element }
            </div>

            <div>
                <span class="me-2 fw-bold">{ "Identifiers:" }</span>
                <BookIdentifiersComponent identifiers={ metadata.identifiers.clone() } />
            </div>

            <div>
                <span class="me-2 fw-bold">{ "Publisher:" }</span>
                { publisher_element }
            </div>

            <div>
                <span class="me-2 fw-bold">{ "Published At:" }</span>
                { published_date }
            </div>

            <div>
                <span class="me-2 fw-bold">{ "Series:" }</span>
                { series_element }
            </div>

            <div class="mt-2">
                <span class="d-block me-2 fw-bold">{ "File Formats" }</span>
                <BookFormatsComponent files={ metadata.files.clone() } />
            </div>

            <NavigationComponent previous_book={ metadata.previous_book } next_book={ metadata.next_book } />
        </div>
    }
}
