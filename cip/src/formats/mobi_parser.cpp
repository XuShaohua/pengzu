// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/mobi_parser.h"

#include <QDebug>

#include "formats/cip_record.h"
#include "formats/mobi_reader.h"
#include "formats/util.h"

bool ParseMobiFile(const QString& filepath) {
  MobiReader reader;
  if (!reader.load(filepath)) {
    qWarning() << "Failed to open mobi file:" << filepath;
    return false;
  }

  const int pages = reader.numPages();
  if (pages < 0) {
    qWarning() << "Invalid page number in mobi file:" << filepath << pages;
    return false;
  }
  QString text;
  bool ok;

  // First 5 pages.
  int front_page = 0;
  for (front_page = 0; front_page < 5 && front_page < pages; ++front_page) {
    ok = reader.readPage(front_page, text);
    if (ok && IsPlainCipPage(text)) {
      return ParseMobiMetadata(filepath, text);
    }
  }

  // Last 5 pages.
  for (int back_page = qMax(pages - 5, front_page); back_page < pages; ++back_page) {
    ok = reader.readPage(back_page, text);
    if (ok && IsPlainCipPage(text)) {
      return ParseMobiMetadata(filepath, text);
    }
  }

  qWarning() << "No cip page found in:" << filepath;
  return false;
}

bool ParseMobiMetadata(const QString& filepath, const QString& html) {
  Q_UNUSED(filepath);
  QString text;
  if (!HtmlToText(html, text)) {
    qWarning() << "Html2Text() failed!" << filepath;
    return false;
  }
  qDebug() << qPrintable(text);

  CipRecord record;
  if (!ParseCipFromText(text, record)) {
    qWarning() << "Failed to parse cip record in:" << filepath;
    return false;
  }
  return true;
}
