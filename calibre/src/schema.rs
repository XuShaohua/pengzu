table! {
    authors (id) {
        id -> Nullable<Integer>,
        name -> Text,
        sort -> Nullable<Text>,
        link -> Text,
    }
}

table! {
    books (id) {
        id -> Nullable<Integer>,
        title -> Text,
        sort -> Nullable<Text>,
        timestamp -> Nullable<Timestamp>,
        pubdate -> Nullable<Timestamp>,
        series_index -> Float,
        author_sort -> Nullable<Text>,
        isbn -> Nullable<Text>,
        lccn -> Nullable<Text>,
        path -> Text,
        flags -> Integer,
        uuid -> Nullable<Text>,
        last_modified -> Timestamp,
    }
}

table! {
    books_authors_link (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        author -> Integer,
    }
}

table! {
    books_languages_link (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        lang_code -> Integer,
        item_order -> Integer,
    }
}

table! {
    books_publishers_link (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        publisher -> Integer,
    }
}

table! {
    books_ratings_link (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        rating -> Integer,
    }
}

table! {
    books_series_link (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        series -> Integer,
    }
}

table! {
    books_tags_link (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        tag -> Integer,
    }
}

table! {
    comments (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        text -> Text,
    }
}

table! {
    data (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        format -> Text,
        uncompressed_size -> Integer,
        name -> Text,
    }
}

table! {
    identifiers (id) {
        id -> Nullable<Integer>,
        book -> Integer,
        #[sql_name = "type"]
        type_ -> Text,
        val -> Text,
    }
}

table! {
    languages (id) {
        id -> Nullable<Integer>,
        lang_code -> Text,
    }
}

table! {
    publishers (id) {
        id -> Nullable<Integer>,
        name -> Text,
        sort -> Nullable<Text>,
    }
}

table! {
    ratings (id) {
        id -> Nullable<Integer>,
        rating -> Nullable<Integer>,
    }
}

table! {
    series (id) {
        id -> Nullable<Integer>,
        name -> Text,
        sort -> Nullable<Text>,
    }
}

table! {
    tags (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    authors,
    books,
    books_authors_link,
    books_languages_link,
    books_publishers_link,
    books_ratings_link,
    books_series_link,
    books_tags_link,
    comments,
    data,
    identifiers,
    languages,
    publishers,
    ratings,
    series,
    tags,
);
