// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/cip_record.h"

#include <QDebug>
#include <QRegularExpression>
#include <QTextStream>

bool IsPlainCipPage(const QString& text) {
  return text.contains("图书在版编目") && text.contains("中国版本图书馆");
}

bool ParseCipFromText(const QString& text, CipRecord& record) {
  QString text_copy = text;
  QTextStream stream(&text_copy);
  QString line;
  while (true) {
    line = stream.readLine();
    if (line.contains("图书在版编目")) {
      break;
    }
    if (stream.atEnd()) {
      qWarning() << "Invalid cip section";
      return false;
    }
  }

  // title
  line = stream.readLine();
  int index;
  while (true) {
    index = line.indexOf('/');
    if (index == -1) {
      index = line.indexOf("／");
    }
    if (index > -1) {
      break;
    }
    if (stream.atEnd()) {
      qWarning() << "Failed to read cip title";
      return false;
    }
    line = stream.readLine();
  }
  record.title = line.left(index).trimmed();
  qDebug() << "title:" << record.title;

  // publisher and pubdate
  // go next line.
  if (!line.contains("-") && !line.contains("—")) {
    line = stream.readLine();
  }
  index = line.lastIndexOf("：");
  if (index == -1) {
    index = line.lastIndexOf(":");
  }
  int index_date = line.lastIndexOf("，");
  if (index_date == -1) {
    index_date = line.lastIndexOf(", ");
  }
  record.publisher = line.mid(index + 1, index_date - index - 1).trimmed();
  qDebug() << "publisher:" << record.publisher;
  record.pubdate = line.right(line.length() - index_date - 1).trimmed();
  qDebug() << "pubdate:" << record.pubdate;
  line = stream.readLine();

  // original title
  if (line.contains("书名原文：")) {
    record.original_title = line.split("：").last().trimmed();
    qDebug() << "original title:" << record.original_title;
    line = stream.readLine();
  }

  // ISBN
  while (!line.contains("ISBN")) {
    if (stream.atEnd()) {
      qWarning() << "Invalid isbn";
      return false;
    }
    line = stream.readLine();
  }
  record.isbn = line.remove("ISBN").replace("-", "").replace("–", "").replace(" ", "").trimmed();
  qDebug() << "isbn:" << record.isbn;

  // category id
  if (line.contains("iii") || line.contains("①") ||
      line.contains("Ⅲ") || line.contains("iV")) {
    QRegularExpression pattern(".*([A-Z][0-9. \\-]+)\\s*$");
    auto match = pattern.match(line);
    if (match.isValid()) {
      record.category_id = match.captured(1).trimmed().replace(" ", "");
      qDebug() << "category id:" << record.category_id;
    }
    line = stream.readLine();
  }

  // cip id
  while (!line.contains("中国版本图书馆")) {
    if (stream.atEnd()) {
      qWarning() << "Invalid cip id!";
      return false;
    }
    line = stream.readLine();
  }
  index = line.lastIndexOf("第");
  int end_index = line.lastIndexOf("号");
  if (index != -1 && end_index != -1) {
    record.cip_id = line.mid(index + 1, end_index - index - 1).trimmed();
    qDebug() << "cip id:" << record.cip_id;
    line = stream.readLine();
  }

  // author
  while (true) {
    QString no_space_line = line.trimmed().replace(" ", "").replace("　", "");
    if (no_space_line.endsWith("著") || no_space_line.startsWith("作者")
        || no_space_line.startsWith("著作") || no_space_line.startsWith("著者")) {
      break;
    }
    if (stream.atEnd()) {
      qWarning() << "Invalid author in cip";
      return false;
    }
    line = stream.readLine();
  }
  index = line.indexOf("：");
  if (index == -1) {
    index = line.indexOf(":");
  }
  if (index == -1) {
    index = line.indexOf("/");
  }
  end_index = line.lastIndexOf("著");
  if (end_index == -1) {
    end_index = line.replace(" ", "").indexOf("作者");
  }
  QString author;
  if (end_index > index) {
    author = line.mid(index + 1, end_index - index - 1).trimmed();
  } else {
    author = line.mid(index + 1).trimmed();
  }
  {
    QString name;
    bool is_country = false;
    bool got_author = false;
    for (const QChar& chr: author) {
      if (chr == '[' || chr == "［" || chr == "〔") {
        is_country = true;
        name.append('[');
        continue;
      }

      if (chr == ']' || chr == "］" || chr == "〕") {
        name.append(']');
        is_country = false;
        continue;
      }
      if (chr.isSpace()) {
        if (is_country) {
          // Ignore space in country code.
        } else if (got_author) {
          record.authors.append(name);
          name.clear();
          got_author = false;
          is_country = false;
        }
        continue;
      }
      name.append(chr);
      if (!is_country) {
        got_author = true;
      }
    }

    if (!name.isEmpty()) {
      record.authors.append(name);
    }
  }
  qDebug() << "authors:" << record.authors;

  // price
  while (true) {
    if (stream.atEnd()) {
      return true;
    }
    line = stream.readLine();
    if (line.contains("元")) {
      break;
    }
  }
  QRegularExpression price_pattern("([0-9. ]+\\s*元)");
  auto price_match = price_pattern.match(line);
  if (price_pattern.isValid()) {
    record.price = price_match.captured(1).trimmed();
    qDebug() << "price: " << record.price;
  }

  return true;
}
