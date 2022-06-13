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
  if (it_ != nullptr) {
    epub_free_iterator(it_);
    it_ = nullptr;
  }

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

bool EpubReader::initIterator() {
  if (epub_ == nullptr) {
    qWarning() << "epub file not loaded";
    return false;
  }

  if (it_ != nullptr) {
    epub_free_iterator(it_);
  }
  it_ = epub_get_iterator(epub_, EITERATOR_SPINE, 0);
  if (it_ == nullptr) {
    qWarning() << "Failed to get eit of epub file";
    return false;
  }

  return true;
}

int EpubReader::numPages() {
  if (!this->initIterator()) {
    return -1;
  }

  char* content = epub_it_get_curr(it_);
  int pages = -1;
  while (content != nullptr) {
    content = epub_it_get_next(it_);
    pages += 1;
  }

  return pages;
}

bool EpubReader::readPage(int number, QString& text) {
  if (!this->initIterator()) {
    return false;
  }

  char* content = nullptr;
  int current = -1;
  while (current < number){
    content = epub_it_get_next(it_);
    if (content == nullptr) {
      break;
    }
    current += 1;
  }
  if (content == nullptr) {
    return false;
  }
  text = content;
  return true;
}
