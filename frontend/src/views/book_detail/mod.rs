// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

mod edit_authors;
mod edit_metadata;
mod edit_publisher;
mod edit_tags;
mod edit_title;
mod metadata;
mod navigation;
mod search_tag;
mod edit_user_tags;

use yew::prelude::*;
use yew_hooks::use_async;

use self::edit_metadata::EditMetadataComponent;
use self::metadata::MetadataComponent;
use crate::hooks::use_user_context;
use crate::router::Route;
use crate::services::books_meta::fetch_book_metadata;
use crate::views::util;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub book_id: i32,
}

#[function_component(BookDetailComponent)]
pub fn book_detail(props: &Props) -> Html {
    util::set_document_title(&format!("Book: {}", props.book_id));

    let user_ctx = use_user_context();
    let is_admin = user_ctx.is_admin();

    let book_metadata = {
        let book_id = props.book_id;
        use_async(async move { fetch_book_metadata(book_id).await })
    };
    {
        let book_metadata_clone = book_metadata.clone();
        use_effect_with_deps(
            move |_book_id| {
                book_metadata_clone.run();
                || ()
            },
            props.book_id,
        );
    }

    book_metadata.data.as_ref().map_or_else(
        || html! {},
        |book_metadata| {
            util::set_document_title(&format!("Book: {}", book_metadata.book.title));

            if is_admin {
                html! {
                    <EditMetadataComponent metadata={ book_metadata.clone() } />
                }
            } else {
                html! {
                    <MetadataComponent metadata={ book_metadata.clone() } />
                }
            }
        },
    )
}
