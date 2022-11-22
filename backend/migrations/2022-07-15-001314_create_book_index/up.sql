-- Your SQL goes here

CREATE INDEX books_title_idx ON books (title);
CREATE INDEX books_authors_link_author_idx ON books_authors_link (author);
CREATE INDEX books_authors_link_book_idx ON books_authors_link (book);
CREATE INDEX books_categories_link_book_idx ON books_categories_link (book);
CREATE INDEX books_categories_link_category_idx ON books_categories_link (category);
CREATE INDEX books_languages_link_lang_idx ON books_languages_link (language);
CREATE INDEX books_languages_link_book_idx ON books_languages_link (book);
CREATE INDEX books_publishers_link_publisher_idx ON books_publishers_link (publisher);
CREATE INDEX books_publishers_link_book_idx ON books_publishers_link (book);
CREATE INDEX books_series_link_series_idx ON books_series_link (series);
CREATE INDEX books_series_link_book_idx ON books_series_link (book);
CREATE INDEX books_tags_link_tag_idx ON books_tags_link (tag);
CREATE INDEX books_tags_link_book_idx ON books_tags_link (book);
CREATE INDEX comments_book_idx ON comments (book);
CREATE INDEX categories_order_index_idx ON categories (order_index);
CREATE INDEX categories_serial_number_idx ON categories (serial_number);
CREATE INDEX categories_parent_idx ON categories (parent);
CREATE INDEX files_book_idx ON files (book);
CREATE INDEX files_format_idx ON files (format);
CREATE INDEX languages_lang_idx ON languages (lang_code);
CREATE INDEX publishers_name_idx ON publishers (name);
CREATE INDEX ratings_rating_idx ON ratings (rating);
CREATE INDEX ratings_book_idx ON ratings (book);
CREATE INDEX series_name_idx ON series (name);
CREATE INDEX tags_name_idx ON tags (name);