table! {
    authors (id) {
        id -> Int4,
        name -> Text,
        sort -> Text,
        link -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    books (id) {
        id -> Int4,
        title -> Text,
        sort -> Text,
        author_sort -> Text,
        path -> Text,
        uuid -> Text,
        has_cover -> Bool,
        created -> Timestamp,
        pubdate -> Timestamp,
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
    books_tags_link (id) {
        id -> Int4,
        book -> Int4,
        tag -> Int4,
        created -> Timestamp,
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
        uncompressed_size -> Int4,
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
        status -> Int4,
        book -> Nullable<Int4>,
        created -> Timestamp,
    }
}

table! {
    import_libraries (id) {
        id -> Int4,
        calibre_path -> Text,
        total -> Int4,
        finished -> Bool,
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
        sort -> Text,
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
    tags (id) {
        id -> Int4,
        name -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors_link,
    books_languages_link,
    books_publishers_link,
    books_tags_link,
    comments,
    file_formats,
    files,
    identifier_types,
    identifiers,
    import_books,
    import_libraries,
    languages,
    publishers,
    ratings,
    tags,
);
