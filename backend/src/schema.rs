table! {
    authors (id) {
        id -> Int4,
        name -> Text,
        sort -> Text,
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
    }
}

table! {
    books_languages_link (id) {
        id -> Int4,
        book -> Int4,
        lang_code -> Int4,
    }
}

table! {
    books_publishers_link (id) {
        id -> Int4,
        book -> Int4,
        publisher -> Int4,
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
    data (id) {
        id -> Int4,
        book -> Int4,
        format -> Text,
        uncompressed_size -> Int4,
        name -> Text,
        sha -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    identifiers (id) {
        id -> Int4,
        book -> Int4,
        scheme -> Text,
        value -> Text,
        created -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    languages (id) {
        id -> Int4,
        lang_code -> Text,
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
    data,
    identifiers,
    languages,
    publishers,
    ratings,
    tags,
);
