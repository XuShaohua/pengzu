// Copyright (c) 2023 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::{function_component, html, AttrValue, Html, Properties};
use zu_util::name;

use crate::services::images::get_cover_url;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub url: AttrValue,

    #[prop_or_default]
    pub title: AttrValue,
}

#[function_component(BookCover)]
pub fn book_cover(props: &Props) -> Html {
    if props.url.is_empty() {
        return default_book_cover(props);
    }

    match get_cover_url(props.url.as_ref()) {
        Ok(cover_url) => {
            html! {
              <img class="book-cover-img"
                src={ cover_url } alt={ &props.title }
                width="135" height="200" />
            }
        }
        Err(err) => {
            log::warn!("{err:?}");
            default_book_cover(props)
        }
    }
}

fn default_book_cover(props: &Props) -> Html {
    let color = name::to_color(&props.title);
    let style = format!("background-color: {color}; border-color: {color};");
    html! {
        <span class="book-cover-img__default" style={style}>
        </span>
    }
}
