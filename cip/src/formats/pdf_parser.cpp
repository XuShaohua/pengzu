// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/pdf_parser.h"

#include <QDebug>

#include "formats/pdf_reader.h"
#include "formats/util.h"

bool ParsePdfFile(const QString& filepath) {
  PdfReader reader;
  if (!reader.load(filepath)) {
    qWarning() << "Failed to open pdf file:" << filepath;
    return false;
  }
  const int pages = reader.numPages();
  QString text;
  bool ok;

  // First 10 pages.
  int front_page = 0;
  for (front_page = 0; front_page < 10 && front_page < pages; ++front_page) {
    ok = reader.readPage(front_page, text);
    if (ok && IsPlainCipPage(text)) {
      return ParsePdfMetadata(filepath, text);
    }
  }

  // Last 5 pages.
  for (int back_page = qMax(pages - 5, front_page); back_page < pages; ++back_page) {
    ok = reader.readPage(back_page, text);
    if (ok && IsPlainCipPage(text)) {
      return ParsePdfMetadata(filepath, text);
    }
  }

  qWarning() << "No cip page found in:" << filepath;
  return false;
}

bool ParsePdfMetadata(const QString& filepath, const QString& text) {
  Q_UNUSED(filepath);
  Q_UNUSED(text);
//  qDebug() << qPrintable(text);
  return true;
}