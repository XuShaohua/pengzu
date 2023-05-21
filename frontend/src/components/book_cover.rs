// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html, Properties};

use crate::services::images::get_cover_image_url;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub url: Option<String>,

    #[prop_or_default]
    pub title: AttrValue,
}

#[function_component(BookCover)]
pub fn book_cover(props: &Props) -> Html {
    let cover_url = get_cover_image_url(&props.url);

    html! {
      <img class="book-cover-img"
        src={ cover_url } alt={ &props.title }
        width="135" height="200" />
    }
}
