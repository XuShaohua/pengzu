// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CIP_CIP_SRC_FORMATS_EPUB_READER_H_
#define CIP_CIP_SRC_FORMATS_EPUB_READER_H_

#include <QObject>

#include "third_party/libepub/epub.h"

class EpubReader : public QObject {
 Q_OBJECT
 public:
  explicit EpubReader(QObject* parent = nullptr);

  ~EpubReader() override;

  bool load(const QString& filepath);

  int numPages() const;

  bool readPage(int number, QString& text);

 private:
  void cleanup();

  epub* epub_{nullptr};
};

#endif  // CIP_CIP_SRC_FORMATS_EPUB_READER_H_
