table! {
    authors (id) {
        id -> Int4,
        name -> Text,
        link -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    books (id) {
        id -> Int4,
        title -> Text,
        path -> Text,
        uuid -> Text,
        has_cover -> Bool,
        pubdate -> Nullable<Timestamp>,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    books_authors_link (id) {
        id -> Int4,
        book -> Int4,
        author -> Int4,
        created -> Timestamp,
    }
}

table! {
    books_categories_link (id) {
        id -> Int4,
        book -> Int4,
        category -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    books_languages_link (id) {
        id -> Int4,
        book -> Int4,
        lang_code -> Int4,
        created -> Timestamp,
    }
}

table! {
    books_publishers_link (id) {
        id -> Int4,
        book -> Int4,
        publisher -> Int4,
        created -> Timestamp,
    }
}

table! {
    books_series_link (id) {
        id -> Int4,
        book -> Int4,
        series -> Int4,
        created -> Timestamp,
    }
}

table! {
    books_tags_link (id) {
        id -> Int4,
        book -> Int4,
        tag -> Int4,
        created -> Timestamp,
    }
}

table! {
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

table! {
    cips (id) {
        id -> Int4,
        cip -> Int4,
        isbn13 -> Int4,
        title -> Text,
        original_title -> Nullable<Text>,
        category_id -> Text,
        publisher -> Int4,
        pubdate -> Text,
        price -> Nullable<Text>,
        intro -> Nullable<Text>,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    cips_authors_link (id) {
        id -> Int4,
        cip -> Int4,
        author -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Int4,
        book -> Int4,
        text -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    douban_authors (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    douban_books (id) {
        id -> Int4,
        subject_id -> Int4,
        title -> Text,
        url -> Text,
        small_cover -> Text,
        isbn -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    douban_books_authors_link (id) {
        id -> Int4,
        book -> Int4,
        author -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    douban_books_detail (id) {
        id -> Int4,
        book_id -> Int4,
        original_title -> Nullable<Text>,
        large_cover -> Text,
        toc -> Nullable<Text>,
        intro -> Nullable<Text>,
        publisher -> Nullable<Int4>,
        rating_number -> Nullable<Float4>,
        rating_people -> Nullable<Int4>,
        price -> Nullable<Text>,
        page -> Nullable<Int4>,
        pubdate -> Timestamp,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    douban_books_recommends_link (id) {
        id -> Int4,
        book -> Int4,
        recommend_book -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    douban_publishers (id) {
        id -> Int4,
        name -> Text,
        url -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    file_formats (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    files (id) {
        id -> Int4,
        book -> Int4,
        format -> Int4,
        size -> Int4,
        name -> Text,
        sha -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    identifier_types (id) {
        id -> Int4,
        name -> Text,
        url_template -> Text,
        description -> Nullable<Text>,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
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

table! {
    import_books (id) {
        id -> Int4,
        library -> Int4,
        calibre_book -> Int4,
        ok -> Bool,
        book -> Nullable<Int4>,
        created -> Timestamp,
    }
}

table! {
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

table! {
    languages (id) {
        id -> Int4,
        lang_code -> Text,
        created -> Timestamp,
    }
}

table! {
    publishers (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    ratings (id) {
        id -> Int4,
        book -> Int4,
        rating -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    series (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    tags (id) {
        id -> Int4,
        order_index -> Int4,
        name -> Text,
        parent -> Int4,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
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

table! {
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

allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors_link,
    books_categories_link,
    books_languages_link,
    books_publishers_link,
    books_series_link,
    books_tags_link,
    categories,
    cips,
    cips_authors_link,
    comments,
    douban_authors,
    douban_books,
    douban_books_authors_link,
    douban_books_detail,
    douban_books_recommends_link,
    douban_publishers,
    file_formats,
    files,
    identifier_types,
    identifiers,
    import_books,
    import_libraries,
    languages,
    publishers,
    ratings,
    series,
    tags,
    user_tags,
    users,
);
