table! {
    authors (id) {
        id -> Int4,
        name -> Text,
        sort -> Text,
        timestamp -> Timestamp,
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
        timestamp -> Timestamp,
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
    comments (id) {
        id -> Int4,
        book -> Int4,
        text -> Text,
        timestamp -> Timestamp,
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
        timestamp -> Timestamp,
        last_modified -> Timestamp,
    }
}

table! {
    identifiers (id) {
        id -> Int4,
        book -> Int4,
        scheme -> Text,
        value -> Text,
        timestamp -> Timestamp,
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
        timestamp -> Timestamp,
        last_modified -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors_link,
    books_languages_link,
    comments,
    data,
    identifiers,
    languages,
    publishers,
);
