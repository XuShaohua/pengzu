// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::authors::Author;
use shared::books_meta::BookMetadata;
use shared::categories::Category;
use shared::files::FileWithPath;
use shared::identifiers::IdentifierWithType;
use shared::publishers::Publisher;
use shared::tags::Tag;
use shared::user_tags::UserTag;
use yew::prelude::*;
use yew_router::prelude::Link;

use super::navigation::NavigationComponent;
use crate::router::Route;
use crate::services::files::get_file_format_url;
use crate::services::images::get_cover_image_url;
use crate::views::util::to_readable_size;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub metadata: BookMetadata,
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

fn generate_formats_element(files: &[FileWithPath]) -> Html {
    files
        .iter()
        .map(|file| {
            let url = get_file_format_url(file);
            let readable_size = to_readable_size(file.size);
            html! {
                <li>
                    <a class="book-format" target="_blank" href={ url }>
                        { format!("{} ({readable_size})", file.format_name) }
                        <i class="bi bi-download ms-1"></i>
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
                <Link<Route> to={ Route::BooksOfPublisher { publisher_id: publisher.id }}
                    classes="col-sm-10">
                    { &publisher.name }
                </Link<Route>>
            }
        },
    )
}

fn generate_identifiers_element(identifiers: &[IdentifierWithType]) -> Html {
    identifiers
        .iter()
        .enumerate()
        .map(|(index, identifier)| {
            let delimiter = if identifiers.len() - index > 1 {
                html! { <span>{ " & " }</span> }
            } else {
                html! {}
            };
            html! {
                <span key={ identifier.id }>
                    <span title={ identifier.name.clone() }>
                        { &identifier.value }
                    </span>
                    { delimiter }
                </span>
            }
        })
        .collect::<Html>()
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

#[function_component(MetadataComponent)]
pub fn metadata_page(props: &Props) -> Html {
    let metadata = &props.metadata;
    let book = &metadata.book;

    let authors_element = generate_author_element(&metadata.authors);
    let publisher_element = generate_publisher_element(&metadata.publisher);
    let tags_element = generate_tags_element(&metadata.tags);
    let user_tags_element = generate_user_tags_element(&metadata.user_tags);
    let formats_element = generate_formats_element(&metadata.files);
    let categories_element = generate_categories_element(&metadata.categories);
    let identifiers_element = generate_identifiers_element(&metadata.identifiers);

    let published_date = book.pubdate.as_ref().map_or_else(String::new, |pubdate| {
        pubdate.date().format("%Y-%m-%d").to_string()
    });

    let series_element = metadata.series.as_ref().map_or_else(|| html!{},
                                                              |series|
                                                                  html! {
            <Link<Route> to={ Route::BooksOfSeries { series_id: series.id } }>{ &series.name }</Link<Route>>
        });

    let cover_url = get_cover_image_url(&book.small_cover);

    html! {
        <div class="container">

            <h2>{ &metadata.book.title }</h2>
            <div class="mt-2 mb-2">
                <img class="" src={ cover_url } alt={ book.title.clone() } />
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
                { identifiers_element }
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
                <ol class="book-formats ms-3">
                    { formats_element }
                </ol>
            </div>

            <NavigationComponent previous_book={ metadata.previous_book } next_book={ metadata.next_book } />
        </div>
    }
}
