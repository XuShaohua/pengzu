-- Your SQL goes here

CREATE VIEW books_meta AS
SELECT id,
       title,
       ( SELECT CONCAT(bal.id, authors.name)
         FROM books_authors_link AS bal
                  INNER JOIN authors ON (bal.author = authors.id)
         WHERE book = books.id )                                                                                    authors,
       ( SELECT name
         FROM publishers
         WHERE publishers.id IN
               ( SELECT publisher FROM books_publishers_link WHERE book = books.id ) )                              publisher,
       ( SELECT rating FROM ratings WHERE book = ratings.book )                                                     rating,
       pubdate,
       ( SELECT MAX(size) FROM files WHERE book = books.id )                                                        size,
       ( SELECT CONCAT(name)
         FROM tags
         WHERE tags.id IN ( SELECT tag FROM books_tags_link WHERE book = books.id ) )                               tags,
       ( SELECT text FROM comments WHERE book = books.id )                                                          comments,
       ( SELECT name
         FROM series
         WHERE series.id IN
               ( SELECT series FROM books_series_link WHERE book = books.id ) )                                     series,
       ( SELECT CONCAT(format) FROM files WHERE files.book = books.id )                                             formats,
       path,
       uuid
FROM books;