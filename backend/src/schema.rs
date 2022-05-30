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
