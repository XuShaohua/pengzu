// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{
    authors::AuthorsComponent, books::BooksComponent, categories::CategoriesComponent,
    discover::DiscoverComponent, file_formats::FileFormatsComponent, not_found::NotFoundComponent,
    publishers::PublishersComponent, ratings::RatingsComponent, series::SeriesComponent,
    tags::TagsComponent,
};

#[derive(Debug, Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/book")]
    Book,
    #[at("/author")]
    Author,
    #[at("/category")]
    Category,
    #[at("/tag")]
    Tag,
    #[at("/publisher")]
    Publisher,
    #[at("/series")]
    Series,
    #[at("/discover")]
    Discover,
    #[at("/format")]
    FileFormat,
    #[at("/rating")]
    Rating,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_route(routes: &Route) -> Html {
    match routes {
        Route::Book | Route::Home => html! { <BooksComponent /> },
        Route::Author => html! { <AuthorsComponent /> },
        Route::Category => html! { <CategoriesComponent /> },
        Route::Tag => html! { <TagsComponent /> },
        Route::Publisher => html! { <PublishersComponent /> },
        Route::Series => html! { <SeriesComponent /> },
        Route::Discover => html! { <DiscoverComponent /> },
        Route::FileFormat => html! { <FileFormatsComponent /> },
        Route::Rating => html! { <RatingsComponent /> },
        Route::NotFound => html! { <NotFoundComponent /> },
    }
}
