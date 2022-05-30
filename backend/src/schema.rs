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

allow_tables_to_appear_in_same_query!(
    authors,
    books,
);
