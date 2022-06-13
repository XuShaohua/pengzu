// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CIP_CIP_SRC_MOBI_MOBI_READER_H_
#define CIP_CIP_SRC_MOBI_MOBI_READER_H_

#include <QObject>

class MobiReader : public QObject {
  Q_OBJECT
 public:
  explicit MobiReader(QObject* parent = nullptr);

};

#endif  // CIP_CIP_SRC_MOBI_MOBI_READER_H_
