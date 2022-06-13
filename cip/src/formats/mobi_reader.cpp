// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#include "formats/mobi_reader.h"

#include <QDebug>

MobiReader::MobiReader(QObject* parent) : QObject(parent) {

}

MobiReader::~MobiReader() {
  if (mobi_ != nullptr) {
    mobi_free(mobi_);
    mobi_ = nullptr;
  }
}

bool MobiReader::load(const QString& filepath) {
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

  return true;
}

int MobiReader::numPages() const {
  if (mobi_ == nullptr) {
    return -1;
  }

  MOBIRawml* rawml = mobi_init_rawml(mobi_);
  const MOBI_RET ret = mobi_parse_rawml(rawml, mobi_);
  if (ret != MOBI_SUCCESS) {
    qWarning() << "Failed to parse rawml in mobi file, ret:" << ret;
    mobi_free_rawml(rawml);
    return -1;
  }

  size_t uid = -1;
  while (true) {
    uid += 1;
    MOBIPart* part = mobi_get_part_by_uid(rawml, uid);
    if (part == nullptr) {
      break;
    }
  }

  mobi_free_rawml(rawml);
  return static_cast<int>(uid);
}

bool MobiReader::readPage(int number, QString& text) {
  return false;
}
