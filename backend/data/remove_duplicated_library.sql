
-- First find library_id from `import_library` table.
-- Never remove records from `import_books` table.

-- Cleanup duplicated library
DELETE FROM books
WHERE id IN (
    SELECT book FROM import_books WHERE library = 2
);

DELETE FROM books_authors_link
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
    );

DELETE FROM books_categories_link
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
);

DELETE FROM books_languages_link
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
);

DELETE FROM books_publishers_link
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
);

DELETE FROM books_series_link
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
);

DELETE FROM books_tags_link
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
);

DELETE FROM books_user_tags_link
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
);

DELETE FROM comments
WHERE book IN (
    SELECT book FROM import_books WHERE library = 2
);