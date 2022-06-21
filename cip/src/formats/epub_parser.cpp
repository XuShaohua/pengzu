// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/epub_parser.h"

#include <QDebug>

#include "formats/epub_reader.h"

bool ParseEpubFile(const QString& filepath) {
  EpubReader reader;
  if (reader.load(filepath)) {
    const auto pages = reader.numPages();
    qDebug() << "page num:" << pages;
    QString text;
    const bool ok = reader.readPage(1, text);
    qDebug() << "ok:" << ok;
    qDebug() << "text:\n" << text;
    return true;
  }

  return false;
}
