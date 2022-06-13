// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/epub_reader.h"

#include <QDebug>

EpubReader::EpubReader(QObject* parent) : QObject(parent) {

}

EpubReader::~EpubReader() {
  // TODO(Shaohua):
  // epub_cleanup();

  this->cleanup();
}

void EpubReader::cleanup() {
  if (epub_ != nullptr) {
    epub_close(epub_);
    epub_ = nullptr;
  }
}

bool EpubReader::load(const QString& filepath) {
  this->cleanup();

  epub_ = epub_open(filepath.toStdString().c_str(), 0);
  if (epub_ == nullptr) {
    qWarning() << "Failed to open epub file:" << filepath;
    return false;
  }

//  epub_dump(epub_);

  return true;
}

int EpubReader::numPages() const {
  if (epub_ == nullptr) {
    qWarning() << "epub file not loaded";
    return -1;
  }

  eiterator* it = epub_get_iterator(epub_, EITERATOR_SPINE, 0);
  if (it == nullptr) {
    qWarning() << "Failed to get eit of epub file";
    return -1;
  }

  char* content = epub_it_get_curr(it);
  int pages = -1;
  while (content != nullptr) {
    content = epub_it_get_next(it);
    pages += 1;
  }
  epub_free_iterator(it);

  return pages;
}

bool EpubReader::readPage(int number, QString& text) {
  return false;
}
