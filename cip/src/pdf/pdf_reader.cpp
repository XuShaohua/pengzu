// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "pdf/pdf_reader.h"

#include <QDebug>

PdfReader::PdfReader(QObject* parent) : QObject(parent) {

}

PdfReader::~PdfReader() {
  delete document_;
}

bool PdfReader::load(const QString& filepath) {
  document_ = Poppler::Document::load(filepath);
  if (document_ == nullptr || document_->isLocked() || document_->isEncrypted()) {
    delete document_;
    return false;
  }

  return true;
}

bool PdfReader::readPage(int number) {
  Poppler::Page* page = document_->page(number);
  if (page == nullptr) {
    qWarning() << "Failed to read page at: " << number;
    return false;
  }
  const auto size = page->pageSizeF();
  const QRectF rect{0.0, 0.0, size.width(), size.height()};
  const QString text = page->text(rect);
  qDebug() << "text of page:" << number << "is:\n" << text;

  delete page;
  return true;
}
