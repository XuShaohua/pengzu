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

  // First 5 pages.
  for (int page = 0; page < 5 && page < pages; ++page) {
    ok = reader.readPage(page, text);
    if (ok && IsPlainCipPage(text)) {
      return ParsePdfMetadata(filepath, text);
    }
  }

  // Last 5 pages.
  for (int page = pages - 5; page < pages; ++page) {
    ok = reader.readPage(page, text);
    if (ok && IsPlainCipPage(text)) {
      return ParsePdfMetadata(filepath, text);
    }
  }

  return false;
}

bool ParsePdfMetadata(const QString& filepath, const QString& text) {
  qDebug() << qPrintable(text);
  return true;
}