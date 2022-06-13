// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CIP_CIP_SRC_PDF_PDF_READER_H_
#define CIP_CIP_SRC_PDF_PDF_READER_H_

#include <QObject>

class PdfReader : public QObject {
  Q_OBJECT
 public:
  explicit PdfReader(QObject* parent = nullptr);

  bool load(const QString& filepath);
};

#endif  // CIP_CIP_SRC_PDF_PDF_READER_H_
