// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use shared::books_meta::BookMetadata;
use yew::prelude::*;

use super::book_formats::BookFormatsComponent;
use super::edit_authors::EditAuthorsComponent;
use super::edit_identifiers::EditIdentifiersComponent;
use super::edit_publisher::EditPublisherComponent;
use super::edit_series::EditSeriesComponent;
use super::edit_tags::EditTagsComponent;
use super::edit_title::EditTitleComponent;
use super::edit_user_tags::EditUserTagsComponent;
use super::navigation::NavigationComponent;
use crate::services::images::get_cover_image_url;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub metadata: BookMetadata,
}

#[function_component(EditMetadataComponent)]
pub fn edit_metadata_page(props: &Props) -> Html {
    let metadata = &props.metadata;
    let book = &metadata.book;

    let cover_url = get_cover_image_url(&book.small_cover);

    html! {
        <div class="container">
            <h2>{ &metadata.book.title }</h2>
            <div class="mt-2 mb-2">
                <img class="" src={ cover_url } alt={ book.title.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "Title "}</h3>
                <EditTitleComponent book_id={ book.id } title={ book.title.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "Authors" }</h3>
                <EditAuthorsComponent book_id={ book.id } authors={ metadata.authors.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "Publisher" }</h3>
                <EditPublisherComponent book_id={ book.id } publisher={ metadata.publisher.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "Series" }</h3>
                <EditSeriesComponent book_id={ book.id } series={ metadata.series.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "Identifiers" }</h3>
                <EditIdentifiersComponent identifiers={ metadata.identifiers.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "Tags" }</h3>
                <EditTagsComponent book_id={ book.id } tags={ metadata.tags.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "User Tags" }</h3>
                <EditUserTagsComponent book_id={ book.id } tags={ metadata.user_tags.clone() } />
            </div>

            <div class="mb-2">
                <h3>{ "File Formats" }</h3>
                <BookFormatsComponent files={ metadata.files.clone() } />
            </div>

            <NavigationComponent previous_book={ metadata.previous_book } next_book={ metadata.next_book } />
        </div>
    }
}
