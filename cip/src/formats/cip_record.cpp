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
  index = line.lastIndexOf("：");
  if (index == -1) {
    index = line.lastIndexOf(":");
  }
  if (index != -1) {
    // publisher and pubdate
    int index_date = line.lastIndexOf("，");
    if (index_date == -1) {
      index_date = line.lastIndexOf(", ");
    }
    record.publisher = line.mid(index + 1, index_date - index - 1).trimmed();
    qDebug() << "publisher:" << record.publisher;
    record.pubdate = line.right(line.length() - index_date - 1).trimmed();
    qDebug() << "pubdate:" << record.pubdate;
    line = stream.readLine();
  } else {
    qWarning() << "Invalid cip title:" << line;
    return false;
  }

  // original title
  if (line.contains("书名原文：")) {
    record.original_title = line.split("：").last().trimmed();
    qDebug() << "original title:" << record.original_title;
    line = stream.readLine();
  }

  // ISBN
  if (line.contains("ISBN")) {
    record.isbn = line.remove("ISBN").replace("-", "").replace(" ", "").trimmed();
    qDebug() << "isbn:" << record.isbn;
    line = stream.readLine();
  }

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
  while (!line.contains("著")) {
    if (stream.atEnd()) {
      return true;
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
    end_index = line.indexOf("作");
  }
  if (end_index > index) {
    const QString author = line.mid(index + 1, end_index - index - 1).trimmed();
    record.authors.append(author);
  } else {
    const QString author = line.mid(index + 1).trimmed();
    record.authors.append(author);
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
  QRegularExpression price_pattern("([0-9.]+\\s*元)");
  auto price_match = price_pattern.match(line);
  if (price_pattern.isValid()) {
    record.price = price_match.captured(1).trimmed();
    qDebug() << "price: " << record.price;
  }

  return true;
}
