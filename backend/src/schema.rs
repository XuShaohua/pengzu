table! {
    books (id) {
        id -> Int4,
        title -> Text,
        sort -> Nullable<Text>,
        author_sort -> Nullable<Text>,
        path -> Text,
        uuid -> Nullable<Text>,
        has_cover -> Nullable<Bool>,
        timestamp -> Nullable<Timestamp>,
        pubdate -> Nullable<Timestamp>,
        last_modified -> Timestamp,
    }
}
