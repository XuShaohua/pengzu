// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/pdf_reader.h"

#include <QDebug>

PdfReader::PdfReader(QObject* parent) : QObject(parent) {

}

PdfReader::~PdfReader() {
  delete document_;
}

bool PdfReader::load(const QString& filepath) {
  if (document_ != nullptr) {
    delete document_;
  }
  document_ = Poppler::Document::load(filepath);
  if (document_ == nullptr || document_->isLocked() || document_->isEncrypted()) {
    delete document_;
    return false;
  }

  return true;
}

int PdfReader::numPages() const {
  if (document_ == nullptr) {
    return -1;
  }
  return document_->numPages();
}

bool PdfReader::readPage(int number, QString& text) {
  Poppler::Page* page = document_->page(number);
  if (page == nullptr) {
    qWarning() << "Failed to read page at: " << number;
    return false;
  }
  const auto size = page->pageSizeF();
  const QRectF rect{0.0, 0.0, size.width(), size.height()};
  text = page->text(rect);
  delete page;
  return true;
}
