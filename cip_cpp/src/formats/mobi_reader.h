// Copyright (c) 2022 Xu Shaohua <shaohua@biofan.org>. All rights reserved.
// Use of this source is governed by GNU General Public License
// that can be found in the LICENSE file.

#ifndef CIP_CIP_SRC_MOBI_MOBI_READER_H_
#define CIP_CIP_SRC_MOBI_MOBI_READER_H_

#include <QObject>

#include <mobi.h>

class MobiReader : public QObject {
  Q_OBJECT
 public:
  explicit MobiReader(QObject* parent = nullptr);
  ~MobiReader() override;

  bool load(const QString& filepath);

  int numPages() const;

  bool readPage(int number, QString& text);

 private:
  void cleanup();

  MOBIData* mobi_{nullptr};
  MOBIRawml* rawml_{nullptr};
};

#endif  // CIP_CIP_SRC_MOBI_MOBI_READER_H_
