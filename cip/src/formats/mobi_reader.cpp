// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/mobi_reader.h"

#include <QDebug>

MobiReader::MobiReader(QObject* parent) : QObject(parent) {

}

MobiReader::~MobiReader() {
  this->cleanup();
}

void MobiReader::cleanup() {
  if (rawml_ != nullptr) {
    mobi_free_rawml(rawml_);
    rawml_ = nullptr;
  }

  if (mobi_ != nullptr) {
    mobi_free(mobi_);
    mobi_ = nullptr;
  }
}

bool MobiReader::load(const QString& filepath) {
  this->cleanup();
  mobi_ = mobi_init();
  if (mobi_ == nullptr) {
    qWarning() << "Failed to init mobi instance";
    return false;
  }

  MOBI_RET ret = mobi_load_filename(mobi_, filepath.toStdString().c_str());
  if (ret != MOBI_SUCCESS) {
    qWarning() << "Failed to load mobi file:" << filepath << ", ret:" << ret;
    return false;
  }

  rawml_ = mobi_init_rawml(mobi_);
  if (rawml_ == nullptr) {
    qWarning() << "Failed to init rawml:" << filepath;
    return false;
  }

  return true;
}

int MobiReader::numPages() const {
  if (mobi_ == nullptr || rawml_ == nullptr) {
    return -1;
  }

  const MOBI_RET ret = mobi_parse_rawml(rawml_, mobi_);
  if (ret != MOBI_SUCCESS) {
    qWarning() << "Failed to parse rawml in mobi file, ret:" << ret;
    return -1;
  }

  size_t uid = -1;
  while (true) {
    uid += 1;
    MOBIPart* part = mobi_get_part_by_uid(rawml_, uid);
    if (part == nullptr) {
      break;
    }
  }

  return static_cast<int>(uid);
}

bool MobiReader::readPage(int number, QString& text) {
  if (mobi_ == nullptr || rawml_ == nullptr) {
    qWarning() << "mobi file not load yet!";
    return -1;
  }

  const MOBI_RET ret = mobi_parse_rawml(rawml_, mobi_);
  if (ret != MOBI_SUCCESS) {
    qWarning() << "Failed to parse rawml in mobi file, ret:" << ret;
    return -1;
  }

  int uid = -1;
  MOBIPart* part = nullptr;
  while (uid < number) {
    uid += 1;
    part = mobi_get_part_by_uid(rawml_, uid);
    if (part == nullptr) {
      break;
    }
  }

  if (part == nullptr) {
    qWarning() << "part is null";
    return false;
  }

  const char* data = reinterpret_cast<char*>(part->data);
  text = data;

  return true;
}
