// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::not_found::NotFoundComponent;
use crate::types::advanced_search::AdvancedSearchQuery;
use crate::views::advanced_search::AdvancedSearchComponent;
use crate::views::authors::AuthorsComponent;
use crate::views::book_detail::BookDetailComponent;
use crate::views::books::BooksComponent;
use crate::views::books_of_advanced_search::BooksOfAdvancedSearchComponent;
use crate::views::books_of_author::BooksOfAuthorComponent;
use crate::views::books_of_category::BooksOfCategoryComponent;
use crate::views::books_of_file_format::BooksOfFileFormatComponent;
use crate::views::books_of_publisher::BooksOfPublisherComponent;
use crate::views::books_of_series::BooksOfSeriesComponent;
use crate::views::books_of_simple_search::BooksOfSimpleSearchComponent;
use crate::views::books_of_tag::BooksOfTagComponent;
use crate::views::books_of_user_tag::BooksOfUserTagComponent;
use crate::views::categories::CategoriesComponent;
use crate::views::discover::DiscoverComponent;
use crate::views::file_formats::FileFormatsComponent;
use crate::views::login::LoginComponent;
use crate::views::logout::LogoutComponent;
use crate::views::publishers::PublishersComponent;
use crate::views::ratings::RatingsComponent;
use crate::views::series::SeriesComponent;
use crate::views::tags::TagsComponent;
use crate::views::user_info::UserInfoComponent;
use crate::views::user_tags::UserTagsComponent;
use crate::views::users::UsersComponent;

#[derive(Debug, Clone, PartialEq, Eq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/book/:book_id")]
    BookDetail { book_id: i32 },
    #[at("/book")]
    Book,
    #[at("/author/books/:author_id")]
    BooksOfAuthor { author_id: i32 },
    #[at("/author")]
    Author,
    #[at("/category/books/:category_id")]
    BooksOfCategory { category_id: i32 },
    #[at("/category")]
    Category,
    #[at("/tag/books/:tag_id")]
    BooksOfTag { tag_id: i32 },
    #[at("/tag")]
    Tag,
    #[at("/publisher/books/:publisher_id")]
    BooksOfPublisher { publisher_id: i32 },
    #[at("/publisher")]
    Publisher,
    #[at("/series/books/:series_id")]
    BooksOfSeries { series_id: i32 },
    #[at("/series")]
    Series,
    #[at("/discover")]
    Discover,
    #[at("/format/books/:format_id")]
    BooksOfFileFormat { format_id: i32 },
    #[at("/format")]
    FileFormat,
    #[at("/rating")]
    Rating,

    #[at("/user-tag/books/:tag_id")]
    BooksOfUserTag { tag_id: i32 },
    #[at("/user-tag")]
    UserTag,

    #[at("/advanced-search/books/:page_id/:query")]
    BooksOfAdvancedSearch { query: String, page_id: i32 },
    #[at("/advanced-search")]
    AdvancedSearch,
    #[at("/search/books")]
    BooksOfSimpleSearch,

    #[at("/user-info")]
    UserInfo,
    #[at("/users")]
    Users,
    #[at("/login")]
    Login,
    #[at("/logout")]
    Logout,

    #[not_found]
    #[at("/404")]
    NotFound,
}

#[must_use]
#[allow(clippy::cognitive_complexity)]
#[allow(clippy::let_unit_value)]
pub fn switch_route(routes: &Route) -> Html {
    match routes {
        Route::BookDetail { book_id } => html! { <BookDetailComponent book_id={ * book_id }/> },
        Route::Book | Route::Home => html! { <BooksComponent /> },
        Route::BooksOfAuthor { author_id } => {
            html! { <BooksOfAuthorComponent author_id={ *author_id }/> }
        }
        Route::Author => html! { <AuthorsComponent /> },
        Route::BooksOfCategory { category_id } => {
            html! { <BooksOfCategoryComponent category_id={ *category_id }/> }
        }
        Route::Category => html! { <CategoriesComponent /> },
        Route::BooksOfTag { tag_id } => html! { <BooksOfTagComponent tag_id={ *tag_id }/>},
        Route::Tag => html! { <TagsComponent /> },
        Route::BooksOfPublisher { publisher_id } => {
            html! { <BooksOfPublisherComponent publisher_id={ *publisher_id }/> }
        }
        Route::Publisher => html! { <PublishersComponent /> },
        Route::BooksOfSeries { series_id } => {
            html! { <BooksOfSeriesComponent series_id={ *series_id }/> }
        }
        Route::Series => html! { <SeriesComponent /> },
        Route::Discover => html! { <DiscoverComponent /> },
        Route::BooksOfFileFormat { format_id } => {
            html! { <BooksOfFileFormatComponent format_id={ *format_id }/> }
        }
        Route::FileFormat => html! { <FileFormatsComponent /> },
        Route::Rating => html! { <RatingsComponent /> },

        Route::BooksOfUserTag { tag_id } => html! { <BooksOfUserTagComponent tag_id={ *tag_id }/> },
        Route::UserTag => html! { <UserTagsComponent /> },

        Route::BooksOfAdvancedSearch { query, page_id } => {
            if let Ok(query_obj) = serde_json::from_str::<AdvancedSearchQuery>(query) {
                html! { <BooksOfAdvancedSearchComponent query={ query_obj } page_id={ *page_id } /> }
            } else {
                // TODO(Shaohua): Remove
                html! { <h1>{"Invalid query" }</h1> }
            }
        }
        Route::AdvancedSearch => html! { <AdvancedSearchComponent /> },
        Route::BooksOfSimpleSearch => {
            html! { <BooksOfSimpleSearchComponent /> }
        }

        Route::UserInfo => html! { <UserInfoComponent /> },
        Route::Users => html! { <UsersComponent /> },
        Route::Login => html! { <LoginComponent /> },
        Route::Logout => html! { <LogoutComponent /> },

        Route::NotFound => html! { <NotFoundComponent /> },
    }
}
