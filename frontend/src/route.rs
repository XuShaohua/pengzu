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
    #[at("/books")]
    Books,
    #[at("/authors")]
    Authors,
    #[at("/categories")]
    Categories,
    #[at("/tags")]
    Tags,
    #[at("/publishers")]
    Publishers,
    #[at("/series")]
    Series,
    #[at("/discover")]
    Discover,
    #[at("/file-formats")]
    FileFormats,
    #[at("/ratings")]
    Ratings,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch_route(routes: &Route) -> Html {
    match routes {
        Route::Books | Route::Home => html! { <BooksComponent /> },
        Route::Authors => html! { <AuthorsComponent /> },
        Route::Categories => html! { <CategoriesComponent /> },
        Route::Tags => html! { <TagsComponent /> },
        Route::Publishers => html! { <PublishersComponent /> },
        Route::Series => html! { <SeriesComponent /> },
        Route::Discover => html! { <DiscoverComponent /> },
        Route::FileFormats => html! { <FileFormatsComponent /> },
        Route::Ratings => html! { <RatingsComponent /> },
        Route::NotFound => html! { <NotFoundComponent /> },
    }
}
