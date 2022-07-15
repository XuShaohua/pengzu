-- This file should undo anything in `up.sql`

DROP INDEX books_title_idx;
DROP INDEX books_authors_link_author_idx;
DROP INDEX books_authors_link_book_idx;
DROP INDEX books_categories_link_book_idx;
DROP INDEX books_categories_link_category_idx;
DROP INDEX books_languages_link_lang_idx;
DROP INDEX books_languages_link_book_idx;
DROP INDEX books_publishers_link_publisher_idx;
DROP INDEX books_publishers_link_book_idx;
DROP INDEX books_series_link_series_idx;
DROP INDEX books_series_link_book_idx;
DROP INDEX books_tags_link_tag_idx;
DROP INDEX books_tags_link_book_idx;
DROP INDEX comments_book_idx;
DROP INDEX categories_order_index_idx;
DROP INDEX categories_serial_number_idx;
DROP INDEX categories_parent_idx;
DROP INDEX files_book_idx;
DROP INDEX files_format_idx;
DROP INDEX languages_lang_idx;
DROP INDEX publishers_name_idx;
DROP INDEX ratings_rating_idx;
DROP INDEX ratings_book_idx;
DROP INDEX series_name_idx;
DROP INDEX tags_name_idx;
