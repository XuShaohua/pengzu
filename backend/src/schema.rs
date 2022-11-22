// @generated automatically by Diesel CLI.

diesel::table! {
    authors (id) {
        id -> Int4,
        name -> Text,
        link -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Text,
        path -> Text,
        author_sort -> Text,
        uuid -> Text,
        has_cover -> Bool,
        pubdate -> Nullable<Timestamp>,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    books_authors_link (id) {
        id -> Int4,
        book -> Int4,
        author -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    books_categories_link (id) {
        id -> Int4,
        book -> Int4,
        category -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    books_languages_link (id) {
        id -> Int4,
        book -> Int4,
        language -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    books_publishers_link (id) {
        id -> Int4,
        book -> Int4,
        publisher -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    books_series_link (id) {
        id -> Int4,
        book -> Int4,
        series -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    books_tags_link (id) {
        id -> Int4,
        book -> Int4,
        tag -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    books_user_tags_link (id) {
        id -> Int4,
        user_id -> Int4,
        book -> Int4,
        tag -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Int4,
        order_index -> Int4,
        serial_number -> Text,
        name -> Text,
        url -> Text,
        description -> Nullable<Text>,
        parent -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    comments (id) {
        id -> Int4,
        book -> Int4,
        text -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    download_history (id) {
        id -> Int4,
        user_id -> Int4,
        book -> Int4,
        file -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    file_formats (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    files (id) {
        id -> Int4,
        book -> Int4,
        format -> Int4,
        size -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    identifier_types (id) {
        id -> Int4,
        name -> Text,
        url_template -> Text,
        description -> Nullable<Text>,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    identifiers (id) {
        id -> Int4,
        book -> Int4,
        scheme -> Int4,
        value -> Text,
        url -> Nullable<Text>,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    import_books (id) {
        id -> Int4,
        library -> Int4,
        calibre_book -> Int4,
        ok -> Bool,
        book -> Nullable<Int4>,
        created -> Timestamp,
    }
}

diesel::table! {
    import_libraries (id) {
        id -> Int4,
        calibre_library_path -> Text,
        library_path -> Text,
        total -> Int4,
        finished -> Bool,
        options -> Text,
        worker_pid -> Nullable<Int4>,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    languages (id) {
        id -> Int4,
        lang_code -> Text,
        created -> Timestamp,
    }
}

diesel::table! {
    publishers (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    ratings (id) {
        id -> Int4,
        book -> Int4,
        rating -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    reading_history (id) {
        id -> Int4,
        user_id -> Int4,
        book -> Int4,
        page -> Int4,
        percent -> Int4,
        created -> Timestamp,
        updated -> Timestamp,
    }
}

diesel::table! {
    series (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        order_index -> Int4,
        name -> Text,
        parent -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    user_tags (id) {
        id -> Int4,
        user_id -> Int4,
        order_index -> Int4,
        name -> Text,
        parent -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        display_name -> Text,
        email -> Text,
        role -> Int4,
        salt -> Text,
        hash -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors_link,
    books_categories_link,
    books_languages_link,
    books_publishers_link,
    books_series_link,
    books_tags_link,
    books_user_tags_link,
    categories,
    comments,
    download_history,
    file_formats,
    files,
    identifier_types,
    identifiers,
    import_books,
    import_libraries,
    languages,
    publishers,
    ratings,
    reading_history,
    series,
    tags,
    user_tags,
    users,
);
